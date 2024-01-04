/// Commandable declerations 
/// Unit methods for recieving and managing orders

use bevy::prelude::*;
use super::orders::*;

/// Attach to a unit, by adding it onto the entity that holds their UnitCoreBundle components
#[derive(Component)]
pub struct Commandable {
    pub unit: Entity, 

    // (generate order cursor)
    gen_cursor: usize, // Where new orders get created 
    // (current order cursor)
    curr_cursor: usize, // Where the order that is currently getting processed is
    
    // Parralel arrays
    order_cores: [OrderCore; Commandable::MAX_CONCURRENT_ORDERS], // Main, points to the rest of them
    attack_target_orders: [AttackTargetOrder; Commandable::MAX_CONCURRENT_ORDERS],
    attack_move_orders: [AttackMoveOrder; Commandable::MAX_CONCURRENT_ORDERS],
    pure_movement_orders: [PureMovementOrder; Commandable::MAX_CONCURRENT_ORDERS],
}
impl Default for Commandable {
    fn default() -> Self {
        let mut return_val = Self { 
            unit: Entity::PLACEHOLDER, 

            gen_cursor: 0, 
            curr_cursor: 0,

            order_cores: [Default::default(); Commandable::MAX_CONCURRENT_ORDERS], 
            attack_target_orders: [Default::default(); Commandable::MAX_CONCURRENT_ORDERS], 
            attack_move_orders: [Default::default(); Commandable::MAX_CONCURRENT_ORDERS], 
            pure_movement_orders: [Default::default(); Commandable::MAX_CONCURRENT_ORDERS],
        };

        return_val.initialize_order_core_index_data();

        return return_val;
    }
}

/// Constants
impl Commandable {
    pub const MAX_CONCURRENT_ORDERS: usize = 32;
}

/// Recieve orders functions
impl Commandable {
    /// Goes from the current order index to the current generation index, and wipes them
    /// Does this via recursion
    pub fn clear_orders(&mut self) {      
        let cur = &mut self.curr_cursor;
        let gen = self.gen_cursor;

        // Edit
        self.order_cores[*cur].order_type = OrderType::Empty;
        // Iterate
        *cur += 1;
        if *cur == Commandable::MAX_CONCURRENT_ORDERS {
            *cur = 0;
        }
        // Exit
        if *cur == gen {
            return;
        }
        // Continue
        self.clear_orders();
    }

    pub fn give_pure_move_order(&mut self, order: PureMovementOrder) {
        let gen = &mut self.gen_cursor;

        // Set order core's order type
        self.order_cores[*gen].order_type = OrderType::PureMovement;
        // Set parralel entry's data
        self.pure_movement_orders[*gen] = order;
        // Increment generation cursor
        *gen += 1;
    }

    pub fn give_attack_move_order(&mut self, order: AttackMoveOrder) {
        let gen = &mut self.gen_cursor;

        // Set order core's order type
        self.order_cores[*gen].order_type = OrderType::AttackMove;
        // Set parralel entry's data
        self.attack_move_orders[*gen] = order;
        // Increment generation cursor
        *gen += 1;
    }

    pub fn give_attack_target_order(&mut self, order: AttackTargetOrder) {
        let gen = &mut self.gen_cursor;

        // Set order core's order type
        self.order_cores[*gen].order_type = OrderType::AttackTarget;
        // Set parralel entry's data
        self.attack_target_orders[*gen] = order;
        // Increment generation cursor
        *gen += 1;
    }

    // Attack target target invalidation systems are still to be added
}

/// Core read and managment functions
impl Commandable {
    /// Increment/Complete current order, moves cursor along and clears the previous order's data
    pub fn complete_current_order(&mut self) {
        let mut cur = &mut self.curr_cursor;

        self.order_cores[*cur].order_type = OrderType::Empty;
        *cur += 1;
    }

    /// Get copy of current order data
    pub fn current_order(&self) -> OrderCore {
        return self.order_cores[self.curr_cursor].clone();
    }
    /// Get copy of current order data
    pub fn current_order_as_pure_move(&self) -> PureMovementOrder {
        return self.pure_movement_orders[self.curr_cursor].clone();
    }
    /// Get copy of current order data
    pub fn current_order_as_attack_move(&self) -> AttackMoveOrder {
        return self.attack_move_orders[self.curr_cursor].clone();
    }
    /// Get copy of current order data
    pub fn current_order_as_attack_target(&self) -> AttackTargetOrder {
        return self.attack_target_orders[self.curr_cursor].clone();
    }
}

