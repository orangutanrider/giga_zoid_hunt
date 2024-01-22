mod basic_kinematic_position;

use bevy::prelude::*;

// Mover is where data is inputed
// The other components (the augments) will enable systems that utillise the mover component to create the movement

// move input is edited directly
#[derive(Component)]
struct BasicMover{
    /// to be updated constantly, to direct the movement
    pub move_input: Vec2, 
    /// parameter, should be set on creation and never updated again
    pub move_power: f32, 
}

// This is un-implmented, but for this one the plan is that I'd have it recieve its move input data via functions
// And then it could ping an event whenever the input was updated
// This'd also mean that it'd have to have a read function, which would probably mean it'd have to copy the data 
// It would be a better implmentation, but I don't need the functionality
#[derive(Component)]
struct AdvancedMover{
    move_input: Vec2,
    move_power: f32,
}

// Mover augments create the movement, by utlising movers
#[derive(Component)]
struct KinematicPositionBasicMoverAugment;
impl KinematicPositionBasicMoverAugment {
    /// Intention is for this value to be used in the movement power calculation as a multiply on the final value.
    /// That way all movers of this type can be adjusted globally, while retaining their own movement power values
    pub const AUG_GLOBAL_POWER: f32 = 1.0; 
}