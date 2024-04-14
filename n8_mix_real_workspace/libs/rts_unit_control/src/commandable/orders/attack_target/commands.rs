use bevy::ecs::system::SystemParam;
use bevy::prelude::*;
use crate::commandable::orders::{
    *,
    attack_target::*,
};
use crate::selectable::*;

#[derive(SystemParam)]
pub struct SelectionAttackTargetCommands<'w, 's> {
    q: Query<'w, 's, (Entity, &'static mut TAttackTargetOrders, &'static mut TActiveOrderType), (With<Selected>, With<Commandable>)>,
    target_q: Query<'w, 's, &'static mut TargetedBy>,
}

impl<'w, 's> SelectionAttackTargetCommands<'w, 's> {
    pub fn command(
        &mut self,
        add_mode: bool,
        order: &AttackTargetOrder,
    ) {
        match add_mode {
            true => self.add_command(order),
            false => self.set_command(order),
        }
    }

    pub fn add_command(&mut self, order: &AttackTargetOrder) {
        for (targeter, mut data_terminal, mut type_terminal) in self.q.iter_mut() {
            insert_targeter(targeter, order.target, &mut self.target_q);

            data_terminal.command(*order);
            type_terminal.command(TypeId::of::<TAttackTargetOrders>());
        }
    }

    pub fn set_command(&mut self, order: &AttackTargetOrder) {
        for (targeter, mut data_terminal, mut type_terminal) in self.q.iter_mut() {
            insert_targeter(targeter, order.target, &mut self.target_q);

            data_terminal.clear();
            type_terminal.clear();
            data_terminal.command(*order);
            type_terminal.command(TypeId::of::<TAttackTargetOrders>());
        }
    }

    pub fn local_clear(
        &mut self,
    ) {
        for (_, mut data_terminal, mut type_terminal) in self.q.iter_mut() {
            data_terminal.clear();
            type_terminal.clear();
        }
    }
}

fn insert_targeter(
    targeter: Entity,
    target: Entity,
    target_q: &mut Query<&mut TargetedBy>,
) {
    if let Ok(mut targeted_by) = target_q.get_mut(target) {
        targeted_by.insert_targeter(targeter);
    } else {
        // commands.entity(target).insert(TargetedBy::new_insert(targeter));
        // You can't use commands to insert it here, because they are executed late, so only one of the units will add themselves to the TargetedBy component.
        // Cause they overwrite eachother when inserting.
        error!("Target was not ready to be targeted, it did not have a TargetedBy component.");
    }
}


/* 
fn clear_target_of_targeter( 
    targeter: &Entity,
    target: Entity,
    target_q: &mut Query<&mut TargetedBy>,
) {
    if let Ok(mut targeted_by) = target_q.get_mut(target) {
        targeted_by.remove_targeter(targeter);
    } 
}
*/