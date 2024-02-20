use bevy::prelude::*;

use crate::rts_unit::{
    *,
    movement::*,
};
use super::*;

pub struct InitializePlugin;
impl Plugin for InitializePlugin{
    fn build(&self, app: &mut App) {
        app.add_systems(Update, basic_pure_move_update);
    }
}

#[derive(Component)]
struct NoOrderToMover;
impl TypeIdGet for NoOrderToMover { }
impl EntityReferenceFlag<2, RTSUnitRoot> for NoOrderToMover {
    const REFERENCE_PATH: [TypeId; 2] = [ToRoot::TYPE_ID, RTSUnitRoot::TYPE_ID];
    const REF_TYPE: EntityRefFlagRefType = EntityRefFlagRefType::Mutable;
}

#[derive(Component)]
pub struct BasicPureMoveOrderBehaviour;

fn basic_pure_move_update(
    behaviour_q: Query<(&TFollowedOrder, &ToRoot), (With<BasicPureMoveOrderBehaviour>, With<NoOrderToMover>)>,
    mut root_q: Query<&mut TMover>,
) {
    for (terminal, to_root) in behaviour_q.iter() {
        basic_pure_move(&mut root_q, terminal, to_root);
    }
}

fn basic_pure_move(
    root_q: &mut Query<&mut TMover>,
    terminal: &TFollowedOrder, 
    to_root: &ToRoot, 
) {
    let order = terminal.read();
    if order.is_some() { 
        return;
    }

    stop_moving(root_q, to_root);
}

fn stop_moving(
    root_q: &mut Query<&mut TMover>,
    to_root: &ToRoot,
) {    
    // Follow reference path
    let root = to_root.entity();
    let mover = root_q.get_mut(root);
    let Ok(mut mover) = mover else {
        NoOrderToMover::print_err_descript(1, "failed at getting TMover from the entity.");
        return;
    };

    // Stop moving
    mover.input(Vec2::ZERO);
}