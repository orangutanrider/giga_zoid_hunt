use bevy::prelude::*;
use bevy::ecs::system::SystemParam;
use bevy_rapier2d::geometry::Collider;
use bevy_rapier2d::plugin::RapierContext;

use crate::unit::commandable;
use crate::unit::commandable::*;
use crate::unit::UnitID;

use self::orders::*;

use super::add_mode::AddModeInput;
use super::unit_mouse::UnitMouse;
use super::unit_selection::*;

mod input;

pub struct InitializePlugin;
impl Plugin for InitializePlugin {
    fn build(&self, app: &mut App) {
        app
        .add_plugins(input::InitializePlugin);
    }
}

#[derive(SystemParam)]
struct OrderUnitCommands<'w, 's> {
    add_mode: AddModeInput<'w>,
    commandable_q: Query<'w, 's, &'static mut Commandable>,
}
impl<'w, 's> OrderUnitCommands<'w, 's> {
    /// let mut commandable = self.get_mut_commandable(unit_id);
    fn get_mut_commandable(&mut self, unit_id: &UnitID) -> Mut<'_, Commandable> {
        let commandable_q = &mut self.commandable_q;
        let commandable = commandable_q.get_mut(unit_id.0);
        let commandable = commandable.unwrap();

        return commandable;
    }

    /// let mut commandable = self.get_commandable(unit_id);
    fn get_commandable(& self, unit_id: &UnitID) -> & Commandable {
        let commandable_q = & self.commandable_q;
        let commandable = commandable_q.get(unit_id.0);
        let commandable = commandable.unwrap();

        return commandable;
    }
    
    pub fn command_attack(
        &mut self,
        waypoint: Vec2,
        selection_commands: & UnitSelectionCommands, // replace with selected Resoruce if refactor
        rapier_context: &Res<RapierContext>,
    ) {
        let enemy_cast = rapier_context.cast_shape(
            waypoint, 
            0.0, 
            Vec2::ZERO, 
            &Collider::ball(100.0), 
            0.0, 
            crate::rapier_groups::E_NON_SOLID_FILTER,
        );

        if enemy_cast.is_none() {
            self.command_attack_move(waypoint, selection_commands);
        }
        else{
            let target = enemy_cast.unwrap().0;
            self.command_attack_target(selection_commands, target);
        }
    }

    pub fn command_pure_move(
        &mut self,
        waypoint: Vec2,
        selection_commands: & UnitSelectionCommands, // replace with selected Resoruce if refactor
    ) {
        if self.add_mode.is_pressed() {
            for unit_id in selection_commands.selected_iter() {
                let mut commandable = self.get_mut_commandable(unit_id);
                commandable.give_pure_move_order(PureMovementOrder{waypoint});
            }
        }
        else {
            for unit_id in selection_commands.selected_iter() {
                let mut commandable = self.get_mut_commandable(unit_id);
                commandable.clear_orders();
                commandable.give_pure_move_order(PureMovementOrder{waypoint});
            }
        }
    }

    pub fn command_stop(
        &mut self,
        selection_commands: & UnitSelectionCommands, // replace with selected Resoruce if refactor
    ){
        for unit_id in selection_commands.selected_iter() {
            let mut commandable = self.get_mut_commandable(unit_id);
            commandable.clear_orders();
        }
    }
    
    pub fn command_attack_move(
        &mut self,
        waypoint: Vec2,
        selection_commands: & UnitSelectionCommands, // replace with selected Resoruce if refactor
    ) {
        if self.add_mode.is_pressed() {
            for unit_id in selection_commands.selected_iter() {
                let mut commandable = self.get_mut_commandable(unit_id);
                commandable.give_attack_move_order(AttackMoveOrder{waypoint});
            }
        }
        else {
            for unit_id in selection_commands.selected_iter() {
                let mut commandable = self.get_mut_commandable(unit_id);
                commandable.clear_orders();
                commandable.give_attack_move_order(AttackMoveOrder{waypoint});
            }
        }
    }

    pub fn command_attack_target(
        &mut self,
        selection_commands: & UnitSelectionCommands, // replace with selected Resoruce if refactor
        target: Entity,
    ) {
        if self.add_mode.is_pressed() {
            for unit_id in selection_commands.selected_iter() {
                let mut commandable = self.get_mut_commandable(unit_id);
                commandable.give_attack_target_order(AttackTargetOrder{invalidated: false, target_unit: target});
            }
        }
        else {
            for unit_id in selection_commands.selected_iter() {
                let mut commandable = self.get_mut_commandable(unit_id);
                commandable.clear_orders();
                commandable.give_attack_target_order(AttackTargetOrder{invalidated: false, target_unit: target});
            }
        }
    }
}