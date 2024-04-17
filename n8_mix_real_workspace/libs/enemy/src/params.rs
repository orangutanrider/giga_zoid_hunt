use bevy::prelude::*;

// ================================
// GENERIC
pub const MOVE_SPEED: f32 = 1.0;

pub const ATTACK_DAMAGE:f32 = 1.0;
pub const HEALTH: f32 = 10.0;

pub const PHYSICS_SIZE:f32 = 32.0;
pub const BODY_SIZE: f32 = 10.0;

// ================================
// STRUCTURE
pub const ROOT_SIZE: Vec2 = Vec2::new(16.0, 4.0);
pub const ROOT_COLOUR: Color = Color::GRAY;

pub const TREE_ROOT_SIZE: Vec2 = Vec2::new(24.0, 4.0);
pub const TREE_ROOT_OFFSET: Vec3 = Vec3::new(0.0, 8.0, 0.0);
pub const TREE_ROOT_COLOUR: Color = Color::GRAY;

pub const HUB_SIZE: Vec2 = Vec2::new(64.0, 64.0);
pub const HUB_OFFSET: Vec2 = Vec2::new(0.0, 36.0);
pub const HUB_COLOUR: Color = Color::PURPLE;

pub const CHASE_SIZE: Vec2 = Vec2::new(24.0, 24.0);
pub const CHASE_OFFSET: Vec2 = Vec2::new(32.0, 70.0);
pub const CHASE_COLOUR: Color = Color::RED;

pub const DEFEND_SIZE: Vec2 = Vec2::new(24.0, 24.0);
pub const DEFEND_OFFSET: Vec2 = Vec2::new(32.0, 70.0);
pub const DEFEND_COLOUR: Color = Color::BLUE;

// ================================
// LASERS
pub const CHASE_LASER_COLOUR: Color = Color::RED;
pub const CHASE_LASER_WIDTH: f32 = 8.0;
pub const CHASE_LASER_FADE: f32 = 3.0;

pub const DEFEND_LASER_COLOUR: Color = Color::BLUE;
pub const DEFEND_LASER_FADE: f32 = 3.0;
pub const DEFEND_LASER_WIDTH: f32 = 8.0;

// ================================
// CHASE BEHAVIOUR
pub const CHASE_MOVE_SPEED: f32 = 1.0; // The speed of the chase head.

pub const CHASE_BODY_MOVE:f32 = 0.5; // Influences how much the chaser head is able to create movement in the body.

pub const CHASE_INJURED_PRIORITY: f32 = 6.0; // Effects how much the chase behaviour prioritises injured units.
pub const CHASE_INACTIVITY_PRIORITY: f32 = 0.001; // Effects how much the chase behaviour priorities inactive units.

// Every player unit's health and max health as one giant health pool.
// This factor makes it so that the chase behaviour is more prevelant when that health pool is low.
pub const CHASE_HEALTH_FRENZY: f32 = 1.2;
// Whenever a player unit dies, the chase behaviour gains a spike of prevelance equal to this.
pub const CHASE_DEATH_SPIKE: f32 = 0.2;
pub const CHASE_DEATH_SPIKE_DECAY: f32 = 0.05; // The gradual decay of the death spike highs.

pub const CHASE_ATTACK_SPEED:f32 = 0.25;
pub const CHASE_ATTACK_RANGE:f32 = 128.0;

pub const CHASE_BODY_AUTHORITY: f32 = 0.01; // This determines how stuck the chase head is to the body.
pub const CHASE_HEAD_AUTONOMY: f32 = 1.0; // This determines how free the chase head is from the body.
pub const CHASE_BODY_PULL:f32 = 1.0; // This determines how powerfully the body pulls on the head.
pub const CHASE_HEAD_PULL:f32 = 1.0; // This determines how powerfully the chase head pulls on the body.

// ================================
// DEFEND BEHAVIOUR
pub const DEFEND_MOVE_SPEED: f32 = 1.0; // The speed of the defend head.

pub const DEFEND_SAFE_SPACE_RADIUS: f32 = 215.0; // This is the radius around the body that will arrouse the defender head's prevelance.
pub const PROXIMITY_FACTOR_WEIGHT: f32 = 0.2; // This is the effect of that space on the defend behaviour's prevelance.
// This effect increases the more units in the space.

// When the body takes damage, the prevelance of the defender head increases.
// It scales with the damage taken and this weight value.
pub const DEFEND_PAIN_WEIGHT: f32 = 2.4; 
pub const DEFEND_PAIN_DECAY: f32 = 0.45; // The speed at which the pain subsides.

// Same as chase these parameters determine the movement relationship between the head and body.
pub const DEFEND_BODY_AUTHORITY: f32 = 0.5;
pub const DEFEND_HEAD_AUTONOMY: f32 = 1.0;
pub const DEFEND_BODY_PULL: f32 = 0.2;
pub const DEFEND_HEAD_PULL: f32 = 2.0;

pub const DEFEND_ATTACK_RANGE: f32 = 120.0;
// When the defend behaviour is prevelant, it's range will decrease, a larger value will scale this effect.
pub const DEFEND_FRENZY_RANGE_DECREASE: f32 = 1.0; 
// These values cap the range between them.
pub const DEFEND_MIN_ATTACK_RANGE: f32 = 1.0;
pub const DEFEND_MAX_ATTACK_RANGE: f32 = 3.3;

pub const DEFEND_ATTACK_SPEED: f32 = 1.0;
// When the defend behaviour is prevelant, it'll attack faster, a smaller value will scale this effect.
pub const DEFEND_FRENZY_ATTACK_SPEED: f32 = 0.5;
// These values cap the attack speed between them.
pub const DEFEND_MIN_ATTACK_SPEED: f32 = 1.0;
pub const DEFEND_MAX_ATTACK_SPEED: f32 = 0.1;