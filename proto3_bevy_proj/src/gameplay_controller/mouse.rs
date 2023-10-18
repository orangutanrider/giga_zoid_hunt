use bevy::prelude::*;
use bevy::render::primitives::Aabb;
use bevy_rapier2d::prelude::*;
use mouse_tracking::MousePosWorld;

use crate::unit::*;
use super::selection::*;
use super::unit_orders::*;

#[derive(Component)]
pub struct SelectionBox{
    pub origin: Vec2,
}

pub struct InitializePlugin;
impl Plugin for InitializePlugin {
    fn build(&self, app: &mut App) {
        println!("");
        println!("Initializing mouse.rs");
        app
            .add_systems(Startup, spawn_selection_box)
            .add_systems(Update, (
                drag_selection,
                click_selection,
                right_click_update,
            )
        );
    }
}

const DRAG_SELECTION_MIN_DISTANCE: f32 = 2.0;

const UNIT_FILTER: QueryFilter = QueryFilter{
    flags: QueryFilterFlags::ONLY_KINEMATIC, 
    groups: None, 
    exclude_collider: None, 
    exclude_rigid_body: None, 
    predicate: None,
};

// Startup
fn spawn_selection_box(mut commands: Commands){
    commands.spawn(SelectionBox{origin: Vec2::ZERO});
}

// Update
fn drag_selection(
    rapier: Res<RapierContext>,
    mouse_world: Res<MousePosWorld>,
    buttons: Res<Input<MouseButton>>,

    mut box_q: Query<&mut SelectionBox>,
    unit_q: Query<&mut UnitEntity, With<Selectable>>,

    mut manager_q: Query<&mut NewSelectionManager>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        box_q.single_mut().origin = mouse_world.truncate();
        return;
    }

    if buttons.just_released(MouseButton::Left) {
        let (min, max) = get_min_max(box_q.single().origin, mouse_world.truncate());

        if min.distance(max) < DRAG_SELECTION_MIN_DISTANCE {
            return;
        }

        let mut manager = manager_q.single_mut();

        selection_input(&mut manager);

        aabb_intersections(rapier, min, max, |entity|{
            let unit = get_unit_from_entity(&unit_q, entity); // Try get unit

            if unit.is_none(){ // Unit was not gotten
                return false;
            }
            
            // Select Unit
            let unit = unit.unwrap();
            select(&mut manager, unit);

            return true;
        });

        return;
    }
}

fn click_selection(
    unit_q: Query<&mut UnitEntity, With<Selectable>>,
    mut manager_q: Query<&mut NewSelectionManager>,
    rapier: Res<RapierContext>,
    mouse_world: Res<MousePosWorld>,
    buttons: Res<Input<MouseButton>>,
) {
    if !buttons.just_pressed(MouseButton::Left){
        return;
    }

    let mut manager = manager_q.single_mut();
    
    selection_input(&mut manager);

    if let Some(unit_entity) = cast_single_click(&unit_q, rapier, mouse_world.truncate()) {
        select(&mut manager, unit_entity);
    }
}


fn right_click_update(
    mouse_world: Res<MousePosWorld>,
    buttons: Res<Input<MouseButton>>,

    mut manager_q: Query<&mut NewOrderManager>
) {
    if !buttons.just_pressed(MouseButton::Right){
        return;
    }

    let mut manager = manager_q.single_mut();
    let order = Order{
        move_to_point: mouse_world.truncate(),
    };

    give_movement_order(manager.as_mut(), order)
}

// Internal
fn aabb_intersections(
    rapier: Res<RapierContext>,
    aabb_min: Vec2,
    aabb_max: Vec2,
    callback: impl FnMut(Entity) -> bool, // Callback called for each intersecting aabb collider
) {
    rapier.colliders_with_aabb_intersecting_aabb(
        Aabb::from_min_max(Vec3::new(aabb_min.x, aabb_min.y, 0.0), Vec3::new(aabb_max.x, aabb_max.y, 0.0)), 
        callback
    );
}

fn get_min_max(vec1: Vec2, vec2: Vec2) -> (Vec2, Vec2) {
    // There is probably a math way to do this, dunno what it is though

    let mut max = Vec2::ZERO;
    // max X
    if vec1.x > vec2.x{
        max.x = vec1.x;
    } else{
        max.x = vec2.x;
    }
    // max y
    if vec1.y > vec2.y{
        max.y = vec1.y;
    }
    else{
        max.y = vec2.y;
    }

    let mut min = Vec2::ZERO;
    // min X
    if vec1.x < vec2.x{
        min.x = vec1.x;
    } else{
        min.x = vec2.x;
    }
    // min y
    if vec1.y < vec2.y{
        min.y = vec1.y;
    }
    else{
        min.y = vec2.y;
    }
    
    (min, max)
}

fn cast_single_click<'a>(
    q: &'a Query<&mut UnitEntity, With<Selectable>>,
    rapier: Res<RapierContext>,

    cast_position: Vec2,
) -> Option<&'a UnitEntity> {
    let cast_results = rapier.cast_shape
    (cast_position, 0.0, Vec2::ZERO, &Collider::ball(5.0), 0.0, UNIT_FILTER);

    if cast_results == None{
        return None;
    }
    else{
        let (entity, toi) = cast_results.unwrap();
        return get_unit_from_entity(&q, entity);
    }
}

fn get_unit_from_entity<'a>(
    q: &'a Query<&mut UnitEntity, With<Selectable>>, 
    entity: Entity
) -> Option<&'a UnitEntity> {
    if let Ok(unit) = q.get(entity){
        return Some(unit);
    }
    else{
        return None;
    }
}