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
use crate::*;

#[derive(Component)]
pub struct ChasePersonaTarget(Entity);
impl Default for ChasePersonaTarget {
    fn default() -> Self {
        Self(Entity::PLACEHOLDER)
    }
}
impl ChasePersonaTarget {
    pub fn read(&self) -> Entity {
        return self.0
    }
}

// Targeting is the same, but it chooses the 2nd highest weighted.
pub fn wc_persona_chase_target_selection_sys(
    inactivity_q: Query<(Entity, &Inactivity), With<PlayerTeam>>,
    health_q: Query<(Entity, &THealth, &MaxHealth), With<PlayerTeam>>,
    mut q: Query<(&mut ChasePersonaTarget, &WildcardPersona)>,
) {
    // Find most inactive enemy
    let mut prev_most_inactive: Entity = Entity::PLACEHOLDER;
    let mut most_inactive: Entity = Entity::PLACEHOLDER;
    let mut most_inactive_val: f32 = 0.0;
    for (entity, inactivity) in inactivity_q.iter() {
        if inactivity.read() > most_inactive_val {
            prev_most_inactive = most_inactive;
            most_inactive = entity;
            most_inactive_val = inactivity.read();
        }
    }

    // Find enemy with lowest percentage current health
    let mut prev_lowest_health: Entity = Entity::PLACEHOLDER;
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

        prev_lowest_health = lowest_health;
        lowest_health = entity;
        lowest_health_val = percentage_current_health;
    }

    // Create weights
    let most_inactive_val = most_inactive_val * CHASE_INACTIVITY_PRIORITY;
    let lowest_health_val = (1.0 / lowest_health_val) * CHASE_INJURED_PRIORITY;
    
    // Prioritise target based on weights
    let target = {
        if lowest_health_val >= most_inactive_val {
            prev_lowest_health
        } else {
            prev_most_inactive
        }
    };

    // Set target
    for (mut chase_target, persona) in q.iter_mut() {
        match persona.0 {
            WildcardPersonas::Wildcard => continue,
            WildcardPersonas::Chase => (),
            WildcardPersonas::Defend => continue,
        }

        chase_target.0 = target;
    }
}

/// Body position, prevelance increasing with distance, exponential
/// Target position, prevelance increasing with chase factor, linear
pub fn wc_persona_chase_head_movement_sys(
    mut head_q: Query<(&ToHub, &GlobalTransform, &ChasePersonaTarget, &PersonaFrenzy, &mut TMoveAggregator, &WildcardPersona), With<WildcardHead>>,
    q: Query<&GlobalTransform, Without<WildcardHead>>, // used for body and target
) {
    for (to_hub, transform, target, chase, mut mover, persona) in head_q.iter_mut() {
        match persona.0 {
            WildcardPersonas::Wildcard => continue,
            WildcardPersonas::Chase => (),
            WildcardPersonas::Defend => continue,
        }

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

#[derive(Component, Default)]
pub struct ChasePersonaAttackTimer(f32);
impl ChasePersonaAttackTimer {
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

pub fn wc_persona_chase_attack_sys(
    mut q: Query<(&mut DirectAttackBang, &mut ChasePersonaAttackTimer, &ChasePersonaTarget, &GlobalTransform, &WildcardPersona), With<WildcardHead>>,
    target_q: Query<&GlobalTransform>,
    time: Res<Time>,
) {
    for (mut attack, mut timer, target, chase, persona) in q.iter_mut() {
        match persona.0 {
            WildcardPersonas::Wildcard => continue,
            WildcardPersonas::Chase => (),
            WildcardPersonas::Defend => continue,
        }

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

pub fn wc_persona_chase_to_body_movement_sys(
    chase_q: Query<(&ToMover, &ChasePersonaTarget, &PersonaFrenzy, &GlobalTransform, Entity, &WildcardPersona), With<WildcardHead>>,
    target_q: Query<&GlobalTransform>,
    mut root_q: Query<&mut TMoveDecider>,
) {
    for (to_mover, target, chase, head, chase_entity, persona) in chase_q.iter() {
        match persona.0 {
            WildcardPersonas::Wildcard => continue,
            WildcardPersonas::Chase => (),
            WildcardPersonas::Defend => continue,
        }

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