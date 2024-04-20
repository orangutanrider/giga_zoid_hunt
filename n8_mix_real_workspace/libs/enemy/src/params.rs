use bevy::prelude::*;

// ================================
// GENERIC
pub const MOVE_SPEED: f32 = 1.0;

pub const ATTACK_DAMAGE:f32 = 1.0;
pub const HEALTH: f32 = 12.0;

pub const PHYSICS_SIZE:f32 = 32.0;
pub const BODY_SIZE: f32 = 10.0;

pub const NECK_WIDTH: f32 = 16.0;

// ================================
// BEHAVIOUR GENERIC
// Determines the range at which the behaviours will retain a bit of distance with the targets.
pub const NEARBY_TARGET_LIMITS: Vec2 = Vec2::new(0.0, 0.0); 
pub const TARGET_HEAD_RETRACTION: Vec2 = Vec2::new(0.0, 0.0);
pub const BODY_SLOW: Vec2 = Vec2::new(0.0, 0.0);

// ================================
// STRUCTURE
pub const ROOT_SIZE: Vec2 = Vec2::new(16.0, 4.0);
pub const ROOT_COLOUR: Color = Color::GRAY;

pub const TREE_ROOT_SIZE: Vec2 = Vec2::new(24.0, 4.0);
pub const TREE_ROOT_OFFSET: Vec3 = Vec3::new(0.0, 8.0, 0.0);
pub const TREE_ROOT_COLOUR: Color = Color::GRAY;

pub const HUB_SIZE: Vec2 = Vec2::new(64.0, 64.0);
pub const HUB_OFFSET: Vec2 = Vec2::new(0.0, 36.0);
pub const HUB_COLOUR: Color = Color::hsl(180.0, 1.0, 0.5);

pub const CHASE_SIZE: Vec2 = Vec2::new(24.0, 24.0);
pub const CHASE_OFFSET: Vec2 = Vec2::new(32.0, 70.0);

pub const DEFEND_SIZE: Vec2 = Vec2::new(24.0, 24.0);
pub const DEFEND_OFFSET: Vec2 = Vec2::new(32.0, 70.0);

pub const WILDCARD_SIZE: Vec2 = Vec2::new(24.0, 24.0);
pub const WILDCARD_OFFSET: Vec2 = Vec2::new(32.0, 70.0);

pub const DEATH_FLARE_COLOUR: Color = HUB_COLOUR;
pub const DEATH_FLARE_WIDTH: f32 = 56.0;
pub const DEATH_FLARE_FADE: f32 = 5.0;

// ================================
// LASERS
pub const CHASE_LASER_COLOUR: Color = Color::RED;
pub const CHASE_LASER_WIDTH: f32 = 8.0;
pub const CHASE_LASER_FADE: f32 = 3.0;

pub const DEFEND_LASER_COLOUR: Color = Color::BLUE;
pub const DEFEND_LASER_FADE: f32 = 3.0;
pub const DEFEND_LASER_WIDTH: f32 = 8.0;

pub const WILDCARD_LASER_COLOUR: Color = Color::RED;
pub const WILDCARD_LASER_WIDTH: f32 = 8.0;
pub const WILDCARD_LASER_FADE: f32 = 3.0;

// ================================
// CHASE BEHAVIOUR
pub const CHASE_COLOUR: Color = Color::hsl(200.0, 1.0, 0.5);
pub const CHASE_FRENZY_COLOUR: Color = Color::hsl(360.0, 1.0, 0.45);
pub const CHASE_FRENZY_TO_COLOUR_MIN_MAX: Vec2 = Vec2::new(0.5, 2.5);

pub const CHASE_MOVE_SPEED: f32 = 1.0; // The speed of the chase head.
pub const CHASE_BODY_MOVE_BASE_SPEED: f32 = 0.45; // Influences how much the chaser head is able to create movement in the body.
pub const CHASE_HEAD_PULL:f32 = 0.56; // Determines how much the head's frenzy can move the body.
pub const CHASE_FRENZY_DOMINANCE: f32 = 1.0; // Determines the frenzy's dominance on the body movement.
pub const CHASE_BASE_DOMINANCE: f32 = 1.0; // Detemines the behaviour's base dominance on the body movement.
pub const CHASE_MOVE_LIMIT: f32 = 6.0;

