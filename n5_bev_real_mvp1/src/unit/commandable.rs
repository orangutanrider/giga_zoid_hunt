use bevy::prelude::*;
use super::orders::*;

/// Attach to a unit, by adding it onto the entity that holds their UnitCoreBundle components
#[derive(Component)]
pub struct Commandable {
    unit: Entity, 
    generate_order_curor: usize,
    current_order_cursor: usize, 
    
    order_cores: [OrderCore; Commandable::MAX_CONCURRENT_ORDERS],
    attack_target_orders: [AttackTarget; Commandable::MAX_CONCURRENT_ORDERS],
    attack_move_orders: [AttackMove; Commandable::MAX_CONCURRENT_ORDERS],
    pure_movement_orders: [PureMovement; Commandable::MAX_CONCURRENT_ORDERS],
}
impl Default for Commandable {
    fn default() -> Self {
        Self { 
            unit: Entity::PLACEHOLDER, 
            generate_order_curor: 0, 
            current_order_cursor: 0,

            order_cores: [OrderCore::EMPTY; Commandable::MAX_CONCURRENT_ORDERS], 
            attack_target_orders: [Default::default(); Commandable::MAX_CONCURRENT_ORDERS], 
            attack_move_orders: [Default::default(); Commandable::MAX_CONCURRENT_ORDERS], 
            pure_movement_orders: [Default::default(); Commandable::MAX_CONCURRENT_ORDERS],
        }
    }
}
impl Commandable {
    pub const MAX_CONCURRENT_ORDERS: usize = 32;

    pub fn clear_orders(&mut self) {
        // Goes from the current order index to the current generation index, and wipes them
        // Does this via recursion

        // Edit
        self.order_cores[self.current_order_cursor] = OrderCore::EMPTY;

        // Iterate
        self.current_order_cursor += 1;
        if self.current_order_cursor == Commandable::MAX_CONCURRENT_ORDERS {
            self.current_order_cursor = 0;
        }

        // Exit
        if self.current_order_cursor == self.generate_order_curor {
            return;
        }

        // Continue
        self.clear_orders();
    }

    pub fn give_pure_move_order(&mut self, order: PureMovement) {
        self.order_cores[self.generate_order_curor] = OrderCore{
            order_type: OrderType::PureMovement,
        };
        self.pure_movement_orders[self.generate_order_curor] = order;
        self.generate_order_curor += 1;
    }

    pub fn give_attack_move_order(&mut self, order: AttackMove) {
        self.order_cores[self.generate_order_curor] = OrderCore{
            order_type: OrderType::AttackMove,
        };
        self.attack_move_orders[self.generate_order_curor] = order;
        self.generate_order_curor += 1;
    }

    pub fn give_attack_target_order(&mut self, order: AttackTarget) {
        self.order_cores[self.generate_order_curor] = OrderCore{
            order_type: OrderType::AttackTarget,
        };
        self.attack_target_orders[self.generate_order_curor] = order;
        self.generate_order_curor += 1;
    }
}