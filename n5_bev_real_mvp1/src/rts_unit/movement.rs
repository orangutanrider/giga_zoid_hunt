pub mod kinematic_position_movement;

use bevy::prelude::*;

pub struct InitializePlugin;
impl Plugin for InitializePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            kinematic_position_movement::InitializePlugin,
        ));
    }
}

#[derive(Component)]
/// Responsible for recieving move input from behaviour scripts
/// And providing it for the different mover components
struct MoveToMover {
    move_vec: Vec2,
    mover_power: f32,
}
impl MoveToMover {
    pub fn new(mover_power:f32) -> Self {
        return Self { 
            move_vec: Vec2::ZERO, 
            mover_power,
        }
    }
}

impl MoveToMover {
    pub fn read(&self) -> Vec2{
        return self.move_vec * self.mover_power
    }
    
    /// Intended for values between -1 and 1
    pub fn input(&mut self, move_vec: Vec2) {
        self.move_vec = move_vec;
    }
}