pub const CHASE_INJURED_PRIORITY: f32 = 1.0; // Effects how much the chase behaviour prioritises injured units.
pub const CHASE_INACTIVITY_PRIORITY: f32 = 0.4; // Effects how much the chase behaviour priorities inactive units.

// Every player unit's health and max health as one giant health pool.
// This factor makes it so that the chase behaviour is more prevelant when that health pool is low.
pub const CHASE_HEALTH_FRENZY: f32 = 3.2;
// Whenever a player unit dies, the chase behaviour gains a spike of prevelance equal to this.
pub const CHASE_DEATH_SPIKE: f32 = 0.45;
pub const CHASE_DEATH_SPIKE_DECAY: f32 = 0.04; // The gradual decay of the death spike highs.
pub const CHASE_DEATH_SPIKE_EXPONENT_DECAY: f32 = 0.23; // At high chase frenzy from death spikes, the frenzy will decay faster, proportional to this value.

pub const CHASE_ATTACK_SPEED:f32 = 0.3;
pub const CHASE_ATTACK_RANGE:f32 = 172.0;

pub const CHASE_BODY_AUTHORITY: f32 = 0.1; // This determines how stuck the chase head is to the body.
pub const CHASE_HEAD_AUTONOMY: f32 = 0.8; // This determines how free the chase head is from the body.
pub const CHASE_BODY_PULL:f32 = 1.0; // Determines how the body's authority scales with distance.
pub const CHASE_NECK_GROWTH: f32 = 2.0; // Determines how much the head can extend, when the behaviour is frenzied.
pub const CHASE_NECK_MIN: f32 = 12.0; // Determines min neck extension.
pub const CHASE_NECK_MAX: f32 = 24.0; // Determines max neck extension.

// ================================
// DEFEND BEHAVIOUR
pub const DEFEND_COLOUR: Color = Color::hsl(160.0, 1.0, 0.5);
pub const DEFEND_FRENZY_COLOUR: Color = Color::hsl(0.0, 1.0, 0.45);
pub const DEFEND_FRENZY_TO_COLOUR_MIN_MAX: Vec2 = Vec2::new(0.5, 2.5);

pub const DEFEND_TARGET_UPDATE_RATE: f32 = 2.0;

pub const DEFEND_MOVE_SPEED: f32 = 1.0; // The speed of the defend head.
pub const DEFEND_BODY_MOVE_BASE_SPEED: f32 = 0.2; // Influences how much the chaser head is able to create movement in the body.
pub const DEFEND_HEAD_PULL:f32 = 0.3; // Determines how much the head's frenzy can move the body.
pub const DEFEND_FRENZY_DOMINANCE: f32 = 0.7; // Determines the frenzy's dominance on the body movement.
pub const DEFEND_BASE_DOMINANCE: f32 = 0.0; // Detemines the behaviour's base dominance on the body movement.
pub const DEFEND_MOVE_LIMIT: f32 = 6.0;

pub const DEFEND_SAFE_SPACE_RADIUS: f32 = 240.0; // This is the radius around the body that will arrouse the defender head's prevelance.
pub const DEFEND_SAFE_SPACE_WEIGHT: f32 = 0.09; // This is the effect of that space on the defend behaviour's prevelance.
// This effect increases the more units in the space.

// When the body takes damage, the prevelance of the defender head increases.
// It scales with the damage taken and this weight value.
pub const DEFEND_PAIN_WEIGHT: f32 = 2.6; 
pub const DEFEND_PAIN_DECAY: f32 = 0.03; // The speed at which the pain subsides.
pub const DEFEND_PAIN_EXPONENT_DECAY: f32 = 0.15; 

pub const DEFEND_BODY_AUTHORITY: f32 = 0.19;
pub const DEFEND_HEAD_AUTONOMY: f32 = 1.0;
pub const DEFEND_BODY_PULL: f32 = 0.2; // Determines how the body's authority scales with distance.
pub const DEFEND_NECK_GROWTH: f32 = 2.8; // Determines how much the head can extend, when the behaviour is frenzied.
pub const DEFEND_NECK_MIN: f32 = 3.9;  // Determines min neck extension.
pub const DEFEND_NECK_MAX: f32 = 12.0;  // Determines max neck extension.

