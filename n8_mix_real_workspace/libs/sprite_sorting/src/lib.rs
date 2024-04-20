use bevy::prelude::*;

pub struct SpriteSorterPlugin;
impl Plugin for SpriteSorterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, z_sorter_sys);
    }
}

#[derive(Component, Default)]
pub struct SpriteSorter{
    pub offset: f32,
}
impl SpriteSorter {
    pub fn new(offset: f32) -> Self {
        return Self { offset }
    }
}

pub const Y_RANGE: Vec2 = Vec2::new(-500.0, 500.0);
pub const Z_RANGE: Vec2 = Vec2::new(500.0, 0.0);

pub fn z_sorter_sys(
    mut q: Query<(&mut Transform, &SpriteSorter, &GlobalTransform), With<SpriteSorter>>,
) {
    for (mut transform, root_sorter, position) in q.iter_mut() {
        let position = position.translation();
        let position = position.y + root_sorter.offset;
        let position = (position + Y_RANGE.x) / Y_RANGE.y; // Position as 0 to 1 in Y_RANGE bounds.

        let new_z = f32::lerp(Z_RANGE.x, Z_RANGE.y, position);
        transform.translation.z = new_z;
    }
}