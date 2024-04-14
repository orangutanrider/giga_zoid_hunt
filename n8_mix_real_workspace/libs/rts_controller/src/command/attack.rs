use bevy::{ecs::system::SystemParam, prelude::*};
use bevy_rapier2d::prelude::*;
use mouse_pos::CursorWorldPos;

use rts_unit_control::{commandable::orders::attack_target::commands::SelectionAttackTargetCommands, prelude::*};

use crate::{add_mode::AddModeInput, rapier::PhysicsQueries};

#[derive(SystemParam)]
pub struct AttackInput<'w> {
    keys: Res<'w, ButtonInput<KeyCode>>,
    add_mode: AddModeInput<'w>,
    mouse_pos: Res<'w, CursorWorldPos>,
}
impl<'w> AttackInput<'w> {
    const KEYS: [KeyCode; 1] = [KeyCode::KeyA];

    pub fn just_pressed(&self) -> bool {
        return self.keys.any_just_pressed(Self::KEYS);
    }

    pub fn add_mode(&self) -> bool {
        return self.add_mode.is_pressed()
    }

    pub fn pos(&self) -> Vec2 {
        return self.mouse_pos.pos();
    }
}

pub fn command_attack_sys(
    input: AttackInput, 
    rapier: PhysicsQueries,
    mut selection_commands: ParamSet<(
        SelectionCommands<TAttackMoveOrders, AttackMoveOrder>, // attack move
        SelectionAttackTargetCommands // attack target
    )>,
) {
    if !input.just_pressed() {
        return;
    }

    match rapier.cast_for_e_attackable(input.pos()) {
        Some(cast) => command_attack_target(cast, input.add_mode(), selection_commands.p1()),
        None => command_attack_move(input.pos(), input.add_mode(), selection_commands.p0()),
    }
}

fn command_attack_target(
    cast: (Entity, Toi),
    add_mode: bool,
    mut selection_commands: SelectionAttackTargetCommands,
) {
    let order = AttackTargetOrder::new(cast.0);
    selection_commands.command(add_mode, &order);
}

fn command_attack_move(
    location: Vec2,
    add_mode: bool,
    mut selection_commands: SelectionCommands<TAttackMoveOrders, AttackMoveOrder>,
) {
    let order = AttackMoveOrder::new(location);
    selection_commands.command(add_mode, &order);
}