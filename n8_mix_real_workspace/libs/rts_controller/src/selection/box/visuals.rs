use bevy::prelude::*;
use mouse_pos::CursorWorldPos;

use super::*;

#[derive(Component)]
pub struct SelectionBox;

#[derive(Bundle)]
pub struct BundSelectionBox {
    pub flag: SelectionBox,
    pub sprite: SpriteBundle,
}

pub fn spawn_selection_box(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>, 
) {
    let square: Handle<Image> = asset_server.load("sprite\\primitive\\1px_square.png");

    commands.spawn(BundSelectionBox {
        flag: SelectionBox,
        sprite: SpriteBundle { 
            sprite: Sprite { color: Color::Hsla { 
                hue: 360.0, 
                saturation: 1.0, 
                lightness: 1.0, 
                alpha: 0.33 
            }, ..Default::default()}, 
            texture: square, 
            visibility: Visibility::Hidden, 
            ..Default::default()
        },
    });
}

pub fn selection_box_visuals_sys(
    mut q: Query<(&mut Transform, &mut Visibility), With<SelectionBox>>,
    origin: Res<BoxOrigin>,
    mouse_pos: Res<CursorWorldPos>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
) {
    let Ok((mut transform, mut visibilty)) = q.get_single_mut() else {
        return;
    };

    if mouse_buttons.any_pressed(BoxInput::BUTTONS) {
        // Position and scale
        const SCALE: f32 = 1.0;
        const Z: f32 = 10.0;

        let diff = mouse_pos.pos() - origin.0;
        let abs_diff = diff.abs();
        let size = abs_diff * SCALE;
        let position = origin.0 + (diff * 0.5);

        transform.translation = position.extend(Z);
        transform.scale = size.extend(1.0);
        
        // Visibility
        *visibilty = Visibility::Visible;
    }
    else {
        *visibilty = Visibility::Hidden;
    }
}