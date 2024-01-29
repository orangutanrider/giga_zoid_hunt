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

pub trait Mover {
    fn read_move_vec(&self) -> Vec2;
    fn input_move_vec(&mut self, move_vec: Vec2);
}

#[derive(Clone, Copy)]
pub struct MoverInternal {
    move_vec: Vec2,
    mover_power: f32,
}
impl MoverInternal {
    fn new(mover_power:f32) -> Self {
        return Self 
        { 
            move_vec: Vec2::ZERO, 
            mover_power,
        }
    }
}