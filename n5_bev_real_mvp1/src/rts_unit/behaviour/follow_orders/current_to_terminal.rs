/// Current order to order terminal, from commandable

use bevy::prelude::*;

use crate::rts_unit::*;
use super::*;

pub struct InitializePlugin;
impl Plugin for InitializePlugin{
    fn build(&self, app: &mut App) {
        app.add_systems(Update, order_from_commandable_update);
    }
}

#[derive(Component)]
pub struct OrderFromCommandable;
impl TypeIdGet for OrderFromCommandable { }
impl EntityReferenceFlag<3, RTSUnitControl> for OrderFromCommandable {
    const REFERENCE_PATH: [TypeId; 3] = [ToRoot::TYPE_ID, RootToControl::TYPE_ID, RTSUnitControl::TYPE_ID];
    const REF_TYPE: EntityRefFlagRefType = EntityRefFlagRefType::Immutable;
}

fn order_from_commandable_update(
    mut behaviour_q: Query<(&mut TFollowedOrder, &ToRoot), With<OrderFromCommandable>>,
    root_q: Query<&RootToControl>,
    control_q: Query<&Commandable>,
) {
    for (mut terminal, to_root) in behaviour_q.iter_mut() {
        order_from_commandable(terminal, to_root, root_q, control_q);
    }
}

fn order_from_commandable(
    mut terminal: Mut<TFollowedOrder>, 
    to_root: &ToRoot,
    root_q: Query<&RootToControl>,
    control_q: Query<&Commandable>,
) {
    // Follow reference path
    let root = to_root.entity();
    let to_control = root_q.get(root);
    let Ok(to_control) = to_control else {
        OrderFromCommandable::print_err(1);
        return;
    };
    let control = to_control.entity();
    let commandable = control_q.get(control);
    let Ok(commandable) = commandable else {
        OrderFromCommandable::print_err_descript(2, "failed at getting commandable from the entity.");
        return;
    };

    // Set order
    let order = commandable.current_order();
    terminal.input(order);
}