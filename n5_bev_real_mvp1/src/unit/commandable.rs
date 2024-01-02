/// Unit methods for recieving and storing orders

use bevy::prelude::*;
use super::orders::*;

/// Attach to a unit, by adding it onto the entity that holds their UnitCoreBundle components
#[derive(Component)]
pub struct Commandable {
    unit: Entity, 
    generate_order_cursor: usize,
    current_order_cursor: usize, 
    
    order_cores: [OrderCore; Commandable::MAX_CONCURRENT_ORDERS],
    attack_target_orders: [AttackTargetOrder; Commandable::MAX_CONCURRENT_ORDERS],
    attack_move_orders: [AttackMoveOrder; Commandable::MAX_CONCURRENT_ORDERS],
    pure_movement_orders: [PureMovementOrder; Commandable::MAX_CONCURRENT_ORDERS],
}
impl Default for Commandable {
    fn default() -> Self {
        let mut return_val = Self { 
            unit: Entity::PLACEHOLDER, 
            generate_order_cursor: 0, 
            current_order_cursor: 0,

            order_cores: [OrderCore::EMPTY; Commandable::MAX_CONCURRENT_ORDERS], 
            attack_target_orders: [Default::default(); Commandable::MAX_CONCURRENT_ORDERS], 
            attack_move_orders: [Default::default(); Commandable::MAX_CONCURRENT_ORDERS], 
            pure_movement_orders: [Default::default(); Commandable::MAX_CONCURRENT_ORDERS],
        };

        return_val.initialize_order_core_indexes();

        return return_val;
    }
}

// RECIEVE ORDERS
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
        if self.current_order_cursor == self.generate_order_cursor {
            return;
        }

        // Continue
        self.clear_orders();
    }

    pub fn give_pure_move_order(&mut self, order: PureMovementOrder) {
        self.order_cores[self.generate_order_cursor] = OrderCore{
            index: self.order_cores[self.generate_order_cursor].index,
            order_type: OrderType::PureMovement,
        };
        self.pure_movement_orders[self.generate_order_cursor] = order;
        self.generate_order_cursor += 1;
    }

    pub fn give_attack_move_order(&mut self, order: AttackMoveOrder) {
        self.order_cores[self.generate_order_cursor] = OrderCore{
            index: self.order_cores[self.generate_order_cursor].index,
            order_type: OrderType::AttackMove,
        };
        self.attack_move_orders[self.generate_order_cursor] = order;
        self.generate_order_cursor += 1;
    }

    pub fn give_attack_target_order(&mut self, order: AttackTargetOrder) {
        self.order_cores[self.generate_order_cursor] = OrderCore{
            index: self.order_cores[self.generate_order_cursor].index,
            order_type: OrderType::AttackTarget,
        };
        self.attack_target_orders[self.generate_order_cursor] = order;
        self.generate_order_cursor += 1;
    }
}

// READ AND MANAGE ORDERS
impl Commandable {
    // Current orders iter or recursive thing
    pub fn read_on_each_current_order(
        &self,
        callback: impl FnMut(OrderCore),
    ){
        self.give_callback_data_from_start_to_generate_cursor(callback, self.current_order_cursor);
    }
    fn give_callback_data_from_start_to_generate_cursor(
        &self,
        mut callback: impl FnMut(OrderCore),
        iter_start: usize,
    ){
        if iter_start >= self.generate_order_cursor {
            return;
        }
        let order_core = self.order_cores[iter_start].clone();
        callback(order_core);
        self.give_callback_data_from_start_to_generate_cursor(callback, iter_start + 1);
    }

    // Increment/Complete current order
    pub fn complete_current_order(&mut self) {
        self.order_cores[self.current_order_cursor].order_type = OrderType::Empty;
        self.current_order_cursor += 1;
    }

    // Current order
    pub fn current_order(&self) -> OrderCore {
        return self.order_cores[self.current_order_cursor].clone();
    }
    pub fn current_order_as_pure_move(&self) -> PureMovementOrder {
        return self.pure_movement_orders[self.current_order_cursor].clone();
    }
    pub fn current_order_as_attack_move(&self) -> AttackMoveOrder {
        return self.attack_move_orders[self.current_order_cursor].clone();
    }
    pub fn current_order_as_attack_target(&self) -> AttackTargetOrder {
        return self.attack_target_orders[self.current_order_cursor].clone();
    }

    // Current order list length
    pub fn current_order_list_length(&self) -> usize {
        if self.generate_order_cursor < self.current_order_cursor {
            return self.current_order_cursor - self.generate_order_cursor;
        }
        else{
            return self.generate_order_cursor - self.current_order_cursor;
        }
    }

    // Cursor positions
    pub fn current_order_cursor_position(&self) -> usize {
        return self.current_order_cursor.clone();
    }

    pub fn generate_order_cursor_position(&self) -> usize {
        return self.generate_order_cursor.clone();
    }
}

// MISC INTERNAL
impl Commandable {
    fn initialize_order_core_indexes(&mut self) {
        self.iterate_and_set_order_core_indexes_from(0);
    }

    fn iterate_and_set_order_core_indexes_from(&mut self, iter_start: usize) {
        if iter_start >= Commandable::MAX_CONCURRENT_ORDERS {
            return;
        }
        self.order_cores[iter_start].index = iter_start;
        self.iterate_and_set_order_core_indexes_from(iter_start + 1);
    }
}