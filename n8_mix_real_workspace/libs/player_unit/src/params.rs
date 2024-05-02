use bevy::prelude::*;

// ================================
// GENERIC
pub(crate) const MOVE_SPEED: f32 = 1.5;

pub(crate) const HEALTH: f32 = 2.1;
pub(crate) const HEALTH_REGEN: f32 = 0.05;

pub(crate) const AGGRO_RANGE: f32 = 280.0;
pub(crate) const ATTACK_RANGE: f32 = 220.0;

pub(crate) const ATTACK_POWER: f32 = 0.06;
pub(crate) const ATTACK_SPEED: f32 = 0.20;
pub(crate) const ATTACK_ANIMATION_TIME: f32 = 0.4;

pub(crate) const PHYSICS_SIZE: f32 = 10.0;
pub(crate) const BODY_SIZE: f32 = 10.0;

pub(crate) const ORDER_COMPLETE_DISTANCE: f32 = 5.0;

// ================================
// STRUCTURE
pub(crate) const SELECTION_MOTIF_SIZE: Vec2 = Vec2::new(72.0, 24.0);
pub(crate) const SELECTION_MOTIF_OFFSET: Vec3 = Vec3::new(0.0, -2.0, -100.0);
pub(crate) const SELECTION_MOTIF_COLOUR: Color = Color::hsla(110., 1., 0.35, 0.7);

pub(crate) const ROOT_SIZE: Vec2 = Vec2::new(16.0, 6.0);
pub(crate) const ROOT_COLOUR: Color = Color::hsl(0.0, 0.0, 0.3);
pub(crate) const ROOT_Z_OFFSET: f32 = 0.0;

pub(crate) const TREE_ROOT_SIZE: Vec2 = Vec2::new(18.0, 6.0);
pub(crate) const TREE_ROOT_OFFSET: Vec3 = Vec3::new(0.0, 6.0, 0.1);
pub(crate) const TREE_ROOT_COLOUR: Color = Color::hsl(0.0, 0.0, 0.5);

pub(crate) const HUB_SIZE: Vec2 = Vec2::new(20.0, 20.0);
pub(crate) const HUB_OFFSET: Vec3 = Vec3::new(0.0, 13.0, 0.1);
pub(crate) const FULL_HEALTH_COLOUR: Color = Color::hsl(0.0, 0.0, 0.7);
pub(crate) const LOW_HEALTH_COLOUR: Color = Color::hsl(0.0, 0.66, 0.5);

pub(crate) const DEATH_FLARE_COLOUR: Color = LOW_HEALTH_COLOUR;
pub(crate) const DEATH_FLARE_WIDTH: f32 = 1.0;
pub(crate) const DEATH_FLARE_FADE: f32 = 2.0;

// ================================
// NODES STRUCTURE
pub(crate) const NODES_SIZE: Vec2 = Vec2::new(6.0, 6.0);
pub(crate) const NODES_Z_OFFSET: f32 = 0.1;
pub(crate) const NODES_OFF_COLOUR: Color = Color::hsl(0.0, 0.0, 0.8);

pub(crate) const AGGRO_D_SIZE: Vec2 = NODES_SIZE;
pub(crate) const AGGRO_D_OFFSET: Vec3 = Vec3::new(8.0, 8.0, NODES_Z_OFFSET);
pub(crate) const AGGRO_D_ON_COLOUR: Color = Color::hsl(280., 1., 0.5);
pub(crate) const AGGRO_D_OFF_COLOUR: Color = NODES_OFF_COLOUR;

pub(crate) const ATTACK_D_SIZE: Vec2 = NODES_SIZE;
pub(crate) const ATTACK_D_OFFSET: Vec3 = Vec3::new(-8.0, 8.0, NODES_Z_OFFSET);
pub(crate) const ATTACK_D_ON_COLOUR: Color = Color::hsl(320., 1., 0.5);
pub(crate) const ATTACK_D_OFF_COLOUR: Color = NODES_OFF_COLOUR;

pub(crate) const IDLE_SIZE: Vec2 = NODES_SIZE;
pub(crate) const IDLE_OFFSET: Vec3 = Vec3::new(0.0, 0.0, NODES_Z_OFFSET);
pub(crate) const IDLE_ON_COLOUR: Color = Color::hsl(10., 1., 0.5);
pub(crate) const IDLE_OFF_COLOUR: Color = NODES_OFF_COLOUR;

pub(crate) const MOVE_SIZE: Vec2 = NODES_SIZE;
pub(crate) const MOVE_OFFSET: Vec3 = Vec3::new(8.0, 0.0, NODES_Z_OFFSET);
pub(crate) const MOVE_ON_COLOUR: Color = Color::hsl(120., 0.9, 0.4);
pub(crate) const MOVE_OFF_COLOUR: Color = NODES_OFF_COLOUR;

pub(crate) const CHASE_SIZE: Vec2 = NODES_SIZE;
pub(crate) const CHASE_OFFSET: Vec3 = Vec3::new(-8.0, 0.0, NODES_Z_OFFSET);
pub(crate) const CHASE_ON_COLOUR: Color = Color::hsl(20., 0.9, 0.4);
pub(crate) const CHASE_OFF_COLOUR: Color = NODES_OFF_COLOUR;

pub(crate) const ATTACK_SIZE: Vec2 = NODES_SIZE;
pub(crate) const ATTACK_OFFSET: Vec3 = Vec3::new(0.0, 8.0, NODES_Z_OFFSET);
pub(crate) const ATTACK_ON_COLOUR: Color = LASER_COLOUR;
pub(crate) const ATTACK_OFF_COLOUR: Color = NODES_OFF_COLOUR;

// ================================
// LASER
pub(crate) const LASER_FADE: f32 = 3.0;
pub(crate) const LASER_WIDTH: f32 = 8.0;
pub(crate) const LASER_COLOUR: Color = Color::hsl(350., 1., 0.5);