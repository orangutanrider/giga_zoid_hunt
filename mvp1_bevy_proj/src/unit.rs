mod commander;
mod orders;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::gameplay_controller::selection::Selectable;
use commander::Commandable;

pub struct InitializePlugin;
impl Plugin for InitializePlugin {
    fn build(&self, app: &mut App) {
        println!("");
        println!("Initializing unit");
    }
}

#[derive(Bundle)]
pub struct UnitCoreBundle { // Root entity and attached components for units
    unit: Unit,
    transform: Transform,

    // Physics
    rigid_body: RigidBody,
    locked_axes: LockedAxes,
    collider: Collider,
}
impl Default for UnitCoreBundle{
    fn default() -> Self {
        UnitCoreBundle { 
            unit: Unit{ 
                entity: Entity::PLACEHOLDER 
            }, 
            transform: Transform::default(),
            
            // Physics
            rigid_body: RigidBody::KinematicPositionBased, 
            locked_axes: LockedAxes::ROTATION_LOCKED, 
            collider: Collider::ball(32.0), 
        }
    }
}

pub struct PlayerUnitCoreBundle { // Root entity and attached components for units
    unit: Unit,
    transform: Transform,

    selectable: Selectable,
    commandable: Commandable,

    // Physics
    rigid_body: RigidBody,
    locked_axes: LockedAxes,
    collider: Collider,
}
impl Default for PlayerUnitCoreBundle{
    fn default() -> Self {
        PlayerUnitCoreBundle { 
            unit: Unit{ 
                entity: Entity::PLACEHOLDER 
            }, 
            transform: Transform::default(),
            
            selectable: Selectable::default(),
            commandable: Commandable::default(),
            
            // Physics
            rigid_body: RigidBody::KinematicPositionBased, 
            locked_axes: LockedAxes::ROTATION_LOCKED, 
            collider: Collider::ball(32.0), 
        }
    }
}

#[derive(Component)]
pub struct Unit {
    pub entity: Entity,
}