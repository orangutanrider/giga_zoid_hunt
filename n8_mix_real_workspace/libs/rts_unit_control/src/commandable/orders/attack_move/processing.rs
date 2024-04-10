use bevy::prelude::*;

use crate::validate_active_terminal_c;

use super::*;

#[derive(Component)]
/// Flags a system, that will process a attack-move order.
/// Doing so, when the distance between the entity and the waypoint of the order, is lower than the threshold.
pub struct AMProximityProcessor{
    threshold: f32
}
impl Default for AMProximityProcessor {
    fn default() -> Self {
        Self { threshold: 0.0 }
    }
}
impl AMProximityProcessor {
    pub fn new(threshold:f32) -> Self {
        return Self{threshold}
    }
    
    pub fn threshold(&self) -> f32 {
        return self.threshold
    }
}

// Ideally you split the system into two, so that you can have OrderProcessedAgar be optional.
// a With one and a Without one.
pub fn am_proximity_processing_sys(
    mut control_q: Query<(&mut TActiveOrderType, &mut TAttackMoveOrders, &AMProximityProcessor, &GlobalTransform, &mut OrderProcessedAgar), Changed<GlobalTransform>>
) {
    for (mut order_types, mut unit_orders, processor, transform, mut agar) in control_q.iter_mut() {
        validate_active_terminal_c!(TAttackMoveOrders, order_types);

        let Some(current) = unit_orders.current() else {
            order_types.clear_current();
            continue;
        };

        let distance = current.waypoint.distance(transform.translation().truncate());
        if distance > processor.threshold {
            continue;
        }

        unit_orders.clear_current();
        agar.bang();
    }
}