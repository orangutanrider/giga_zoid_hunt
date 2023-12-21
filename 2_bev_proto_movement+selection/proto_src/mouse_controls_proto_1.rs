// This file exists just as an archive of my workings, for when I was creating the mouse_controls script

use bevy::{prelude::*, render::primitives::Aabb};
use bevy_rapier2d::prelude::*;
use mouse_tracking::MousePosWorld;

#[derive(Component)]
pub struct BoxSelectionEmpty{
    pub origin: Vec2,
}

pub struct InitializePlugin;
impl Plugin for InitializePlugin {
    fn build(&self, app: &mut App) {
        println!("");
        println!("Initializing mouse_controls.rs");
        app
            .add_systems(Startup, startup)
            .add_systems(Update, (
                update,
                selection_single_click,
            )
        );
    }
}

fn startup(mut commands: Commands){
    commands.spawn(BoxSelectionEmpty{origin: Vec2::ZERO});
}

fn update(
    rapier_context: Res<RapierContext>,
    mouse_world: Res<MousePosWorld>,
    buttons: Res<Input<MouseButton>>,
    mut box_empty_q: Query<&mut BoxSelectionEmpty>,
){
    if buttons.just_released(MouseButton::Left) {
        /* 
        if let entity = intersecting_cuboid_test_input(rapier_context, box_empty_q.single_mut().origin, mouse_world.truncate()){
            println!("HIT");
        }
        */

        /*
        if let entity = intersecting_aabb_test(rapier_context) {
            println!("HIT");
        }
        */

        /* 
        if let Some((entity, toi)) = cast_box_drag_click_single(rapier_context,  box_empty_q.single_mut().origin, mouse_world.truncate()) {
            println!("HIT");
        }
        */

        /* 
        let mut hits = 0;
        cast_box_drag_click(rapier_context, box_empty_q.single_mut().origin, mouse_world.truncate(), |entity|{
            hits += 1;
            println!("E");
            true
        });
        println!("Hits: {}", hits);
        */

        return;
    }

    if buttons.just_pressed(MouseButton::Left) {
        box_empty_q.single_mut().origin = mouse_world.truncate();
        return;
    }
}

fn cast_box_drag_click(    
    rapier_context: Res<RapierContext>,
    origin: Vec2,
    release: Vec2,
    callback: impl FnMut(Entity) -> bool, // Callback called on each collider hit by the ray
) {
    let (box_origin, box_shape) = box_from_two_points(origin, release);
    rapier_context.intersections_with_shape
    (box_origin, 0.0, &box_shape, QueryFilter::default(), callback);
}

fn intersecting_aabb_test(
    rapier_context: Res<RapierContext>,
){
    rapier_context.colliders_with_aabb_intersecting_aabb(
        aabb_from_points(Vec2::new(-10000.0, -10000.0), Vec2::new(10000.0, 10000.0)), 
        |entity|{
            println!("The entity {:?} has an AABB intersecting our test AABB", entity);
            true
        }
    );
}

fn intersecting_cuboid_test(
    rapier_context: Res<RapierContext>,
) {
    let (box_origin, box_shape) = box_from_two_points(Vec2::new(-10000.0, -10000.0), Vec2::new(10000.0, 10000.0));
    rapier_context.intersections_with_shape
    (box_origin, 0.0, &box_shape, QueryFilter::default(), |entity|{
        println!("E");
        true
    });

    // The results of this is that in the implementation:
    /*
        if let entity = intersecting_cuboid_test(rapier_context){
            println!("HIT");
        }
    */
    // It prints HIT
    // However it doesn't print any of the messsages from it's own callback
    // So that indicates that it still isn't working to me
    // Hmm, I should test this further by implementing input with it
    // That way I can see if it is just printing Hit every time or if it really is detecting something and printing because of that
}

fn intersecting_cuboid_test_input(
    rapier_context: Res<RapierContext>,
    origin: Vec2,
    release: Vec2,
) {
    let (box_origin, box_shape) = box_from_two_points(origin, release);
    rapier_context.intersections_with_shape
    (box_origin, 0.0, &box_shape, QueryFilter::default(), |entity|{
        println!("E");
        true
    });

    /*
        if let entity = intersecting_cuboid_test_input(rapier_context, box_empty_q.single_mut().origin, mouse_world.truncate()){
            println!("HIT");
        }
    */
    // Test results are that it really is just returning true every time, no matter what

    // Hmm...
    // This is very strange. Why would the single click work if this doesnt?
    // It must be that I'm like not iterating over the callback or something, right?
    // But then why does the AABB one work right out of the box? Hmm?

    // Hold up, could it be that it's because I am detecting ball colliders with a ball?
    // And that I can't detect them with a cuboid?
    // HMM?! There must be a way around that though.

    // And why wouldn't that be more well documented? The only thing that leads me to think that is this one line in the tooltip:
    // "shape - The shape to test"
    // The rest of it leads me to think otherwise
    // I mean that sounds ridiculous doesn't it? Why the hell would it work like that?

    // Hmm, okay I think I can test this though
    // If I create a ball intersection, capable of handling multiple results, then under this theory it should print it's results

    // WAIT
    // THIS: rapier_context.intersections_with()
    // It's this right? I'm just using the wrong method

    // Wait no, that methods required an entity which implies that it's supposed to be used on real colliders, not fake ones used for shape casts
    // hmm...
    // I will try the other shape thing

    // Okay yeah in the documentation:
    // "Intersection tests will find all the colliders with a shape intersecting a given shape."
    // That is confirmation right there, so I'm not gonna do that test, I've already spent enough time on this
    // I will just use the bounding box method
}

fn aabb_from_points(point_a: Vec2, point_b: Vec2) -> Aabb{
    Aabb::from_min_max(
        Vec3::new(point_a.x, point_a.y, 0.0), 
        Vec3::new(point_b.x, point_b.y, 0.0))
}

fn cast_box_drag_click_single(    
    rapier_context: Res<RapierContext>,
    origin: Vec2,
    release: Vec2,
) -> Option<(Entity, Toi)> {
    let (box_origin, box_shape) = box_from_two_points(origin, release);
    rapier_context.cast_shape
    (box_origin, 0.0, Vec2::ZERO, &box_shape, 0.0, QueryFilter::default())
}

fn box_from_two_points(point_a: Vec2, point_b: Vec2) -> (Vec2, Collider) {
    let half_x =  (point_a.x - point_b.x) / 2.0;
    let half_y = (point_a.y - point_b.y) / 2.0;
    let origin = point_a + Vec2{x: half_x, y: half_y};
    (origin, Collider::cuboid(half_x, half_y))
}

// once box selection is added, there needs to be a thing for deciding whether to do this or box selection.
// in the prototype I did that by storing the position on mouse down and then comparing it to the position on mouse up
// if the distance is too low, I'd do a single click selection
fn selection_single_click(
    rapier_context: Res<RapierContext>,
    mouse_world: Res<MousePosWorld>,
    buttons: Res<Input<MouseButton>>,
){
    if !buttons.just_pressed(MouseButton::Left){
        return;
    }

    if let Some((entity, toi)) = cast_single_click(rapier_context, mouse_world.truncate()) {
        println!("HIT"); // this doesn't actually do any selection yet
    }
}

fn cast_single_click(
    rapier_context: Res<RapierContext>,
    cast_position: Vec2,
) -> Option<(Entity, Toi)> {
    rapier_context.cast_shape
    (cast_position, 0.0, Vec2::ZERO, &Collider::ball(5.0), 0.0, QueryFilter{..default()})
}