use crate::unit_order_terminal;
use super::*;

#[derive(Clone, Copy)]
pub struct AttackTargetOrder {
    pub target: Option<Entity>,
}
impl Default for AttackTargetOrder {
    fn default() -> Self {
        return Self {
            target: None, 
        }
    }
}
impl AttackTargetOrder {
    pub fn new(target: Entity) -> Self {
        return Self {
            target: Some(target), 
        }
    }
}

#[derive(Component)]
/// Pure movement order terminal
pub struct TAttackTargetOrders(Vec<AttackTargetOrder>);
unit_order_terminal!(TAttackTargetOrders, AttackTargetOrder);