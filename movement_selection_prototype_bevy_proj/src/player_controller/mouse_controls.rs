use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use mouse_tracking::MousePosWorld;

pub struct InitializePlugin;
impl Plugin for InitializePlugin {
    fn build(&self, app: &mut App) {
        println!("");
        println!("Initializing mouse_controls.rs");
        app
           .add_systems(Update, update);
    }
}

fn update(
    rapier_context: Res<RapierContext>,
    mouse_world: Res<MousePosWorld>,
    buttons: Res<Input<MouseButton>>,
){
    selection_single_click(rapier_context, mouse_world, buttons);
}

// once box selection is added, there needs to be a thing for deciding whether to do this or box selection.
// in the prototype I did that by storing the position on mouse down and then comparing it to the position on mouse up
// if the distance is too low, I'd do a single click selection
fn selection_single_click(
    rapier_context: Res<RapierContext>,
    mouse_world: Res<MousePosWorld>,
    buttons: Res<Input<MouseButton>>,
){
    if !buttons.just_pressed(MouseButton::Left){
        return;
    }

    if let Some((entity, toi)) = cast_single_click(rapier_context, mouse_world.truncate()) {
        println!("HIT"); // this doesn't actually do any selection yet
    }
}

fn cast_single_click(
    rapier_context: Res<RapierContext>,
    cast_position: Vec2,
) -> Option<(Entity, Toi)> {
    rapier_context.cast_shape
    (cast_position, 0.0, Vec2::ZERO, &Collider::ball(5.0), 0.0, QueryFilter{..default()})
}