use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use super::*;
use crate::gameplay_controller::rapier_mouse::*;

use mouse_tracking::MousePosWorld;

pub struct InitializePlugin;
impl Plugin for InitializePlugin {
    fn build(&self, app: &mut App) {
        println!("Initializing gameplay_controller::selection::mouse");
        app
        .add_systems(Update, selection_box)
        .add_systems(Startup, spawn_selection_box);
    }
}

#[derive(Component)]
pub struct SelectionBox{
    pub origin: Vec2,
}

/// Startup
fn spawn_selection_box(mut commands: Commands){
    commands.spawn(SelectionBox{origin: Vec2::ZERO});
}

/// Update
fn selection_box(
    rapier: Res<RapierContext>,
    mouse_world: Res<MousePosWorld>,
    buttons: Res<Input<MouseButton>>,
    mut selection: ResMut<SelectionContext>,
    mut box_q: Query<&mut SelectionBox>,
    unit_q: Query<&mut Unit, With<Selectable>>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        box_q.single_mut().origin = mouse_world.truncate(); // Store origin
        return;
    }

    if !buttons.just_released(MouseButton::Left) {
        return;
    } // If button has been released

    let origin = box_q.single_mut().origin;
    let end_point = mouse_world.truncate();

    selection.mark_selection_input();

    let aabb = vec2s_to_aabb(origin, end_point);
    let callback = |entity| {
        let unit = unit_q.get(entity);

        if unit.is_err(){ // Unit was not gotten
            return false;
        }
        
        // Select Unit
        let unit = unit.unwrap();
        selection.add_select(unit);

        return true;
    };

    multi_aabb_intersections(rapier, aabb, callback);
}