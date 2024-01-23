use bevy::prelude::*;
use bevy::ecs::system::SystemParam;
use bevy_rapier2d::prelude::*;
use bevy::render::primitives::Aabb;

use mouse_tracking::MousePosWorld;

use crate::unit::selectable::Selectable;
use crate::unit::*;

use super::unit_selection::input::SelectInput;

pub struct InitializePlugin;
impl Plugin for InitializePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SelectionBoxOrigin>();
        app.add_systems(Update, mouse_down_store_origin);
    }
}

fn mouse_down_store_origin(
    input: SelectInput,
    mouse_world: Res<MousePosWorld>,
    mut origin: ResMut<SelectionBoxOrigin>,
) {
    if !input.just_pressed() {
        return;
    }
    origin.0 = **mouse_world;
}

#[derive(Resource, Default)]
struct SelectionBoxOrigin(Vec3);

#[derive(SystemParam)]
pub struct UnitMouse<'w, 's> {
    mouse_origin: Res<'w, SelectionBoxOrigin>,
    mouse_world: Res<'w, MousePosWorld>,
    rapier: Res<'w, RapierContext>,
    transform_q: Query<'w, 's, &'static Transform>,
    selectable_q: Query<'w, 's, &'static mut Selectable>,
    enemy_q: Query<'w, 's, &'static mut Unit>, // Replace with enemy 
}
impl<'w, 's> UnitMouse<'w, 's> {
    pub fn mouse_location(&self) -> Vec2 {
        return self.mouse_origin.0.truncate();
    }

    /// Returns a enemy unit, out of the units in a small detected area around the mouse, picking the one closest to the mouse
    /// Uses translation for deciding which unit is closer
    pub fn enemy_cast (
        &self,
    ) -> Option<UnitID> {
        let mut results: Vec<Entity> = Vec::new();
        let callback = |entity| -> bool {
            let enemy = self.enemy_q.get(entity);
            if enemy.is_err() {
                return false;
            }
            results.push(entity);
            return true;
        };
        let location = self.mouse_world.truncate();
        self.single_cast(location, callback);

        if results.is_empty() {
            return None;
        }

        let mut lowest_distance = f32::MAX;
        let mut nearest_entity = Entity::PLACEHOLDER;
        for entity in results.iter() {
            let transform = self.transform_q.get(entity.clone());
            let transform = transform.unwrap();
            let entity_position = transform.translation.truncate();
            let distance = location.distance(entity_position);
            if distance < lowest_distance {
                lowest_distance = distance;
                nearest_entity = *entity;
            }
        }

        return Some(UnitID(nearest_entity));
    }

    pub fn selection_drag_click_release(
        &self,
    ) -> Vec<UnitID> {
        let mut return_vec: Vec<UnitID> = Vec::new();
        let callback = |entity| -> bool {
            let selectable = self.selectable_q.get(entity);
            if selectable.is_err() {
                return false;
            }
            return_vec.push(UnitID(entity));
            return true;
        };
        let location1 = self.mouse_origin.0.truncate();
        let location2 = self.mouse_world.truncate();
        self.box_intersect(location1, location2, callback);

        return return_vec;
    }

    fn single_cast(
        &self,
        location: Vec2,
        callback: impl FnMut(Entity) -> bool,
    ) {
        const SINGLE_CAST_SIZE: f32 = 5.0;
        
        let minimum = (location + (Vec2::ONE * SINGLE_CAST_SIZE)).extend(0.0);
        let maximum = (location + (Vec2::NEG_ONE * SINGLE_CAST_SIZE)).extend(0.0);
        let aabb = Aabb::from_min_max(minimum, maximum);
        let rapier = & self.rapier;
        rapier.colliders_with_aabb_intersecting_aabb(
            aabb,
            callback,
        );
    }
    
    fn box_intersect(
        &self,
        location1: Vec2,
        location2: Vec2,
        callback: impl FnMut(Entity) -> bool,
    ) {
        let aabb = vec2s_to_aabb(location1, location2);
        let rapier = & self.rapier;
        rapier.colliders_with_aabb_intersecting_aabb(
            aabb,
            callback,
        );
    }
}

fn vec2s_to_aabb(vec1: Vec2, vec2: Vec2) -> Aabb {
    let mut max: Vec2 = Vec2::ZERO;
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
    } 
    else{
        min.x = vec2.x;
    }
    // min y
    if vec1.y < vec2.y{
        min.y = vec1.y;
    }
    else{
        min.y = vec2.y;
    }
    
    return Aabb::from_min_max(
        min.extend(0.0),
        max.extend(0.0),
    );
}