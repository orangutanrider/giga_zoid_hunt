pub mod commandable;
pub mod orders;

use bevy::prelude::*;
use bevy::ecs::system::SystemParam;

use crate::rts_unit::*;
use self::orders::*;
use self::commandable::*;
use super::selector::SelectedUnits;

pub struct InitializePlugin;
impl Plugin for InitializePlugin {
    fn build(&self, app: &mut App) {

    }
}

#[derive(SystemParam)]
pub struct UnitCommander<'w, 's> {
    unit_selection: Res<'w, SelectedUnits>,
    commandable_q: Query<'w, 's, &'static mut Commandable>,
}

// Methods
impl<'w, 's> UnitCommander<'w, 's> {
    pub fn command_selection(
        &mut self,
        add_mode: bool,
        order: OrderType,
    ) {
        let unit_selection = self.unit_selection;
        if add_mode {
            for unit_id in unit_selection.selected_iter() {
                let mut commandable = self.get_mut_commandable(unit_id);
                commandable.give_order(order);
            }
        }
        else {
            for unit_id in unit_selection.selected_iter() {
                let mut commandable = self.get_mut_commandable(unit_id);
                commandable.clear_orders();
                commandable.give_order(order);
            }
        }
    }

    pub fn command_selectoion_stop(
        &mut self,
    ) {
        let unit_selection = self.unit_selection;
        for unit_id in unit_selection.selected_iter() {
            let mut commandable = self.get_mut_commandable(unit_id);
            commandable.clear_orders();
        }
    }
}

/// Internal
impl<'w, 's> UnitCommander<'w, 's> {
    /// let mut commandable = self.get_mut_commandable(unit_id);
    fn get_mut_commandable(&mut self, unit_id: &RTSUnitID) -> Mut<'_, Commandable> {
        let commandable_q = &mut self.commandable_q;
        let commandable = commandable_q.get_mut(unit_id.0);
        let commandable = commandable.unwrap();

        return commandable;
    }

    /// let commandable = self.get_commandable(unit_id);
    fn get_commandable(& self, unit_id: &RTSUnitID) -> & Commandable {
        let commandable_q = & self.commandable_q;
        let commandable = commandable_q.get(unit_id.0);
        let commandable = commandable.unwrap();

        return commandable;
    }
}