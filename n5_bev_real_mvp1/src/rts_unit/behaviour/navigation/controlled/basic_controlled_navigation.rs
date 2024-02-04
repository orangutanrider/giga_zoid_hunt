use bevy::prelude::*;

use crate::rts_unit::control::prelude::*;
use crate::rts_unit::{
    movement::Mover,
    RTSUnitSubEntity
};

#[derive(Component)]
struct BasicControlled;

fn behaviour_update (
    mut behaviour_q: Query<&RTSUnitSubEntity, With<BasicControlled>>,
    mut control_q: Query<&mut Commandable>,
    mut root_q: Query<&mut Mover>,
) {
    for sub_entity in behaviour_q.iter_mut() {
        let root = sub_entity.root();
    }
}

fn follow_pure_move_order() {
    
}

fn follow_attack_move_order() {
    
}

fn follow_attack_target_order() {

}
