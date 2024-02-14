pub mod kinematic_position_movement;

use bevy::prelude::*;

use crate::rts_unit::*;

pub struct InitializePlugin;
impl Plugin for InitializePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            kinematic_position_movement::InitializePlugin,
        ));
    }
}

#[derive(Component)]
/// Mover Terminal
/// Responsible for recieving move input from behaviour scripts
/// And providing it for the different movement components (which create the movement)
pub struct TMover {
    move_vec: Vec2,
    mover_power: f32,
}
impl Default for TMover {
    fn default() -> Self {
        Self { 
            move_vec: Vec2::ZERO, 
            mover_power: 0.0 
    }}
}
impl TMover {
    pub fn new(mover_power:f32) -> Self {
        return Self { 
            move_vec: Vec2::ZERO, 
            mover_power,
    }}
}

impl TMover {
    pub fn read(&self) -> Vec2{
        return self.move_vec * self.mover_power
    }
    
    /// Intended for values between -1 and 1
    pub fn input(&mut self, move_vec: Vec2) {
        self.move_vec = move_vec;
    }
}