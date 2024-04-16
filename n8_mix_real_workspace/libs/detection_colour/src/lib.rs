use bevy::prelude::*;
use rts_unit_detectors::prelude::*;

pub struct DetectionColourPlugin;

impl Plugin for DetectionColourPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, detection_colour_sys);
    }
}

#[derive(Component)]
pub struct DetectionColour{
    active: Color,
    inactive: Color
}
impl Default for DetectionColour {
    fn default() -> Self {
        Self { active: Color::GREEN, inactive: Color::RED }
    }
}
impl DetectionColour { 
    pub fn new(
        active: Color,
        inactive: Color
    ) -> Self {
        return Self{
            active,
            inactive,
        }
    }

    pub fn colour(&self, is_detection: bool) -> Color {
        match is_detection {
            true => return self.active,
            false => return self.inactive,
        }
    }
}

pub fn detection_colour_sys(
    mut q: Query<(&mut Sprite, &TIntersectionsAggregate, &DetectionColour), Changed<TIntersectionsAggregate>>
) {
    for (mut sprite, aggregate, colour) in q.iter_mut() {
        sprite.color = colour.colour(!(aggregate.0.len() == 0));
    }
}