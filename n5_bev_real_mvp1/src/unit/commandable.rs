use bevy::prelude::*;
use super::orders::*;

/// Attach to a unit, by adding it onto the entity that holds their UnitCoreBundle components
#[derive(Component)]
pub struct Commandable {
    unit: Entity, 

    generate_order_curor: u8, // 0 to 31
    current_order_cursor: u8, // 0 to 31
    
    order_cores: [OrderCore; 32],
    attack_target_orders: [AttackTarget; 32],
    attack_move_orders: [AttackMove; 32],
    pure_movement_orders: [PureMovement; 32],
}