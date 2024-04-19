// Chase 
    // Nav is waypoint of chase target

// Chase target = 
    // inactive units 
    // nearby low-health units

// Mover for head
    // Aggregate mover that brings it to the body, vector increases with distance from body.

// From nav send output to body
// Multiply output when player units have died

// Insta-Rotate to face target

// Attack
    // Targeted detection or distance check
    // Timer
    // Short range

use std::any::TypeId;
use bevy_rapier2d::prelude::*;
use bevy::prelude::*;
use bevy_rand::prelude::*;
use rand_core::*;

use random::*;
use attack_laser::*;
use rts_unit_death::*;
use rts_unit_health::*;
use rts_unit_movers::*;
use rts_unit_nav::*;
use rts_unit_team::*;

use super::*;

#[derive(Component, Default)]
pub struct ChaseHead;

#[derive(Component)]
pub struct ToChase(Entity);
waymark!(ToChase);

#[derive(Bundle, Default)]
pub struct BundChase {
    pub to_mover: ToMover,
    pub to_hub: ToHub,
    pub flag: ChaseHead,
    
    pub chase_factor: ChaseFrenzy, 
    pub chase_target: ChaseTarget,

    pub mover_in: TMoveAggregator,
    pub mover_process: LocalTransformMovement,
    pub speed: MoveSpeed,
    
    pub attack: DirectAttackBang,
    pub damage: DirectAttackPower,
    pub attack_timer: AttackTimer,
    pub laser: LaserVisualsOnAttack,
}

#[derive(Component)]
pub struct ChaseTarget(Entity);
impl Default for ChaseTarget {
    fn default() -> Self {
        Self(Entity::PLACEHOLDER)
    }
}
impl ChaseTarget {
    pub fn read(&self) -> Entity {
        return self.0
    }
}

pub fn chase_target_selection_sys(
    inactivity_q: Query<(Entity, &Inactivity), With<PlayerTeam>>,
    health_q: Query<(Entity, &THealth, &MaxHealth), With<PlayerTeam>>,
    mut q: Query<&mut ChaseTarget>
) {
    // Find most inactive enemy
    let mut most_inactive: Entity = Entity::PLACEHOLDER;
    let mut most_inactive_val: f32 = 0.0;
    for (entity, inactivity) in inactivity_q.iter() {
        if inactivity.read() > most_inactive_val {
            most_inactive = entity;
            most_inactive_val = inactivity.read();
        }
    }

    // Find enemy with lowest percentage current health
    let mut lowest_health: Entity = Entity::PLACEHOLDER;
    let mut lowest_health_val: f32 = f32::MAX;
    for (entity, health, max_health) in health_q.iter() {
        let max_health = max_health.read();
        let health = health.0;
        if max_health == 0.0 { continue; }

        let percentage_current_health = health / max_health;

        if !(percentage_current_health < lowest_health_val) {
            continue;
        }

        lowest_health = entity;
        lowest_health_val = percentage_current_health;
    }

    // Create weights
    let most_inactive_val = most_inactive_val * CHASE_INACTIVITY_PRIORITY;
    let lowest_health_val = (1.0 / lowest_health_val) * CHASE_INJURED_PRIORITY;
    
    // Prioritise target based on weights
    let target = {
        if lowest_health_val >= most_inactive_val {
            lowest_health
        } else {
            most_inactive
        }
    };

    // Set target
    for mut chase_target in q.iter_mut() {
        chase_target.0 = target;
    }
}

#[derive(Component, Default)]
pub struct ChaseFrenzy{
    health_factor: f32,
    death_spikes: f32,
}
impl ChaseFrenzy {
    pub fn read(&self) -> f32 {
        return self.health_factor + self.death_spikes
    }
}

