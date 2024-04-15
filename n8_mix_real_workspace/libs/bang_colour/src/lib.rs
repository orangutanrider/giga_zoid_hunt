use bevy::prelude::*;
use behaviour_tree::prelude::*;

pub struct BangColourPlugin;

impl Plugin for BangColourPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, bang_colour_sys);
    }
}

#[derive(Component)]
pub struct BangColour{
    active: Color,
    inactive: Color
}
impl Default for BangColour {
    fn default() -> Self {
        Self { active: Color::GREEN, inactive: Color::RED }
    }
}
impl BangColour { 
    pub fn new(
        active: Color,
        inactive: Color
    ) -> Self {
        return Self{
            active,
            inactive,
        }
    }

    pub fn colour(&self, bang: bool) -> Color {
        match bang {
            true => return self.active,
            false => return self.inactive,
        }
    }
}

pub fn bang_colour_sys(
    mut q: Query<(&mut Sprite, &Bang, &BangColour), Changed<Bang>>
) {
    for (mut sprite, bang, colour) in q.iter_mut() {
        sprite.color = colour.colour(bang.is_active());
    }
}