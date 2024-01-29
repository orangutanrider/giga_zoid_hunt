use bevy::prelude::*;
use bevy::ecs::system::SystemParam;
use bevy_rapier2d::prelude::*;

use crate::rapier_config::prelude::E_ATTACKABLE_FILTER;

#[derive(SystemParam)]
pub struct RtsCommanderRapierQueries<'w, 's>{
    rapier: Res<'w, RapierContext>,
}

/// Methods
impl<'w, 's> RtsCommanderRapierQueries<'w, 's> {
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
}