use bevy::prelude::*;

use crate::validate_active_terminal_c;

use super::*;

#[derive(Component)]
/// Flags a system, that will process a attack-move order.
/// Doing so, when the distance between the entity and the waypoint of the order, is lower than the threshold.
pub struct AMProximityProcessor{
    threshold: f32
}
impl AMProximityProcessor {
    pub fn threshold(&self) -> f32 {
        return self.threshold
    }
}

pub fn pm_proximity_processing_sys(
    mut control_q: Query<(&mut ActiveOrderTerminal, &mut TAttackMoveOrders, &AMProximityProcessor, &GlobalTransform, &mut ProcessCurrentOrderBang), Changed<GlobalTransform>>
) {
    for (mut order_types, unit_orders, processor, transform, mut signal) in control_q.iter_mut() {
        validate_active_terminal_c!(TAttackMoveOrders, order_types);

        let Some(current) = unit_orders.current() else {
            order_types.clear_current();
            continue;
        };

        let distance = current.waypoint.distance(transform.translation().truncate());
        if distance > processor.threshold {
            continue;
        }

        signal.bang();
    }
}