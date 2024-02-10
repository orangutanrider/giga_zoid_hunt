pub mod circle_intersections;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Component)]
pub struct AdditionalDetectorFilter(Group);
impl Default for AdditionalDetectorFilter {
    fn default() -> Self {
        Self(Group::NONE)
    }
}
impl AdditionalDetectorFilter {
    pub fn new(group: Group) -> Self {
        return Self(group)
    }

    pub fn group(&self) -> Group {
        return self.0
    }
}