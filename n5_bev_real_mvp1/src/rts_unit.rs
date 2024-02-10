#[macro_use]
mod control;
#[macro_use]
mod behaviour;
#[macro_use]
mod soul;
#[macro_use]
mod detection;

mod movement;
mod unit_type;

pub mod parts;
pub mod blocks;

use bevy::prelude::*;

pub struct InitializePlugin;
impl Plugin for InitializePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            control::InitializePlugin,
            unit_type::InitializePlugin,
            behaviour::InitializePlugin,
            movement::InitializePlugin,
        ));
    }
}

#[macro_export]
macro_rules! rts_entity_impls { ($t:ty) => {
    impl $t {
        pub const PLACEHOLDER: Self = Self(Entity::PLACEHOLDER);

        pub fn new(entity: Entity) -> Self {
            return Self(entity)
        }

        pub fn entity(&self) -> Entity {
            return self.0
        }
    }

    impl Default for $t {
        fn default() -> Self {
            return Self::PLACEHOLDER
        }
    }
};}
pub(crate) use rts_entity_impls;

#[derive(Clone, Copy)]
#[derive(Component)]
/// Attach to the root entity
/// An entity that is expected to be a the root entity of an RTS unit
pub struct RTSUnit(Entity);
rts_entity_impls!(RTSUnit);

#[derive(Component)]
/// For entities attached to the root in the transform tree
pub struct ToRTSUnitRoot(Entity);
rts_entity_impls!(ToRTSUnitRoot);