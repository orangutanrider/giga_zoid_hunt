pub mod pure_move;
pub mod attack_move;
pub mod attack_target;

use std::any::TypeId;

use bevy::prelude::*;

use super::{ActiveOrderTerminal, ClearOrdersBang};

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

pub fn order_processing_sys<Processor, Logic, Terminal, OrderType>(
    mut control_q: Query<(&mut Terminal, &mut ActiveOrderTerminal), With<Processor>>,
    mut logic: Logic,
) where 
    Processor: Component,
    Logic: FnMut(&OrderType) -> bool,
    Terminal: Component + TUnitOrder<OrderType>
{
    for (unit_orders, types) in control_q.iter_mut() {
        // Validate that the current terminal is active.
        let Some(current_type) = types.current() else {
            continue;
        };
        if current_type != TypeId::of::<Terminal>() {
            continue;
        }

        processs_order::<Processor, Logic, Terminal, OrderType>(unit_orders, types, &mut logic);
    }
}

pub fn processs_order<Processor, Logic, Terminal, OrderType>(
    mut unit_orders: Mut<'_, Terminal>,
    mut types: Mut<'_, ActiveOrderTerminal>,
    logic: &mut Logic,
) where 
    Processor: Component,
    Logic: FnMut(&OrderType) -> bool,
    Terminal: Component + TUnitOrder<OrderType>
{
    let Some(current) = unit_orders.current() else {
        types.clear_current();
        return;
    };

    if !logic(&current) {
        return;
    }

    unit_orders.clear_current();
}