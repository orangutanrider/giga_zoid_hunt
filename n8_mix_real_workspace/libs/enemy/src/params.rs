use bevy::prelude::*;

pub const CHASE_BODY_MOVE:f32 = 1.0;

pub const MOST_INACTIVE_FACTOR:f32 = 1.0;
pub const HEALTH_FACTOR: f32 = 1.0;
pub const DEATH_SPIKE: f32 = 10.0;
pub const DEATH_SPIKE_DECAY: f32 = 10.0;

pub const ATTACK_DAMAGE:f32 = 1.0;
pub const ATTACK_SPEED:f32 = 1.0;
pub const ATTACK_RANGE:f32 = 1.0;

pub const LASER_COLOUR: Color = Color::BLUE;
pub const LASER_WIDTH: f32 = 8.0;
pub const LASER_FADE: f32 = 3.0;

pub const BODY_DISTANCE_SCALAR: f32 = 1.0;
pub const CHASE_SCALAR: f32 = 1.0;
pub const BODY_POWER:f32 = 1.0;
pub const CHASE_POWER:f32 = 1.0;

pub const PHYSICS_SIZE:f32 = 32.0;
pub const BODY_SIZE: f32 = 10.0;
pub const HEALTH: f32 = 10.0;

pub const ROOT_SIZE: Vec2 = Vec2::new(16.0, 4.0);
pub const ROOT_COLOUR: Color = Color::PURPLE;

pub const TREE_ROOT_SIZE: Vec2 = Vec2::new(24.0, 4.0);
pub const TREE_ROOT_OFFSET: Vec3 = Vec3::new(0.0, 8.0, 0.0);
pub const TREE_ROOT_COLOUR: Color = Color::PURPLE;

pub const HUB_SIZE: Vec2 = Vec2::new(64.0, 64.0);
pub const HUB_OFFSET: Vec2 = Vec2::new(0.0, 36.0);
pub const HUB_COLOUR: Color = Color::PURPLE;

pub const CHASE_SIZE: Vec2 = Vec2::new(24.0, 24.0);
pub const CHASE_OFFSET: Vec2 = Vec2::new(32.0, 70.0);
pub const CHASE_COLOUR: Color = Color::PURPLE;

