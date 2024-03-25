use std::marker::PhantomData;

use bevy::prelude::*;
use bevy::ecs::system::SystemParam;

use self::commandable::orders::TUnitOrder;
use self::commandable::Commandable;
use self::selectable::Selected;

use super::*;

#[derive(SystemParam)]
pub struct SelectionCommands<'w, 's, Terminal, Order: 'static>
where 
    Terminal: Component + TUnitOrder<Order>,
    Order: Copy,
{
    control_q: Query<'w, 's, &'static mut Terminal, (With<Selected>, With<Commandable>)>,
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
        for mut commandable in self.control_q.iter_mut() {
            commandable.command(*order);
        }
    }

    pub fn set_command(&mut self, order: &Order) {
        for mut commandable in self.control_q.iter_mut() {
            commandable.clear();
            commandable.command(*order);
        }
    }

    pub fn command_stop(
        &mut self,
    ) {
        for mut commandable in self.control_q.iter_mut() {
            commandable.clear();
        }
    }
}