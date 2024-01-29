use bevy::prelude::*;
use bevy::ecs::system::SystemParam;
use mouse_tracking::MousePosWorld;

use super::unit_selection::SelectInput;

pub struct InitializePlugin;
impl Plugin for InitializePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SelectBoxOrigin>();
        app.add_systems(Update, record_select_just_pressed_origin);
    }
}

#[derive(SystemParam)]
pub struct RtsMouse<'w> {
    origin: Res<'w, SelectBoxOrigin>,
    position: Res<'w, MousePosWorld>,
}
impl<'w> RtsMouse<'w> {
    pub fn position(&self) -> Vec2 {
        return self.position.truncate();
    }

    pub fn select_box_origin(&self) -> Vec2 {
        return self.origin.0.truncate();
    }
}

#[derive(Resource, Default)]
/// Records and stores the mouse world position on SelectInput.just_pressed()
struct SelectBoxOrigin(Vec3); 

fn record_select_just_pressed_origin(
    select_input: SelectInput,
    mouse_world: Res<MousePosWorld>,
    mut origin_keeper: ResMut<SelectBoxOrigin>,
) {
    if !select_input.just_pressed() {
        return;
    }
    origin_keeper.0 = **mouse_world;
}