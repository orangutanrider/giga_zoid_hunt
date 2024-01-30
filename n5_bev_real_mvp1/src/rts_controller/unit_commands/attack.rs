use bevy::prelude::*;
use bevy::ecs::system::SystemParam;

use super::RtsCommanderContext;
use crate::rts_unit::control::prelude::*;
use crate::rts_controller::rapier_queries::RtsControllerRapierQueries;

pub struct InitializePlugin;
impl Plugin for InitializePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, command_attack);
    }
}

#[derive(SystemParam)]
struct AttackInput<'w> {
    keys: Res<'w, Input<KeyCode>>,
}
impl<'w> AttackInput<'w> {
    const KEYS: [KeyCode; 1] = [KeyCode::A];

    fn just_pressed(&self) -> bool {
        return self.keys.any_just_pressed(Self::KEYS);
    }
}

/// Update system
fn command_attack (
    attack_input: AttackInput, 
    mut commander_context: RtsCommanderContext,
    rapier_queries: RtsControllerRapierQueries,
) {
    if !attack_input.just_pressed() {
        return;
    }

    let unit_commander = &mut commander_context.unit_commander;
    let mouse = &commander_context.mouse;
    let add_mode = commander_context.add_mode_input.is_pressed();

    let mut order = OrderType::AttackMove(
        AttackMoveOrder{waypoint: commander_context.mouse.position()}
    );

    let enemy_cast = rapier_queries.cast_for_e_attackable(mouse.position());
    if let Some(enemy_cast) = enemy_cast {
        order = OrderType::AttackTarget(AttackTargetOrder::new(enemy_cast.0))
    }

    unit_commander.command_selection(add_mode, order)
}