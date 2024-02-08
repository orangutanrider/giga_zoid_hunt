pub mod p_proto_unit;
pub mod prince;
pub mod prelude;

use bevy::prelude::*;

pub struct InitializePlugin;
impl Plugin for InitializePlugin{
    fn build(&self, app: &mut App) {
        app.add_plugins(p_proto_unit::InitializePlugin);
    }
}