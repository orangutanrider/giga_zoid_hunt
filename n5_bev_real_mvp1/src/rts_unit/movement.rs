mod basic_kinematic_position;

use bevy::prelude::*;

pub struct InitializePlugin;
impl Plugin for InitializePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            basic_kinematic_position::InitializePlugin,
        ));
    }
}

/// Mover is where data is inputed
/// The other components (the augments) enable systems that utillise the mover component to create the movement
#[derive(Component)]
pub struct Mover{
    /// to be updated constantly, to direct the movement
    move_input: Vec2, 
    /// parameter, should be set on creation and never updated again
    move_power: f32, 
}
impl Mover{
    pub fn new(move_power: f32) -> Mover {
        return BasicMover { move_input: Vec2::ZERO, move_power };
    }

    pub fn read_move_vec(&self) -> Vec2 {
        return self.move_input * self.move_power;
    }

    pub fn input_move_vec(&mut self, move_vec: Vec2) {
        self.move_input = move_vec.normalize_or_zero() * move_vec.length().clamp(0.0, 1.0);
    }
}

/// Augment creates movement, by using mover data
#[derive(Component)]
pub struct KinematicPositionMoverAugment{
    entity: Entity,
}
impl KinematicPositionBasicMoverAugment {
    /// Intention is for this value to be used in the movement power calculation as a multiply on the final value.
    /// That way all movers of this type can be adjusted globally, while retaining their own movement power values
    pub const AUG_GLOBAL_POWER: f32 = 1.0; 

    pub fn new(entity: Entity) -> KinematicPositionBasicMoverAugment {
        return {
            KinematicPositionBasicMoverAugment{
                entity,
            }
        }
    }
}