pub const DEFEND_ATTACK_RANGE: f32 = 120.0;
// When the defend behaviour is prevelant, it's range will decrease.
pub const DEFEND_FRENZY_RANGE_DECREASE: f32 = 0.3; 
// These values cap the range multiply from the behaviour between them.
pub const DEFEND_MIN_ATTACK_RANGE: f32 = 1.4;
pub const DEFEND_MAX_ATTACK_RANGE: f32 = 3.2;

pub const DEFEND_ATTACK_SPEED: f32 = 1.0;
// When the defend behaviour is prevelant, it'll attack faster, a smaller value will scale this effect.
pub const DEFEND_FRENZY_ATTACK_SPEED: f32 = 1.4;
// These values cap the attack speed between them.
pub const DEFEND_MIN_ATTACK_SPEED: f32 = 3.1;
pub const DEFEND_MAX_ATTACK_SPEED: f32 = 0.1;

// ================================
// WILDCARD BEHAVIOUR
pub const WILDCARD_MOVE_SPEED: f32 = 1.0;

pub const WILDCARD_RANDOM_WAYPOINT_BOUNDS: Vec2 = Vec2::new(1000.0, 500.0);
pub const WILDCARD_WAYPOINT_COMPLETION_DISTANCE: f32 = 10.0;
pub const WILDCARD_WAYPOINT_REFRESH_TIMER_RANG: Vec2 = Vec2::new(2.0, 10.0);

pub const WILDCARD_BODY_PULL: f32 = 1.0;
pub const WILDCARD_NECK_GROWTH: f32 = 1.0;
pub const WILDCARD_NECK_MIN: f32 = 1.0;
pub const WILDCARD_NECK_MAX: f32 = 1.0;
pub const WILDCARD_BODY_AUTHORITY: f32 = 1.0;
pub const WILDCARD_HEAD_AUTONOMY: f32 = 1.0;

pub const WILDCARD_CHASE_FRENZY_THRESHOLD: f32 = 1.0;
pub const WILDCARD_DEFEND_FRENZY_THRESHOLD: f32 = 1.0;
pub const WILDCARD_TOTAL_FRENZY_THRESHOLD: f32 = 1.0;

pub const WILDCARD_HEAD_PULL: f32 = 1.0;
pub const WILDCARD_BODY_MOVE_BASE_SPEED: f32 = 1.0;
pub const WILDCARD_MOVE_LIMIT: f32 = 1.0;
pub const WILDCARD_FRENZY_DOMINANCE: f32 = 1.0;
pub const WILDCARD_BASE_DOMINANCE: f32 = 1.0;

pub const WILDCARD_PERSONA_SWITCHING_HEALTH_THRESHOLD: f32 = 0.5; // Percentage current health
pub const WILDCARD_PERSONA_ACTIVATION_TIME_RANGE: Vec2 = Vec2::new(0.0, 1.0);
pub const WILDCARD_PERSONA_DURATION_TIME_RANGE: Vec2 = Vec2::new(0.0, 1.0);

pub const WILDCARD_MOTIF_COLOUR: Color = Color::WHITE;

pub const WILDCARD_COLOUR: Color = Color::WHITE;
pub const WILDCARD_FRENZY_COLOUR: Color = Color::WHITE;
pub const WILDCARD_FRENZY_COLOUR_MIN_MAX: Vec2 = Vec2::new(0.5, 2.5);

pub const WILDCARD_CHASE_PERSONA_COLOUR: Color = Color::WHITE;
pub const WILDCARD_CHASE_PERSONA_FRENZY_COLOUR: Color = Color::WHITE;
pub const WILDCARD_CHASE_PERSONA_FRENZY_COLOUR_MIN_MAX: Vec2 = Vec2::new(0.5, 2.5);

pub const WILDCARD_DEFEND_PERSONA_COLOUR: Color = Color::WHITE;
pub const WILDCARD_DEFEND_PERSONA_FRENZY_COLOUR: Color = Color::WHITE;
pub const WILDCARD_DEFEND_PERSONA_FRENZY_COLOUR_MIN_MAX: Vec2 = Vec2::new(0.5, 2.5);
