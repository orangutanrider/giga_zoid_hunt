use bevy::prelude::*;
use bevy::ecs::system::SystemParam;

use crate::rts_unit::*;
use super::Selectable;

pub struct InitializePlugin;
impl Plugin for InitializePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SelectedUnits>();
    }
}

#[derive(Resource, Default)]
/// Resource that stores the selected units, publicly it can be read but not edited, to edit use unit selector
pub struct SelectedUnits {
    selected: Vec<RTSUnitID>,
}
impl SelectedUnits {
    pub fn iter(&self) -> std::slice::Iter<'_, RTSUnitID> {
        return self.selected.iter();
    }

    fn clear(&mut self) {
        self.selected.clear();
    }
}

#[derive(SystemParam)]
/// System param that provides methods for selecting units
pub struct UnitSelector<'w, 's>{
    unit_selection: ResMut<'w, SelectedUnits>,
    selectable_q: ParamSet<'w, 's, (
        Query<'w, 's, &'static mut Selectable>,
        Query<'w, 's, &'static Selectable>,
    )>
}

/// Methods
impl<'w, 's> UnitSelector<'w, 's> {
    pub fn select_nothing(
        &mut self,
        add_mode: bool,
    ) {
        if !add_mode {
            self.clear_selection();
        }
    }

    pub fn select_unit(
        &mut self, 
        add_mode: bool,
        unit_id: RTSUnitID
    ) {
        let selected = &mut self.unit_selection;

        if !add_mode {
            selected.clear();
        }
        
        // Get selectable
        let q = &mut self.selectable_q.p0();
        let selectable = q.get_mut(unit_id.0);
        let mut selectable = selectable.unwrap();

        // Don't add already selected units to selection
        if selectable.is_selected == true {
            return;
        }

        // Add to selection
        let selectable = selectable.as_mut();
        selectable.is_selected = true;
        selected.selected.push(unit_id);
    }

    pub fn clear_selection(&mut self) {
        // Clear flags
        let q = &mut self.selectable_q.p0();
        for unit_id in self.unit_selection.iter() {
            let selectable = q.get_mut(unit_id.0);
            let mut selectable = selectable.unwrap();
            selectable.is_selected = false;
        }

        // Remove from selection
        self.unit_selection.clear();
    }
}