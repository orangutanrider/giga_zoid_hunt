use bevy::prelude::*;
use bevy::ecs::system::SystemParam;

use super::RtsSelectorContext;
use crate::rts_controller::mouse::RtsMouse;
use crate::rts_controller::rapier_queries::RtsControllerRapierQueries;
use crate::rts_unit::control::RTSUnitControlID;
use crate::rts_unit::ToRTSUnitRoot;

pub struct InitializePlugin;
impl Plugin for InitializePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SelectionBoxOrigin>();
        app.add_systems(Update, (
            selection_box,
            record_select_input_origin,
        ));
    }
}

#[derive(SystemParam)]
pub struct SelectionBoxInput<'w> {
    mouse_buttons: Res<'w, Input<MouseButton>>,
}
impl<'w> SelectionBoxInput<'w> {
    const BUTTONS: [MouseButton; 1] = [MouseButton::Left];

    pub fn just_pressed(&self) -> bool {
        return self.mouse_buttons.any_just_pressed(Self::BUTTONS);
    }
    
    pub fn just_released(&self) -> bool {
        return self.mouse_buttons.any_just_released(Self::BUTTONS);
    }
}

#[derive(Resource, Default)]
/// Records and stores the mouse world position on SelectionBoxInput.just_pressed()
struct SelectionBoxOrigin(Vec2); 

fn record_select_input_origin(
    select_input: SelectionBoxInput,
    mouse: RtsMouse,
    mut origin_keeper: ResMut<SelectionBoxOrigin>,
) {
    if !select_input.just_pressed() {
        return;
    }
    origin_keeper.0 = mouse.position();
}

/// Update system
fn selection_box(
    select_input: SelectionBoxInput,
    input_origin: Res<SelectionBoxOrigin>,
    mut selector_context: RtsSelectorContext,
    rapier_queries: RtsControllerRapierQueries,
){
    if !select_input.just_released() {
        return;
    }
    let add_mode = selector_context.add_mode_input.is_pressed();
    
    let selector = &mut selector_context.unit_selector;
    selector.select_nothing(add_mode);

    let origin = input_origin.0;
    let release = selector_context.mouse.position();

    let callback = |entity: Entity| -> bool {
        selector.select_unit(add_mode, RTSUnitControlID::new(entity));
        return true;
    };
    rapier_queries.cast_for_p_selectable(origin, release, callback);
}