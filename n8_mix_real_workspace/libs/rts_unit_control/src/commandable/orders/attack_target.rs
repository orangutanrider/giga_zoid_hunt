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
/// Pure movement order terminal
pub struct TAttackTargetOrders(Vec<AttackTargetOrder>);
unit_order_terminal!(TAttackTargetOrders, AttackTargetOrder);