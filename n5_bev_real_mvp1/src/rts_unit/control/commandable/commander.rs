use bevy::prelude::*;
use bevy::ecs::system::SystemParam;

use crate::rts_unit::*;
use super::orders::*;
use super::*;
use crate::rts_unit::control::selectable::selector::SelectedUnits;

#[derive(SystemParam)]
pub struct UnitCommander<'w, 's> {
    unit_selection: Res<'w, SelectedUnits>,
    commandable_q: Query<'w, 's, &'static mut Commandable>,
}

/// Methods
impl<'w, 's> UnitCommander<'w, 's> {
    pub fn command_selection(
        &mut self,
        add_mode: bool,
        order: OrderType,
    ) {
        let unit_selection = &self.unit_selection;
        if add_mode {
            for unit_id in unit_selection.iter() {
                let commandable = self.commandable_q.get_mut(unit_id.0);
                let mut commandable = commandable.unwrap();
                commandable.give_order(order);
            }
        }
        else {
            for unit_id in unit_selection.iter() {
                let commandable = self.commandable_q.get_mut(unit_id.0);
                let mut commandable = commandable.unwrap();
                commandable.clear_orders();
                commandable.give_order(order);
            }
        }
    }

    pub fn command_selection_stop(
        &mut self,
    ) {
        let unit_selection = &self.unit_selection;
        for unit_id in unit_selection.iter() {
            let commandable = self.commandable_q.get_mut(unit_id.0);
            let mut commandable = commandable.unwrap();
            commandable.clear_orders();
        }
    }
}