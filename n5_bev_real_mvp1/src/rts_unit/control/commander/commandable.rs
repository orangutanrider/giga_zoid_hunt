use bevy::prelude::*;
use super::orders::*;

#[derive(Component)]
pub struct Commandable {
    orders: Vec<RTSUnitOrder>,
}
impl Default for Commandable {
    fn default() -> Self {
        return Self { orders: Vec::new() }
    }
}
impl Commandable {
    fn new() -> Self{
        return Self { orders: Vec::new() }
    }
}

/// Recieve order methods
impl Commandable {
    pub fn clear_orders(&mut self) {      
        self.orders.clear();
    }

    pub fn give_order(&mut self, order: OrderType) {
        self.orders.push(RTSUnitOrder{order_type: order});
    }
}

/// Read order methods
impl Commandable {
    pub fn current_order(&self) -> RTSUnitOrder {
        return self.orders[self.orders.len() - 1]
    }

    pub fn number_of_orders(&self) -> usize {
        return self.orders.len()
    }

    pub fn orders_iter(&self) -> core::slice::Iter<'_, RTSUnitOrder> {
        return self.orders.iter()
    }
}

// Internal
impl Commandable {
    fn complete_current_order(&mut self) {
        self.orders.pop();
        let e = self.orders.iter();
    }
}