pub fn chase_factor_sys(
    mut q: Query<&mut ChaseFrenzy>,
    death_q: Query<&DeathBang, (Changed<DeathBang>, With<PlayerTeam>)>,
    health_q: Query<(&THealth, &MaxHealth), With<PlayerTeam>>,
) {
    // Calculate death spike
    let mut deaths = 0;
    for death in death_q.iter() {
        if death.is_active() {
            deaths = deaths + 1;
        }
    }
    let deaths = deaths as f32;
    let deaths_spike = deaths * CHASE_DEATH_SPIKE;
    
    // Calculate global percentage current health.
    let mut global_max_health = 0.01;
    let mut global_current_health = 0.01;
    for (health, max_health) in health_q.iter() {
        global_current_health = global_current_health + health.0;
        global_max_health = global_current_health + max_health.read();
    }
    let health_factor = global_current_health / global_max_health;
    let health_factor = (1.0 / health_factor) - 1.0;

    for mut chase_factor in q.iter_mut() {
        chase_factor.health_factor = health_factor * CHASE_HEALTH_FRENZY;
        chase_factor.death_spikes = chase_factor.death_spikes + deaths_spike;
    }
}

pub fn death_spike_decay_sys(
    mut q: Query<&mut ChaseFrenzy>,
    time: Res<Time>,
) {
    for mut chase_factor in q.iter_mut() {
        if chase_factor.death_spikes <= 0.0 {
            continue;
        }
        chase_factor.death_spikes = chase_factor.death_spikes - (time.delta_seconds() * (
            CHASE_DEATH_SPIKE_DECAY + (chase_factor.death_spikes * CHASE_DEATH_SPIKE_EXPONENT_DECAY)
        ));
    }
}

// Movement
/* 
pub fn rotate_to_face_target_sys(
    mut q: Query<(&mut Transform, &GlobalTransform, &ChaseTarget), With<ChaseHead>>,
    target_q: Query<&GlobalTransform>,
) {
    for (mut transform, head, target) in q.iter_mut() {
        // Get
        let target = target.0;
        let Ok(target) = target_q.get(target) else { continue; };
        let target = target.translation().truncate();
        
        let head = head.translation().truncate();

        let diff = target - head;
        let direction = diff.normalize();

        let rotation = Quat::from_rotation_z(direction.to_angle());
        
        // Set
        transform.rotation = rotation;
    }
}
*/

/// Body position, prevelance increasing with distance, exponential
/// Target position, prevelance increasing with chase factor, linear
pub fn chase_head_movement_sys(
    mut head_q: Query<(&ToHub, &GlobalTransform, &ChaseTarget, &ChaseFrenzy, &mut TMoveAggregator), With<ChaseHead>>,
    q: Query<&GlobalTransform, Without<ChaseHead>>, // used for body and target
) {
    for (to_hub, transform, target, chase, mut mover) in head_q.iter_mut() {
        // Get
        let hub = to_hub.go();
        let Ok(body) = q.get(hub) else { continue; };
        let body = body.translation().truncate();

        let head = transform.translation().truncate();

        let body_head_distance = body.distance(head);

        let target = target.read();
        let Ok(target) = q.get(target) else { continue; };
        let target = target.translation().truncate();

        // Calculate prevelance
        let body_prevelance = (body_head_distance * CHASE_BODY_PULL) / 1.0;

        let chase = chase.read();
        let chase_prevelance = (chase * CHASE_NECK_GROWTH).clamp(CHASE_NECK_MIN, CHASE_NECK_MAX);
        
        // Calculate move vectors
        let to_body_move = (body - head).normalize_or_zero() * CHASE_BODY_AUTHORITY;

        let to_target_move = (target - head).normalize_or_zero() * CHASE_HEAD_AUTONOMY;

        // To mover
        use rts_unit_movers::Key as MoveKey;
        mover.inputs.insert(MoveKey::External(hub), (to_body_move, body_prevelance)); // Body
        let local = TypeId::of::<ChaseHead>();
        mover.inputs.insert(MoveKey::Local(local), (to_target_move, chase_prevelance)); // Move
    }
}

// Attack
    // Targeted detection or distance check
    // Timer
    // Short range
    // Only attack held target

#[derive(Component, Default)]
pub struct AttackTimer(f32);
impl AttackTimer {
    fn decrement(&mut self, delta: f32) {
        if self.0 <= 0.0 { return; }
        self.0 = self.0 - delta;
    }

    fn increment(&mut self, delta: f32) {
        self.0 = self.0 + delta;
    }

