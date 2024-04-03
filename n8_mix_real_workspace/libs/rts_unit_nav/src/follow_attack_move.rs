use bevy::prelude::*;

use crate::*;

#[derive(Component)]
/// Switch-able navigation for attack move orders. 
/// Can be switched on and off, because other systems are expected to manage behaviour that pertains to the unit stopping when in range.
/// When off, if active, it will input zero to the navigation output.
pub struct AttackMoveNavigation(pub bool);
impl AttackMoveNavigation {
    pub fn is_on(&self) -> bool {
        return self.0
    }
}

pub fn attack_move_navigation_system(
    mut q: Query<(&mut NavVectorOutput, &GlobalTransform, &TNavAttackMove, &TNavType, &AttackMoveNavigation)>
) {
    for (mut output, transform, order_data, order_type, switch) in q.iter_mut() {
        c_validate_data_terminal!(TNavAttackMove, order_type);

        if !switch.is_on() {
            output.0 = Vec2::ZERO;
            continue;
        }

        let vector = order_data.0;
        let vector = vector - transform.translation().truncate();
        let vector = vector.normalize();

        output.0 = vector;
    }
} 