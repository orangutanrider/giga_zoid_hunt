use bevy::prelude::*;

#[derive(Component)]
pub struct THealth(pub f32);

#[derive(Component)]
pub struct MaxHealth(f32);
impl MaxHealth {
    pub fn read(&self) -> f32 {
        return self.0;
    }

    pub fn new(v: f32) -> Self {
        return Self(v);
    }
}