    fn reset(&mut self) {
        self.0 = 0.0;
    }

    fn read(&self) -> f32 {
        return self.0
    }
}

pub fn chase_attack_sys(
    mut q: Query<(&mut DirectAttackBang, &mut AttackTimer, &ChaseTarget, &GlobalTransform), With<ChaseHead>>,
    target_q: Query<&GlobalTransform>,
    time: Res<Time>,
) {
    for (mut attack, mut timer, target, chase) in q.iter_mut() {
        let target = target.0;
        let Ok(target_at) = target_q.get(target) else { continue; };
        let target_at = target_at.translation().truncate();
        
        let chase = chase.translation().truncate();

        let distance = chase.distance(target_at);

        if distance > CHASE_ATTACK_RANGE { 
            timer.decrement(time.delta_seconds());
            continue; 
        }

        timer.increment(time.delta_seconds());
        if timer.read() < CHASE_ATTACK_SPEED { continue; }

        attack.bang(target);
        timer.reset();
    }
}

pub fn chase_frenzy_to_colour(
    mut q: Query<(&mut Sprite, &ChaseFrenzy)>
) {
    let min = CHASE_FRENZY_TO_COLOUR_MIN_MAX.x;
    let max = CHASE_FRENZY_TO_COLOUR_MIN_MAX.y;
    for (mut sprite, frenzy) in q.iter_mut() {
        let current = frenzy.read();
        
        let t = (current + min) / max;
        let colour_min: Vec3 = CHASE_COLOUR.hsl_to_vec3();
        let colour_max = CHASE_FRENZY_COLOUR.hsl_to_vec3();
        let colour = Vec3::lerp(colour_min, colour_max, t);
        
        sprite.color = Color::hsl(colour.x, colour.y, colour.z);
    }
}

#[derive(Component)]
pub struct ChaseNeck{
    pub hub: Entity,
    pub chase: Entity,
}

pub fn chase_neck_sys(
    mut q: Query<(&mut Transform, &ChaseNeck)>,
    transform_q: Query<&GlobalTransform>,
) {
    for (mut transform, neck) in q.iter_mut() {
        let origin = neck.hub;
        let Ok(origin) = transform_q.get(origin) else {continue;};
        let origin = origin.translation().truncate();

        let target = neck.chase;
        let Ok(target) = transform_q.get(target) else {continue;};
        let target = target.translation().truncate();

        let distance = origin.distance(target);
        let diff = target - origin;
        let direction = diff.normalize();
    
        let translation = (origin + (direction * distance * 0.5)).extend(-0.5);
        let rotation = Quat::from_rotation_z(direction.to_angle());
    
        let scale = Vec3::new(distance, NECK_WIDTH, 0.1);

        transform.scale = scale;
        transform.translation = translation;
        transform.rotation = rotation;
    }
}

pub fn chase_to_body_movement_sys(
    chase_q: Query<(&ToMover, &ChaseTarget, &ChaseFrenzy, &GlobalTransform, Entity), With<ChaseHead>>,
    target_q: Query<&GlobalTransform>,
    mut root_q: Query<&mut TMoveDecider>,
) {
    for (to_mover, target, chase, head, chase_entity) in chase_q.iter() {
        // Get
        let head = head.translation().truncate();

        let target = target.read();
        let Ok(target) = target_q.get(target) else { continue; };
        let target = target.translation().truncate();

        let chase = chase.read();

        let chase_move = (target - head).normalize_or_zero();
        let chase_move = chase_move * ((chase * CHASE_HEAD_PULL) + CHASE_BODY_MOVE_BASE_SPEED);
        let chase_move = chase_move.clamp_length(0.0, CHASE_MOVE_LIMIT);


        let chase_prevelance = (chase * CHASE_FRENZY_DOMINANCE) + CHASE_BASE_DOMINANCE; // Move decision prevelance

        // Set
        let hub = to_mover.go();
        let Ok(mut body) = root_q.get_mut(hub) else { continue; };

        use rts_unit_movers::Key as MoveKey;
        body.inputs.insert(MoveKey::External(chase_entity), (chase_move, chase_prevelance));
    }
}