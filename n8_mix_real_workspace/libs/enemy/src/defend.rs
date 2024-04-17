// Defend

// Recent damage increases defend factor
// Many enemies in aggregate detector range increases defend factor

// High defend factor = {
//      more body speed,
//      move away from enemies,
//      more aggressive defence head,
//      more health regen,
//      higher attack speed and damage,
// }

// Low defend factor = {
//      weak long range attacks,
//      reclusive head
// }

// Defence targets = closest

// Defence nav = away from closest
// Defence nav = slow update-rate

use std::any::TypeId;

use bevy::prelude::*;

use rts_unit_detectors::*;
use rts_unit_health::*;
use rts_unit_team::PlayerTeam;

use crate::*;
use super::*;

#[derive(Component, Default)]
pub struct DefendHead;

#[derive(Bundle, Default)]
pub struct BundDefend {
    pub to_mover: ToMover,
    pub to_hub: ToHub,
    pub flag: DefendHead,

    pub factor: DefendFactor,
    pub target: DefendTarget,

    pub mover_in: TMoveAggregator,
    pub mover_process: LocalTransformMovement,
    pub speed: MoveSpeed,
    
    pub attack: DirectAttackBang,
    pub damage: DirectAttackPower,
    pub attack_timer: AttackTimer,
    pub laser: LaserVisualsOnAttack,
}

// Defend factor data collection
#[derive(Component, Default)]
pub struct DefendFactor{
    damaged_factor: f32,
    proximity_factor: u32,

    previous_health: f32,
}
impl DefendFactor {
    pub fn read(&self) -> f32 {
        return (self.damaged_factor * DEFEND_PAIN_WEIGHT) + ((self.proximity_factor as f32) * PROXIMITY_FACTOR_WEIGHT) + 0.001
    }
}

pub fn defend_factor_sys(
    mut q: Query<(&mut DefendFactor, &ToHub), With<DefendHead>>,
    hub_q: Query<(&THealth, &TIntersectionsAggregate)>
) {
    for (mut defend, to_hub) in q.iter_mut() {
        let hub = to_hub.go();
        let Ok((health, detection)) = hub_q.get(hub) else {
            continue;
        };

        // Damage to defend factor
        let before_damage = defend.previous_health;
        if before_damage > health.0 {
            let damage = before_damage - health.0;
            defend.damaged_factor = defend.damaged_factor + damage;
            defend.previous_health = health.0;
        } else {
            defend.previous_health = health.0;
        }

        // Detection to defend factor
        let enemies = detection.0.len(); // num of enemies invading the safe space >:(
        defend.proximity_factor = enemies as u32;
    }
}

pub fn defend_factor_damaged_decay(
    mut q: Query<&mut DefendFactor, With<DefendHead>>,
    time: Res<Time>,
) {
    for mut defend in q.iter_mut() {
        defend.damaged_factor = (defend.damaged_factor - (DEFEND_PAIN_DECAY * time.delta_seconds()))
            .clamp(0.0, f32::MAX);
    }
}

// Defend target
#[derive(Component)]
pub struct DefendTarget(Entity);
impl Default for DefendTarget {
    fn default() -> Self {
        Self(Entity::PLACEHOLDER)
    }
}
impl DefendTarget {
    pub fn read(&self) -> Entity {
        return self.0
    }
}

// Target = closest
pub fn defend_target_selection_sys(
    enemy_q: Query<(Entity, &GlobalTransform), (With<PlayerTeam>, With<THealth>)>,
    mut defend_q: Query<(&mut DefendTarget, &ToHub)>,
    hub_q: Query<&GlobalTransform>,
) {
    for (mut target, to_hub) in defend_q.iter_mut() {
        // Get
        let Ok(body) = hub_q.get(to_hub.go()) else { continue; }; 
        let body = body.translation().truncate();

        let mut closest = Entity::PLACEHOLDER;
        let mut closest_val = f32::MAX;
        for (enemy, enemy_position) in enemy_q.iter() {
            let enemy_position = enemy_position.translation().truncate();
            
            let distance = body.distance(enemy_position);
            if distance > closest_val { continue; }
            
            closest_val = distance;
            closest = enemy;
        }
        let closest = closest;

        // Set
        target.0 = closest;
    }
}

// Head movement
// High defend factor = long neck + fast
// Low defend factor = short neck + avg
pub fn defend_head_movement_sys(
    mut head_q: Query<(&mut TMoveAggregator, &DefendFactor, &GlobalTransform, &ToHub, &DefendTarget), With<DefendHead>>,
    q: Query<&GlobalTransform>
) {
    for (mut mover, defend, head_location, to_hub, target) in head_q.iter_mut() {
        // Get
        let hub = to_hub.go();
        let Ok(body) = q.get(hub) else { continue; };
        let body = body.translation().truncate();

        let head = head_location.translation().truncate();

        let body_head_distance = body.distance(head);

        let target = target.read();
        let Ok(target) = q.get(target) else { continue; };
        let target = target.translation().truncate();

        // Calculate prevelance
        let body_prevelance = (body_head_distance * DEFEND_BODY_PULL) / 1.0;

        //println!("{}", body_prevelance);

        let defend = defend.read();
        let defend_prevelance = defend * DEFEND_HEAD_PULL;

        // Calculate move vectors
        let to_body_move = (body - head).normalize_or_zero() * DEFEND_BODY_AUTHORITY;
        let to_target_move = (target - head).normalize_or_zero() * DEFEND_HEAD_AUTONOMY;

        //println!("{}", to_body_move);
        //println!("{}", to_target_move);

        // To mover
        use rts_unit_movers::Key as MoveKey;
        mover.inputs.insert(MoveKey::External(hub), (to_body_move, body_prevelance)); // Body
        let local = TypeId::of::<DefendHead>();
        mover.inputs.insert(MoveKey::Local(local), (to_target_move, defend_prevelance)); // Move
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

pub fn defend_attack_sys(
    mut q: Query<(&mut DirectAttackBang, &mut AttackTimer, &DefendTarget, &GlobalTransform, &DefendFactor), With<DefendHead>>,
    target_q: Query<&GlobalTransform>,
    time: Res<Time>,
) {
    for (mut attack, mut timer, target, head_position, defend) in q.iter_mut() {
        let target = target.0;
        let Ok(target_at) = target_q.get(target) else { continue; };
        let target_at = target_at.translation().truncate();
        
        let head_position = head_position.translation().truncate();

        let distance = head_position.distance(target_at);

        let defend = defend.read();

        let range_equation = { DEFEND_ATTACK_RANGE *
            ((DEFEND_FRENZY_RANGE_DECREASE / defend)
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