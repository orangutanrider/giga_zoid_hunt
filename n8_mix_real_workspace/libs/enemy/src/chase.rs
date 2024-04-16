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

use attack_laser::LaserVisualsOnAttack;
use bevy_rapier2d::prelude::*;
use bevy::{prelude::*, time::Stopwatch};
use rts_unit_death::*;
use rts_unit_health::*;
use rts_unit_movers::*;
use rts_unit_nav::*;
use rts_unit_team::*;

use super::*;

#[derive(Component, Default)]
pub struct Chase;

#[derive(Bundle, Default)]
pub struct BundChase {
    pub to_mover: ToMover,
    pub to_hub: ToHub,
    pub flag: Chase,
    
    pub chase_factor: ChaseFactor, 
    pub chase_target: ChaseTarget,

    pub mover_in: TMoveAggregator,
    pub mover_process: LocalTransformMovement,

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
    let mut most_inactive: Entity = Entity::PLACEHOLDER;
    let mut most_inactive_val: f32 = -1.0;

    for (entity, inactivity) in inactivity_q.iter() {
        if inactivity.read() > most_inactive_val {
            most_inactive = entity;
            most_inactive_val = inactivity.read();
        }
    }

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

    let most_inactive_val = {
        if most_inactive_val <= 0.0 {
            f32::MAX
        } else {
            MOST_INACTIVE_FACTOR / most_inactive_val
        }
    };
    let lowest_health_val = lowest_health_val;
    
    let target = {
        if lowest_health_val < most_inactive_val {
            lowest_health
        } else {
            most_inactive
        }
    };

    for mut chase_target in q.iter_mut() {
        chase_target.0 = target;
    }
}

#[derive(Component, Default)]
pub struct ChaseFactor{
    health_factor: f32,
    death_spikes: f32,
}
impl ChaseFactor {
    pub fn read(&self) -> f32 {
        return self.health_factor + self.death_spikes
    }
}

pub fn chase_factor_sys(
    mut q: Query<&mut ChaseFactor>,
    death_q: Query<&DeathBang, (Changed<DeathBang>, With<PlayerTeam>)>,
    health_q: Query<(&THealth, &MaxHealth), With<PlayerTeam>>,
) {
    let mut deaths = 0;
    for death in death_q.iter() {
        if death.is_active() {
            deaths = deaths + 1;
        }
    }
    
    let mut global_max_health = 0.01;
    let mut global_current_health = 0.01;
    for (health, max_health) in health_q.iter() {
        global_current_health = global_current_health + health.0;
        global_max_health = global_current_health + max_health.read();
    }

    let deaths = deaths as f32;
    let health_factor = global_current_health / global_max_health;

    for mut chase_factor in q.iter_mut() {
        chase_factor.health_factor = health_factor * HEALTH_FACTOR;
        chase_factor.death_spikes = chase_factor.death_spikes + (deaths * DEATH_SPIKE);
    }
}

pub fn death_spike_decay_sys(
    mut q: Query<&mut ChaseFactor>,
    time: Res<Time>,
) {
    for mut chase_factor in q.iter_mut() {
        if chase_factor.death_spikes <= 0.0 {
            continue;
        }
        chase_factor.death_spikes = chase_factor.death_spikes - (time.delta_seconds() * DEATH_SPIKE_DECAY);
    }
}

// Movement
pub fn rotate_to_face_target_sys(
    mut q: Query<(&mut Transform, &GlobalTransform, &ChaseTarget), With<Chase>>,
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

/// Body position, prevelance increasing with distance, exponential
/// Target position, prevelance increasing with chase factor, linear
pub fn head_movement_sys(
    mut head_q: Query<(&ToHub, &GlobalTransform, &ChaseTarget, &ChaseFactor, &mut TMoveAggregator), With<Chase>>,
    q: Query<&GlobalTransform, Without<Chase>>, // used for body and target
) {
    // body_head distance.

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
        let body_prevelance = (body_head_distance / BODY_DISTANCE_SCALAR).exp();

        let chase = chase.read();
        let chase_prevelance = chase * CHASE_SCALAR;

        // Calculate move vectors
        let to_body_move = (body - head).normalize_or_zero() * BODY_POWER;

        let to_target_move = (head - target).normalize_or_zero() * CHASE_POWER;

        //println!("{}", to_body_move);

        // To mover
        use rts_unit_movers::Key as MoveKey;
        mover.inputs.insert(MoveKey::External(hub), (to_body_move, body_prevelance)); // Body
        let local = TypeId::of::<Chase>();
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
    mut q: Query<(&mut DirectAttackBang, &mut AttackTimer, &ChaseTarget, &GlobalTransform), With<Chase>>,
    target_q: Query<&GlobalTransform>,
    time: Res<Time>,
) {
    for (mut attack, mut timer, target, chase) in q.iter_mut() {
        let target = target.0;
        let Ok(target_at) = target_q.get(target) else { continue; };
        let target_at = target_at.translation().truncate();
        
        let chase = chase.translation().truncate();

        let distance = chase.distance(target_at);

        if distance > ATTACK_RANGE { 
            timer.decrement(time.delta_seconds());
            continue; 
        }

        timer.increment(time.delta_seconds());
        if timer.read() < ATTACK_SPEED { continue; }

        attack.bang(target);
        timer.reset();
    }
}