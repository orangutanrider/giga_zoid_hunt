pub mod velocity;
pub mod transform;
use std::any::TypeId;

pub use velocity::*;
pub use transform::*;

use bevy::{prelude::*, utils::HashMap};
use ref_paths::*;

pub struct MoversPlugin;

impl Plugin for MoversPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, move_aggregator_re_calculate_sys);
        app.add_systems(Update, (
            velocity_movement_sys,
            velocity_movement_aggregator_sys,
            velocity_movement_decider_sys,

            tranform_movement_sys,
            tranform_movement_aggregator_sys,
            tranform_movement_decider_sys,
            
            inactivity_sys,
            move_decider_re_calculate_sys,
        ));
    }
}

#[derive(Component)]
pub struct ToMover(Entity);
waymark!(ToMover);


#[derive(Component)]
pub struct MoveSpeed(f32);
impl Default for MoveSpeed {
    fn default() -> Self {
        Self(0.0)
    }
}
impl MoveSpeed {
    pub fn new(speed: f32) -> Self {
        return MoveSpeed(speed)
    }

    pub fn read(&self) -> f32 {
        return self.0
    }
}

#[derive(Component)]
/// Data terminal.
pub struct TMoveVector(pub Vec2);
impl Default for TMoveVector {
    fn default() -> Self {
        Self(Vec2::ZERO)
    }
}

#[derive(Hash, Clone, Copy, Eq, PartialEq)]
pub enum Key {
    External(Entity),
    Local(TypeId)
}

#[derive(Component)]
/// Input
pub struct TMoveAggregator{
    pub inputs: HashMap<Key, (Vec2, f32)>,
    aggregate: Vec2
}
impl Default for TMoveAggregator {
    fn default() -> Self {
        Self{
            inputs: HashMap::new(),
            aggregate: Vec2::ZERO,
        }
    }
}
impl TMoveAggregator {
    fn re_calculate(&mut self) {
        self.aggregate = Vec2::ZERO;
        
        let len = self.inputs.len();
        if len == 0 { return; }

        let len = len as f32;
        let avg_factor = 1.0 / len;

        for (_, (move_vec, prevelance)) in self.inputs.iter() {
            let move_vec = *move_vec;
            let prevelance = *prevelance;
            let nav_vec = move_vec * avg_factor * prevelance;
            self.aggregate = self.aggregate + nav_vec;
        }
    }
    
    pub fn read(&self) -> Vec2 {
        return self.aggregate
    }
}

pub fn move_aggregator_re_calculate_sys(
    mut q: Query<&mut TMoveAggregator, Changed<TMoveAggregator>>
) {
    for mut aggregator in q.iter_mut() {
        aggregator.bypass_change_detection();
        aggregator.re_calculate();
    }
}

#[derive(Component, Default)]
pub struct Inactivity(f32);
impl Inactivity {
    pub fn read(&self) -> f32 {
        return self.0
    }
}

pub fn inactivity_sys( 
    mut q: Query<(&mut Inactivity, &TMoveVector)>,
    time: Res<Time>
) {
    for (mut inactivity, mover) in q.iter_mut() {
        if mover.0 != Vec2::ZERO {
            inactivity.0 = (inactivity.0 - time.delta_seconds()).clamp(0.0, f32::MAX);
        }
        else {
            inactivity.0 = inactivity.0 + time.delta_seconds();
        }
    }
}

#[derive(Component)]
/// Input
pub struct TMoveDecider{
    pub inputs: HashMap<Key, (Vec2, f32)>,
    decision: Vec2,
}
impl Default for TMoveDecider {
    fn default() -> Self {
        Self{
            inputs: HashMap::new(),
            decision: Vec2::ZERO,
        }
    }
}
impl TMoveDecider {
    fn re_calculate(&mut self) {
        self.decision = Vec2::ZERO;

        let mut decision = Vec2::ZERO;
        let mut dominant_prevelance = 0.0;
        for (_, (move_vec, prevelance)) in self.inputs.iter() {
            if prevelance < &dominant_prevelance { continue; }

            decision = *move_vec;
            dominant_prevelance = *prevelance;
        }

        self.decision = decision;
    }
    
    pub fn read(&self) -> Vec2 {
        return self.decision
    }
}

pub fn move_decider_re_calculate_sys(
    mut q: Query<&mut TMoveDecider, Changed<TMoveDecider>>,
    time: Res<Time>,
) {
    for mut decider in q.iter_mut() {
        decider.bypass_change_detection();
        decider.re_calculate();
    }
}