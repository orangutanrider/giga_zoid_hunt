use bevy::{ecs::system::SystemParam, prelude::*, transform};

use super::{commandable::{self, orders::{OrderType, PureMovementOrder}, Commandable}, movement::BasicMover, ProtoUnit, Unit};

pub struct InitializePlugin;
impl Plugin for InitializePlugin {
    fn build(&self, app: &mut App) {
        println!("Initializing unit::player_units::proto_unit::ai");
        app.add_systems(Update, ai_follow_current_order);
    }
}

// AI
// Follow pure move waypoints via basic movement
// For attack target, move towards target, stop when within attack range distance
// For attack move, move towards waypoint, scan for units in range of an aggro distance, move towards units within that range, stop when within attack range of any unit

// For idle state, do not attack, I will add settings for modifying this behaviour later in development
// The plan is to have those behaviour settings be able to changed during gameplay, and set before playing too, but these options will be hidden by default, as to not overwhelm

#[derive(SystemParam)]
struct ProtoUnitAI<'w, 's> {
    commandable_q: ParamSet<'w, 's, (
        Query<'w, 's, &'static mut Commandable, With<ProtoUnit>>,
        Query<'w, 's, &'static Commandable, With<ProtoUnit>>,
    )>,
    mover_q: Query<'w, 's, &'static mut BasicMover, With<ProtoUnit>>,
    transform_q: Query<'w, 's, &'static Transform, With<ProtoUnit>>,
}

fn follow_pure_move(
    mut mover: Mut<'_, BasicMover>,
    position: Vec2,
    order: & PureMovementOrder,
) {
    let move_vec = (order.waypoint - position).normalize_or_zero();
    //println!("{}", order.waypoint);
    //println!("{}", move_vec);
    mover.input_move_vec(move_vec);
}

fn ai_follow_current_order (
    mut params: ProtoUnitAI,
) {
    for commandable in params.commandable_q.p1().iter() {
        let mover = params.mover_q.get_mut(commandable.unit);
        let mut mover = mover.unwrap();
        let transform = params.transform_q.get(commandable.unit);
        let transform = transform.unwrap();
        let position = transform.translation.truncate();
        let current_order = commandable.current_order();
        match current_order.order_type {
            OrderType::Empty => {
                mover.input_move_vec(Vec2::ZERO);
            },
            OrderType::PureMovement => {
                follow_pure_move(mover, position,&commandable.current_order_as_pure_move());
            },
            OrderType::AttackMove => {

            },
            OrderType::AttackTarget => {},
        }
    }
}