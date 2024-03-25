pub mod commandable;
pub mod selectable;
pub mod commander;

use bevy::prelude::*;
use ref_paths::*;

#[derive(Component)]
/// Self flag
pub struct UnitControl;

#[derive(Component)]
pub struct ToUnitControl(Entity);
waymark!(ToUnitControl);
