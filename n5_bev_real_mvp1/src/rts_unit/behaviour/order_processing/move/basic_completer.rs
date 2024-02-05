use bevy::prelude::*;

use crate::rts_unit::*;
use crate::rts_unit::control::prelude::*;

pub struct InitializePlugin;
impl Plugin for InitializePlugin{
    fn build(&self, app: &mut App) {
        app.add_systems(Update, check_for_order_completion);
    }
}

#[derive(Component)]
pub struct BasicMoveOrderCompleter;
impl Default for BasicMoveOrderCompleter {
    fn default() -> Self {
        return Self {}
    }
}
impl BasicMoveOrderCompleter {
    const ORDER_COMPLETE_DISTANCE: f32 = 10.0;

    fn new() -> Self{
        return Self {}
    }
}

fn check_for_order_completion(
    mut control_q: Query<(&mut Commandable, &RTSUnitSubEntity)>,
    transform_q: Query<&Transform>,
) {
    for (commandable, sub_entity) in control_q.iter_mut() {
        let root = sub_entity.root().0;
        let position = transform_q.get(root);
        let position = position.unwrap().translation.truncate();

        let order = commandable.current_order();
        match order.order_type {
            OrderType::PureMovement(pure_move_order) => {
                process_pure_move(commandable, pure_move_order, position);
            },
            OrderType::AttackMove(attack_move_order) => {
                process_attack_move(commandable, attack_move_order, position);
            },
            OrderType::AttackTarget(_) => {},
            OrderType::Empty => {},
        }
    }
}

fn process_pure_move(
    mut commandable: Mut<Commandable>,
    pure_move_order: PureMovementOrder,
    position: Vec2,
) {
    if !pure_move_order.is_within_distance_of(BasicMoveOrderCompleter::ORDER_COMPLETE_DISTANCE, position) {
        return;
    }
    commandable.complete_current_order();
}

fn process_attack_move(
    mut commandable: Mut<Commandable>,
    attack_move_order: AttackMoveOrder,
    position: Vec2,
) {
    if !attack_move_order.is_within_distance_of(BasicMoveOrderCompleter::ORDER_COMPLETE_DISTANCE, position) {
        return;
    }
    commandable.complete_current_order();
}