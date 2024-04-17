use bevy::prelude::*;

// ================================
// GENERIC
pub const MOVE_SPEED: f32 = 1.0;

pub const HEALTH: f32 = 5.1;
pub const HEALTH_REGEN: f32 = 0.1;

pub const AGGRO_RANGE: f32 = 280.0;
pub const ATTACK_RANGE: f32 = 210.0;

pub const ATTACK_POWER: f32 = 0.1;
pub const ATTACK_SPEED: f32 = 0.75;
pub const ATTACK_ANIMATION_TIME: f32 = 1.1;

pub const PHYSICS_SIZE: f32 = 10.0;
pub const BODY_SIZE: f32 = 10.0;

pub const ORDER_COMPLETE_DISTANCE: f32 = 5.0;

// ================================
// STRUCTURE
pub const ROOT_SIZE: Vec2 = Vec2::new(16.0, 4.0);
pub const ROOT_OFFSET: Vec3 = Vec3::new(0.0, 0.0, 1.0);
pub const ROOT_COLOUR: Color = Color::GRAY;

pub const TREE_ROOT_SIZE: Vec2 = Vec2::new(16.0, 2.0);
pub const TREE_ROOT_OFFSET: Vec3 = Vec3::new(0.0, 0.0, 2.0);
pub const TREE_ROOT_COLOUR: Color = Color::BLACK;

pub const HUB_SIZE: Vec2 = Vec2::new(16.0, 20.0);
pub const HUB_OFFSET: Vec2 = Vec2::new(0.0, 16.0);
pub const LOW_HEALTH_COLOUR: Color = Color::hsl(298.0, 1.0, 0.1);
pub const FULL_HEALTH_COLOUR: Color = Color::ORANGE_RED;

pub const NODES_SIZE: Vec2 = Vec2::new(4.0, 4.0);
pub const NODES_Y_OFFSET: f32 = 16.0;
pub const NODES_X_OFFSET: f32 = 6.0;

// ================================
// LASER
pub const LASER_FADE: f32 = 3.0;
pub const LASER_WIDTH: f32 = 8.0;
pub const LASER_COLOUR: Color = Color::PINK;