/// Navigation = Waypoint to movement vector.

pub mod direct_nav;

use std::any::TypeId;

use bevy::{prelude::*, utils::HashMap};
use ref_paths::*;

pub use direct_nav::*;

pub struct NavPlugin;

impl Plugin for NavPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, direct_nav_sys);
    }
}

#[derive(Component)]
/// Cairn.
pub struct Nav;

#[derive(Component)]
/// Waymark.
pub struct ToNav(Entity);
waymark!(ToNav);

#[derive(Component)]
/// Input
pub struct TNavWaypoint(pub Vec2);
impl Default for TNavWaypoint {
    fn default() -> Self {
        Self(Vec2::ZERO)
    }
}

#[derive(Component)]
/// Output
pub struct NavVectorOutput(pub Vec2);
impl Default for NavVectorOutput {
    fn default() -> Self {
        Self(Vec2::ZERO)
    }
}