use std::any::TypeId;
use std::marker::PhantomData;

use bevy::prelude::*;
use bevy::ecs::system::SystemParam;

use super::*;

#[derive(SystemParam)]
pub struct SelectionCommands<'w, 's, Terminal, Order: 'static>
where 
    Terminal: Component + TUnitOrder<Order>,
    Order: Copy,
{
    control_q: Query<'w, 's, (&'static mut Terminal, &'static mut TActiveOrderType), (With<Selected>, With<Commandable>)>,
    phantom: PhantomData<Order> // This is here to satisfy the Order generic parameter
}
impl<'w, 's, Terminal, Order> SelectionCommands<'w, 's, Terminal, Order> 
where 
    Terminal: Component + TUnitOrder<Order>,
    Order: Copy,
{
    pub fn command(
        &mut self,
        add_mode: bool,
        order: &Order,
    ) {
        match add_mode {
            true => self.add_command(order),
            false => self.set_command(order),
        }
    }

    pub fn add_command(&mut self, order: &Order) {
        for (mut data_terminal, mut type_terminal) in self.control_q.iter_mut() {
            data_terminal.command(*order);
            type_terminal.command(TypeId::of::<Terminal>());
        }
    }

    pub fn set_command(&mut self, order: &Order) {
        for (mut data_terminal, mut type_terminal) in self.control_q.iter_mut() {
            data_terminal.clear();
            type_terminal.clear();
            data_terminal.command(*order);
            type_terminal.command(TypeId::of::<Terminal>());
        }
    }

    pub fn local_clear(
        &mut self,
    ) {
        for (mut data_terminal, mut type_terminal) in self.control_q.iter_mut() {
            data_terminal.clear();
            type_terminal.clear();
        }
    }
}