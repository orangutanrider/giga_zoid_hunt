use bevy::prelude::*;

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