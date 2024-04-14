use bevy::prelude::*;
use bevy::ecs::system::SystemParam;
use bevy_rapier2d::prelude::*;

use rapier_config::*;

#[derive(SystemParam)]
pub struct PhysicsQueries<'w>(
    Res<'w, RapierContext>
);

/// Methods
impl<'w> PhysicsQueries<'w> {
    /// e_attackable = an attackable enemy
    pub fn cast_for_e_attackable(
        &self,
        location: Vec2,
    ) -> Option<(Entity, Toi)> {
        const RADIUS: f32 = 10.0;
        let shape = Collider::ball(RADIUS);

        let rapier = &self.0;
        return rapier.cast_shape(
            location, 
            0.0, 
            Vec2::ZERO, 
            &shape, 
            0.0,
            true,
            DETECTABLE_ENEMY_UNITS_FILTER,
        )
    }

    pub fn cast_for_selectable(
        &self,
        origin: Vec2,
        release: Vec2,
        callback: impl FnMut(Entity) -> bool, // called for each detected collider
    ) {
        let half_extents = (origin - release).abs(); // Dimensions
        let half_extents = half_extents * 0.5; // Half extents
        let shape = Collider::cuboid(half_extents.x, half_extents.y);
        
        let location = (origin + release) * 0.5;

        let rapier = &self.0;
        return rapier.intersections_with_shape( // Only detects shapes that share its shape
            location, 
            0.0, 
            &shape, 
            SELECTABLE_FILTER, 
            callback,
        )
    }
}