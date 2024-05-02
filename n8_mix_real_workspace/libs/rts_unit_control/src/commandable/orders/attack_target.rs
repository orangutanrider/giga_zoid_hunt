pub mod processing;
pub mod commands;

use bevy::prelude::*;
use std::collections::HashSet;

use crate::unit_order_terminal;
use self::processing::*;
use super::*;

pub struct AttackTargetPlugin;
impl Plugin for AttackTargetPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, (
            current_target_clear_sys,
            current_target_validation_sys,
        ));
        app.add_systems(Update, t_unit_order_clear_sys::<TAttackTargetOrders, AttackTargetOrder>);
        app.add_systems(PostUpdate, (
            target_to_current_sys,
        ));
    }
}

#[derive(Clone, Copy, Debug)]
pub struct AttackTargetOrder {
    pub target: Entity,
}
impl AttackTargetOrder {
    pub fn new(target: Entity) -> Self {
        return Self {
            target, 
        }
    }
}

#[derive(Component)]
/// Attack target order terminal
pub struct TAttackTargetOrders(VecDeque<AttackTargetOrder>);
impl TAttackTargetOrders {
    pub fn move_current(&mut self) -> Option<AttackTargetOrder> {
        return self.0.pop_back()        
    }
}

#[derive(Component)]
/// When attack orders become current, they're moved into here and validated (to check if their targets are still valid, i.e. not dead).
/// (You have to do this, cause entities are just IDs, they aren't concurrently known values like object references are in C#).
pub struct TCurrentTarget(Option<Entity>);
impl Default for TCurrentTarget {
    fn default() -> Self {
        Self(None)
    }
}
impl TCurrentTarget {
    pub fn new() -> Self {
        return Self(None)
    }

    pub fn is_some(&self) -> bool {
        return match self.0 {
            Some(_) => true,
            None => false,
        }
    }

    pub fn read(&self) -> Option<Entity> {
        return self.0
    }
}

unit_order_terminal!(TAttackTargetOrders, AttackTargetOrder);