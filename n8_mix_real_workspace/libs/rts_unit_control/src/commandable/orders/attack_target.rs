pub mod processing;

use std::collections::HashSet;

use crate::unit_order_terminal;
use super::*;

#[derive(Clone, Copy)]
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