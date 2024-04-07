pub mod pure_move;
pub mod attack_move;
pub mod attack_target;

use bevy::prelude::*;

use self::{
    attack_move::{*, processing::am_proximity_processing_sys},
    attack_target::*,
    pure_move::{*, processing::pm_proximity_processing_sys}
};

use super::*;

pub struct BuiltInOrdersPlugin;
impl Plugin for BuiltInOrdersPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(AttackTargetPlugin);
        app.add_systems(Update, (
            t_unit_order_clear_sys::<TPureMoveOrders, PureMoveOrder>,
            pm_proximity_processing_sys,
            t_unit_order_clear_sys::<TAttackMoveOrders, AttackMoveOrder>,
            am_proximity_processing_sys,
        ));
    }
}

/// Unit order terminal blueprint.
/// Typically, an order terminal is a wrapper/tuple for a vec of the order type.
pub trait TUnitOrder<OrderType> {
    fn clear(&mut self);
    fn clear_current(&mut self);
    fn command(&mut self, order: OrderType);

    fn current(&self) -> Option<OrderType>;
    fn count(&self) -> usize;
    fn iter(&self) -> core::slice::Iter<'_, OrderType>;
}

#[macro_export]
macro_rules! unit_order_terminal { ($terminal:ty, $order:ty) => {
    impl Default for $terminal {
        fn default() -> Self {
            Self(Vec::new())
        }
    }

    impl $terminal {
        pub fn new() -> Self {
            return Self(Vec::new())
        }
    }

    impl TUnitOrder<$order> for $terminal {        
        fn clear(&mut self) {
            self.0.clear();
        }
        fn clear_current(&mut self) {
            self.0.pop();
        }
        fn command(&mut self, order: $order) {
            self.0.push(order);
        }

        fn current(&self) -> Option<$order> {
            let index = self.0.len().wrapping_sub(1);
            return self.0.get(index).copied()
        }
        fn count(&self) -> usize {
            return self.0.len()
        }
        fn iter(&self) -> core::slice::Iter<'_, $order> {
            return self.0.iter()
        }
    }
};}

/// Runs on Update.
/// Prefab clear system .
pub fn t_unit_order_clear_sys<Terminal, OrderType>(
    mut control_q: Query<&mut Terminal, Changed<ClearOrdersBang>>,
) where
    Terminal: Component + TUnitOrder<OrderType>
{
    // It doesn't have to check if the bang is true.
    // Because it can only ever be set to true, externally.
    // And internally, when it sets back to false, it skips change detection.

    for mut terminal in control_q.iter_mut() {
        terminal.clear();
    }
}

#[macro_export]
macro_rules! validate_active_terminal_c { ($data_terminal:ty, $type_terminal:ident) => {
    use std::any::TypeId;
    let Some(current_type) = $type_terminal.current() else {
        continue;
    };
    if current_type != TypeId::of::<$data_terminal>() {
        continue;
    }
};}

#[macro_export]
macro_rules! validate_active_terminal_r { ($data_terminal:ty, $type_terminal:ident) => {
    use std::any::TypeId;
    let Some(current_type) = $type_terminal.current() else {
        return;
    };
    if current_type != TypeId::of::<$data_terminal>() {
        return;
    }
};}