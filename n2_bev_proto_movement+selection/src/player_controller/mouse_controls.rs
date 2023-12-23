use bevy::{prelude::*, render::primitives::Aabb};
use bevy_rapier2d::prelude::*;
use super::selection_controller::*;
use mouse_tracking::MousePosWorld;

use crate::unit_system::Unit;

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
                //selection_single_click,
            )
        );
    }
}

const UNIT_FILTER: QueryFilter = QueryFilter{
    flags: QueryFilterFlags::ONLY_KINEMATIC, 
    groups: None, 
    exclude_collider: None, 
    exclude_rigid_body: None, 
    predicate: None,
};

fn startup(mut commands: Commands){
    commands.spawn(BoxSelectionEmpty{origin: Vec2::ZERO});
}

fn update(
    rapier_context: Res<RapierContext>,
    mouse_world: Res<MousePosWorld>,
    buttons: Res<Input<MouseButton>>,
    mut box_empty_q: Query<&mut BoxSelectionEmpty>,
    unit_q: Query<&mut Unit>,
    mut manager_q: Query<&mut UnitSelectionManager>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        box_empty_q.single_mut().origin = mouse_world.truncate();
        return;
    }

    if buttons.just_released(MouseButton::Left) {

        let (min, max) = get_min_max(box_empty_q.single().origin, mouse_world.truncate());
        aabb_intersections(rapier_context, min, max, |entity|{
            let unit = get_unit_from_entity(&unit_q, entity); // try get unit
            if unit.is_none(){ // unit was not gotten
                return false;
            }
            select_unit(&mut manager_q.single_mut(), &unit.unwrap()); // unit has been gotten, and is being selected
            return true;
        });

        return;
    }
}

fn aabb_intersections(
    rapier_context: Res<RapierContext>,
    aabb_min: Vec2,
    aabb_max: Vec2,
    callback: impl FnMut(Entity) -> bool, // Callback called for each intersecting aabb collider
) {
    rapier_context.colliders_with_aabb_intersecting_aabb(
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

// once box selection is added, there needs to be a thing for deciding whether to do this or box selection.
// in the prototype I did that by storing the position on mouse down and then comparing it to the position on mouse up
// if the distance is too low, I'd do a single click selection
fn selection_single_click(
    q: Query<&mut Unit>,
    rapier_context: Res<RapierContext>,
    mouse_world: Res<MousePosWorld>,
    buttons: Res<Input<MouseButton>>,
) {
    if !buttons.just_pressed(MouseButton::Left){
        return;
    }

    if let Some(unit) = cast_single_click(&q, rapier_context, mouse_world.truncate()) {
        // Select Unit
    }
}


fn cast_single_click<'a>(
    q: &'a Query<&mut Unit>,
    rapier_context: Res<RapierContext>,
    cast_position: Vec2,
) -> Option<&'a Unit> {
    let cast_results = rapier_context.cast_shape
    (cast_position, 0.0, Vec2::ZERO, &Collider::ball(5.0), 0.0, UNIT_FILTER);

    if cast_results == None{
        None
    }
    else{
        let (entity, toi) = cast_results.unwrap();
        get_unit_from_entity(&q, entity)
    }
}

fn get_unit_from_entity<'a>(
    q: &'a Query<&mut Unit>, 
    entity: Entity
) -> Option<&'a Unit> {
    if let Ok(unit) = q.get(entity){
        Some(unit)
    }
    else{
        None
    }
}

fn try_select_entity_as_unit(
    entity: Entity, 
    unit_q: Query<&mut Unit>,
    mut manager_q: Query<&mut UnitSelectionManager>,
) -> bool {
    let unit = get_unit_from_entity(&unit_q, entity); // try get unit
    if unit.is_none(){ // unit was not gotten
        return false;
    }
    select_unit(&mut manager_q.single_mut(), &unit.unwrap()); // unit has been gotten, and is being selected
    return true;
}