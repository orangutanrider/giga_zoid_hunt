use bevy::{ecs::system::SystemParam, prelude::*};
use mouse_pos::CursorWorldPos;

use rts_unit_control::prelude::*;

use crate::{add_mode::AddModeInput, rapier::PhysicsQueries};

#[derive(Resource, Default)]
pub struct BoxOrigin(Vec2); 

#[derive(SystemParam)]
pub struct BoxInput<'w> {
    mouse_buttons: Res<'w, ButtonInput<MouseButton>>,
    add_mode: AddModeInput<'w>,
    mouse_pos: Res<'w, CursorWorldPos>,
}
impl<'w> BoxInput<'w> {
    const BUTTONS: [MouseButton; 1] = [MouseButton::Left];

    pub fn just_pressed(&self) -> bool {
        return self.mouse_buttons.any_just_pressed(Self::BUTTONS);
    }
    
    pub fn just_released(&self) -> bool {
        return self.mouse_buttons.any_just_released(Self::BUTTONS);
    }

    pub fn add_mode(&self) -> bool {
        return self.add_mode.is_pressed()
    }

    pub fn pos(&self) -> Vec2 {
        return self.mouse_pos.pos();
    }
}

/// Save the selection box origin 
pub fn box_origin_sys(
    input: BoxInput,
    mut origin: ResMut<BoxOrigin>,
) {
    if !input.just_pressed() {
        return;
    }
    origin.0 = input.pos();
}

pub fn box_selection_sys(
    input: BoxInput,
    origin: Res<BoxOrigin>,
    rapier: PhysicsQueries,
    mut commands: Commands,
    selected_q: Query<Entity, With<Selected>>,
    //selectable_q: Query<Entity, With<Selectable>>,
) {
    if !input.just_released() {
        return;
    }
    if input.add_mode() {
        un_select_all(&mut commands, &selected_q);
    }

    let callback = |entity: Entity| -> bool {
        //if !selectable_q.contains(entity) {
        //    return false; 
        //}
        // This isn't needed, cause the physics query already filters it.

        let entity_commands = commands.entity(entity);
        select(entity_commands); 
        return true;
    };
    rapier.cast_for_p_selectable(origin.0, input.pos(), callback)
}