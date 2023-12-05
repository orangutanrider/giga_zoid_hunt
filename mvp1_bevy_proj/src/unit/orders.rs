/// unit's AIs follow orders that're given to them from external sources

use bevy::prelude::*;

pub struct InitializePlugin;
impl Plugin for InitializePlugin {
    fn build(&self, app: &mut App) {
        println!("");
        println!("Initializing unit::orders");
    }
}

#[derive(Resource)]
pub struct TerminusOrder {
    order: Entity,
}
impl FromWorld for TerminusOrder{
    fn from_world(world: &mut World) -> Self {
        let order =
        world.spawn (
            OrderCore {
                recieving_unit: Entity::PLACEHOLDER,
                next_order: Entity::PLACEHOLDER,
                order_type: OrderType::Terminus,
            }
        )
        .id();

        return TerminusOrder { order }
    }
}

#[derive(Component)]
pub struct OrderCore {
    pub recieving_unit: Entity, 
    pub next_order: Entity,
    pub order_type: OrderType,
}

#[derive(Clone, Copy)]
#[derive(PartialEq, Eq)]
pub enum OrderType {
    Terminus,
    PureMovement,
    AttackMove,
    AttackTarget,
}

// ATTACK TARGET
#[derive(Component)]
pub struct AttackTarget {
    pub order_entity: Entity,
    pub invalidated: bool,
    pub target_unit: Entity,
}
#[derive(Bundle)]
pub struct OrderAttackTargetBundle {
    order_core: OrderCore,
    attack_target: AttackTarget,
}

// ATTACK MOVE
#[derive(Component)]
pub struct AttackMove {
    pub order_entity: Entity,
    pub waypoint: Vec2,
}
#[derive(Bundle)]
pub struct OrderAttackMoveBundle {
    order_core: OrderCore,
    attack_move: AttackMove,
}

// PURE MOVEMENT
#[derive(Component)]
pub struct PureMovement {
    pub order_entity: Entity,
    pub waypoint: Vec2,
}
#[derive(Bundle)]
pub struct OrderPureMovementBundle {
    order_core: OrderCore,
    pure_movement: PureMovement,
}