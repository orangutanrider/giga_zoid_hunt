pub mod processing;

use bevy::prelude::*;
use std::collections::HashSet;

use crate::unit_order_terminal;
use self::processing::*;
use super::*;

pub struct AttackTargetPlugin;
impl Plugin for AttackTargetPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, (
            abort_current_target_bang_sys,
            target_to_current_sys
        ));
        app.add_systems(Update, t_unit_order_clear_sys::<TAttackTargetOrders, AttackTargetOrder>);
        app.add_systems(PostUpdate, (
            current_target_clear_sys,
            abort_current_target_sys,
            current_target_validation_sys,
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
pub struct TAttackTargetOrders(Vec<AttackTargetOrder>);
impl TAttackTargetOrders {
    pub fn move_current(&mut self) -> Option<AttackTargetOrder> {
        return self.0.pop()        
    }
}

unit_order_terminal!(TAttackTargetOrders, AttackTargetOrder);