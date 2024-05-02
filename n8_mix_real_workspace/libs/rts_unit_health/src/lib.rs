use bevy::prelude::*;

use ref_paths::*;

pub struct HealthPlugin;

impl Plugin for HealthPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            health_regen_sys,
            health_to_colour_sys,
        ));
    }
}

#[derive(Component)]
pub struct ToHealth(Entity);
waymark!(ToHealth);

#[derive(Component)]
pub struct THealth(pub f32);

impl Default for THealth {
    fn default() -> Self {
        Self(1.0)
    }
}

#[derive(Component)]
pub struct MaxHealth(f32);
impl Default for MaxHealth {
    fn default() -> Self {
        Self(1.0)
    }
}
impl MaxHealth {
    pub fn read(&self) -> f32 {
        return self.0;
    }

    pub fn new(v: f32) -> Self {
        return Self(v);
    }
}

#[derive(Component, Default)]
pub struct HealthRegeneration(pub f32);

pub fn health_regen_sys(
    mut q: Query<(&mut THealth, &MaxHealth, &HealthRegeneration)>,
    time: Res<Time>
) {
    for (mut health, max, regen) in q.iter_mut() {
        health.0 = (health.0 + (regen.0 * time.delta_seconds())).clamp(f32::MIN, max.0);
    }
}

#[derive(Component)]
pub struct HealthToColour{
    max: Color,
    min: Color,
}
impl Default for HealthToColour {
    fn default() -> Self {
        Self { max: Color::GREEN, min: Color::RED }
    }
}
impl HealthToColour {
    pub fn new(
        max: Color,
        min: Color,
    ) -> Self {
        return Self{
            max,
            min,
        }
    }

    pub fn read(
        &self,
        current: f32,
        max: f32,
    ) -> Color {
        let t = current / max;
        let colour_min = self.min.rgb_to_vec3();
        let colour_max = self.max.rgb_to_vec3();
        let colour = Vec3::lerp(colour_min, colour_max, t);
        return Color::rgb(colour.x, colour.y, colour.z)
    }
}

pub fn health_to_colour_sys(
    mut q: Query<(&mut Sprite, &THealth, &MaxHealth, &HealthToColour)> 
) {
    for (mut sprite, health, max_health, to_colour) in q.iter_mut() {
        let colour = to_colour.read(health.0, max_health.0);
        sprite.color = colour;
    }
}