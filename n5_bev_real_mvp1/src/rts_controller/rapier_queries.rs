use bevy::prelude::*;
use bevy::ecs::system::SystemParam;
use bevy_rapier2d::prelude::*;

use crate::rapier_config::prelude::{
    E_ATTACKABLE_FILTER,
    P_SELECTABLE_FILTER,
};

#[derive(SystemParam)]
pub struct RtsControllerRapierQueries<'w, 's>{
    rapier: Res<'w, RapierContext>,
}

/// Methods
impl<'w, 's> RtsControllerRapierQueries<'w, 's> {
    /// e_attackable = an attackable enemy
    pub fn cast_for_e_attackable(
        &self,
        location: Vec2,
    ) -> Option<(Entity, Toi)> {
        const SHAPE: Collider = Collider::ball(10.0);

        let rapier = self.rapier;
        return rapier.cast_shape(
            location, 
            0.0, 
            Vec2::ZERO, 
            &SHAPE, 
            0.0,
            E_ATTACKABLE_FILTER,
        )
    }

    /// p_selectable = a selectable unit on the player team
    pub fn cast_for_p_selectable(
        &self,
        origin: Vec2,
        release: Vec2,
        callback: impl FnMut(Entity) -> bool, // called for each detected collider
    ) {
        let half_extents = (origin - release).abs(); // Dimensions
        let half_extents = half_extents * 0.5; // Half extents
        let shape = Collider::cuboid(half_extents.x, half_extents.y);
        
        let location = (origin + release) * 0.5;

        let rapier = self.rapier;
        return rapier.intersections_with_shape( // Only detects shapes that share its shape
            location, 
            0.0, 
            &shape, 
            P_SELECTABLE_FILTER, 
            callback,
        )
    }
}