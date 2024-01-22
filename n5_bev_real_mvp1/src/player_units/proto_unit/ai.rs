use bevy::prelude::*;

pub struct InitializePlugin;
impl Plugin for InitializePlugin {
    fn build(&self, app: &mut App) {
        println!("Initializing unit::player_units::proto_unit::ai");
    }
}

// AI
// Follow pure move waypoints via basic movement
// For attack target, move towards target, stop when within attack range distance
// For attack move, move towards waypoint, scan for units in range of an aggro distance, move towards units within that range, stop when within attack range of any unit

// For idle state, do not attack, I will add settings for modifying this behaviour later in development
// The plan is to have those behaviour settings be able to changed during gameplay, and set before playing too, but these options will be hidden by default, as to not overwhelm