/// Additional read and managment functions
impl Commandable {
    /// Gives the callback a copy of the data for each order
    /// Does this via recursion
    pub fn read_order_list_data(
        &self,
        callback: impl FnMut(OrderCore),
    ){
        self.read_order_cores_from_iter_start_to_generate_order_cursor(callback, self.curr_cursor);
    }
    fn read_order_cores_from_iter_start_to_generate_order_cursor(
        &self,
        mut callback: impl FnMut(OrderCore),
        iter_start: usize,
    ){
        if iter_start >= self.gen_cursor {
            return;
        }
        let order_core = self.order_cores[iter_start].clone();
        callback(order_core);
        self.read_order_cores_from_iter_start_to_generate_order_cursor(callback, iter_start + 1);
    }

    /// Get copy of order data, at index
    pub fn order_at_index(&self, index: usize) -> OrderCore {
        return self.order_cores[index].clone();
    }
    /// Get copy of order data, at index
    pub fn order_at_index_as_pure_move(&self, index: usize) -> PureMovementOrder {
        return self.pure_movement_orders[index].clone();
    }
    /// Get copy of order data, at index
    pub fn order_at_index_as_attack_move(&self, index: usize) -> AttackMoveOrder {
        return self.attack_move_orders[index].clone();
    }
    /// Get copy of order data, at index
    pub fn order_at_index_as_attack_target(&self, index: usize) -> AttackTargetOrder {
        return self.attack_target_orders[index].clone();
    }

    /// Calculates the number of orders, by comparing the cursor positions
    pub fn order_list_len(&self) -> usize {
        let cur = self.curr_cursor;
        let gen = self.gen_cursor;
        if gen < cur {
              return cur - gen;
        }
        else {
             return gen - cur;
        }
    }
    
    /// Returns a copy of the current order cursor position
    pub fn current_order_cursor_position(&self) -> usize {
        return self.curr_cursor.clone();
    }
    /// Returns a copy of the generate order cursor position
    pub fn generate_order_cursor_position(&self) -> usize {
        return self.gen_cursor.clone();
    }
}

/// Misc internal
impl Commandable {
    /// Sets the order core index values, to their array positions
    /// Does this via recursion
    fn initialize_order_core_index_data(&mut self) {
        self.set_order_core_index_data_from_iter_start_to_end(0);
    }
    fn set_order_core_index_data_from_iter_start_to_end(&mut self, iter_start: usize) {
        if iter_start >= Commandable::MAX_CONCURRENT_ORDERS {
            return;
        }
        self.order_cores[iter_start].index = iter_start;
        self.set_order_core_index_data_from_iter_start_to_end(iter_start + 1);
    }
}

/// Debug
impl Commandable {
    pub fn println_all_order_core_slots(&self) {
        println!("println_all_order_core_slots");
        self.println_order_core_data_from_iter_start_to_end(0);
    }
    fn println_order_core_data_from_iter_start_to_end(&self, iter_start: usize) {
        if iter_start >= Commandable::MAX_CONCURRENT_ORDERS {
            return;
        }

        let order_core = self.order_cores[iter_start];
        println!("OrderCore{}, index:{}, type:{:?}", iter_start, order_core.index, order_core.order_type);

        self.println_order_core_data_from_iter_start_to_end(iter_start + 1);
    }

    pub fn println_cursor_positions(&self) {
        println!("println_cursor_positions");
        println!("curr_cursor:{}, gen_cursor:{}", self.curr_cursor, self.gen_cursor);
    }

    pub fn println_order_data(&self) { 
        println!("println_order_data");
        self.println_order_data_from_iter_start_to_gen_cursor(0);
    }
    fn println_order_data_from_iter_start_to_gen_cursor(&self, iter_start: usize) {
        if iter_start >= self.gen_cursor {
            return;
        }

        println!("{}", iter_start);

        let order_core = self.order_cores[iter_start];
        println!("(OrderCore{}), index:{}, type:{:?}", iter_start, order_core.index, order_core.order_type);

        match order_core.order_type {
            OrderType::Empty => {},
            OrderType::AttackMove => {
                let attack_move = self.attack_move_orders[iter_start];
                println!("(AttackMove{}), waypoint:{}", iter_start, attack_move.waypoint);
            },
            OrderType::PureMovement => {
                let pure_move = self.pure_movement_orders[iter_start];
                println!("(PureMove{}), waypoint:{}", iter_start, pure_move.waypoint);
            },
            OrderType::AttackTarget => {
                let attack_target = self.attack_target_orders[iter_start];
                println!("(AttackTarget{}), target_unit:{}, Invalidated:{}", iter_start, attack_target.target_unit.index(), attack_target.invalidated);
            },
        }
        self.println_order_data_from_iter_start_to_gen_cursor(iter_start + 1);
    }
}