use std::any::TypeId;

use bevy::prelude::*;

use rts_unit_detectors::*;
use rts_unit_health::*;
use rts_unit_team::PlayerTeam;

use crate::*;
use super::*;

// Defend target
#[derive(Component)]
pub struct DefendPersonaTarget{
    target: Entity,
    update_cooldown: f32,
}
impl Default for DefendPersonaTarget {
    fn default() -> Self {
        Self{
            target: Entity::PLACEHOLDER,
            update_cooldown: 0.0,
        }
    }
}
impl DefendPersonaTarget {
    pub fn read(&self) -> Entity {
        return self.target
    }
}

// Target = closest
// It is the same but it chooses the 2nd highest priority.
pub fn wc_persona_defend_target_selection_sys(
    enemy_q: Query<(Entity, &GlobalTransform), (With<PlayerTeam>, With<THealth>)>,
    mut defend_q: Query<(&mut DefendPersonaTarget, &ToHub, &WildcardPersona)>,
    hub_q: Query<&GlobalTransform>,
    time: Res<Time>
) {
    for (mut target, to_hub, persona) in defend_q.iter_mut() {
        match persona.0 {
            WildcardPersonas::Wildcard => continue,
            WildcardPersonas::Chase => continue,
            WildcardPersonas::Defend => (),
        }

        //target.update_cooldown = target.update_cooldown + time.delta_seconds();
        //if target.update_cooldown < DEFEND_TARGET_UPDATE_RATE { continue; }

        // Get
        let Ok(body) = hub_q.get(to_hub.go()) else { continue; }; 
        let body = body.translation().truncate(); 

        let mut prev_closest = Entity::PLACEHOLDER;
        let mut closest = Entity::PLACEHOLDER;
        let mut closest_val = f32::MAX;
        for (enemy, enemy_position) in enemy_q.iter() { // This loop doesn't have to be nested in this other loop.
            let enemy_position = enemy_position.translation().truncate();
            
            let distance = body.distance(enemy_position);
            if distance > closest_val { continue; }
            
            closest_val = distance;
            prev_closest = closest;
            closest = enemy;
        }
        let closest = prev_closest;

        // Set
        target.target = closest;
    }
}

// Head movement
// High defend factor = long neck + fast
// Low defend factor = short neck + avg
pub fn wc_persona_defend_head_movement_sys(
    mut head_q: Query<(&mut TMoveAggregator, &PersonaFrenzy, &GlobalTransform, &ToHub, &DefendPersonaTarget, &WildcardPersona), With<WildcardHead>>,
    q: Query<&GlobalTransform>
) {
    for (mut mover, defend, head_location, to_hub, target, persona) in head_q.iter_mut() {
        match persona.0 {
            WildcardPersonas::Wildcard => continue,
            WildcardPersonas::Chase => continue,
            WildcardPersonas::Defend => (),
        }

        // Get
        let hub = to_hub.go();
        let Ok(body) = q.get(hub) else { continue; };
        let body = body.translation().truncate();

        let head = head_location.translation().truncate();

        let body_head_distance = body.distance(head);

        let target = target.read();
        let Ok(target) = q.get(target) else { continue; };
        let target = target.translation().truncate();

        // Calculate body authority
        let body_prevelance = (body_head_distance * DEFEND_BODY_PULL) / 1.0;

        // Calculate head autonomy
        let defend = defend.read();
        let defend_prevelance = (defend * DEFEND_NECK_GROWTH).clamp(DEFEND_NECK_MIN, DEFEND_NECK_MAX);

        // Calculate move vectors
        let to_body_move = (body - head).normalize_or_zero() * DEFEND_BODY_AUTHORITY;
        let to_target_move = (target - head).normalize_or_zero() * DEFEND_HEAD_AUTONOMY;

        // To mover
        use rts_unit_movers::Key as MoveKey;
        mover.inputs.insert(MoveKey::External(hub), (to_body_move, body_prevelance)); // Body
        let local = TypeId::of::<WildcardHead>();
        mover.inputs.insert(MoveKey::Local(local), (to_target_move, defend_prevelance)); // Move
    }
}

// Attack
    // Targeted detection or distance check
    // Timer
    // Short range
    // Only attack held target

#[derive(Component, Default)]
pub struct DefendPersonaAttackTimer(f32);
impl DefendPersonaAttackTimer {
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

pub fn wc_persona_defend_attack_sys(
    mut q: Query<(&mut DirectAttackBang, &mut DefendPersonaAttackTimer, &DefendPersonaTarget, &GlobalTransform, &PersonaFrenzy, &WildcardPersona), With<WildcardHead>>,
    target_q: Query<&GlobalTransform>,
    time: Res<Time>,
) {
    for (mut attack, mut timer, target, head_position, defend, persona) in q.iter_mut() {
        match persona.0 {
            WildcardPersonas::Wildcard => continue,
            WildcardPersonas::Chase => continue,
            WildcardPersonas::Defend => (),
        }

        let target = target.target;
        let Ok(target_at) = target_q.get(target) else { continue; };
        let target_at = target_at.translation().truncate();
        
        let head_position = head_position.translation().truncate();

        let distance = head_position.distance(target_at);

        let defend = defend.read();

        let range_equation = { DEFEND_ATTACK_RANGE *
            ((1.0 / (defend * DEFEND_FRENZY_RANGE_DECREASE))
            .clamp(DEFEND_MIN_ATTACK_RANGE, DEFEND_MAX_ATTACK_RANGE))
        };

        if distance > range_equation { 
            timer.decrement(time.delta_seconds());
            continue; 
        }

        timer.increment(time.delta_seconds());
        if timer.read() < (
            ((DEFEND_ATTACK_SPEED / defend * DEFEND_FRENZY_ATTACK_SPEED)
            .clamp(DEFEND_MAX_ATTACK_SPEED, DEFEND_MIN_ATTACK_SPEED)
        )) { continue; }

        attack.bang(target);
        timer.reset();
    }
}

pub fn wc_persona_defend_to_body_movement_sys(
    defend_q: Query<(&ToMover, &DefendPersonaTarget, &PersonaFrenzy, &GlobalTransform, Entity, &WildcardPersona), With<WildcardHead>>,
    target_q: Query<&GlobalTransform>,
    mut root_q: Query<&mut TMoveDecider>,
) {
    for (to_mover, target, defend, head, defend_entity, persona) in defend_q.iter() {
        match persona.0 {
            WildcardPersonas::Wildcard => continue,
            WildcardPersonas::Chase => continue,
            WildcardPersonas::Defend => (),
        }

        // Get
        let head = head.translation().truncate();

        let target = target.read();
        let Ok(target) = target_q.get(target) else { continue; };
        let target = target.translation().truncate();

        let defend = defend.read();

        let defend_move = (target - head).normalize_or_zero();
        let defend_move = defend_move * ((defend * DEFEND_HEAD_PULL) + DEFEND_BODY_MOVE_BASE_SPEED);
        let defend_move = defend_move.clamp_length(0.0, DEFEND_MOVE_LIMIT);

        let defend_prevelance = (defend * DEFEND_FRENZY_DOMINANCE) + DEFEND_BASE_DOMINANCE; // Move decision prevelance

        // Set
        let root = to_mover.go();
        let Ok(mut body) = root_q.get_mut(root) else { continue; };

        use rts_unit_movers::Key as MoveKey;
        body.inputs.insert(MoveKey::External(defend_entity), (defend_move, defend_prevelance));
    }
}