use bevy::prelude::*;
use player_unit::Root;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        //app.add_systems(Update, (
        //    //prnt_units_sys
        //));
    }
}

fn prnt_units_sys(
    q: Query<&Root>,
) {
    let mut e = 0;
    for unit in q.iter() {
        e = e + 1;
    }

    println!("{}", e);
}