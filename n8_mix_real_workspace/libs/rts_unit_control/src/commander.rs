pub mod grid_formation;
use grid_formation::XYIter;

use std::any::TypeId;
use std::marker::PhantomData;

use bevy::prelude::*;
use bevy::ecs::system::SystemParam;

use super::*;

const FORMATION_SCALE: Vec2 = Vec2::new(30.0, 20.0);

pub struct FormationIter{
    origin: Vec2,
    scale: Vec2,
    iter: XYIter,
}
impl FormationIter {
    pub fn new(scale: Vec2, origin: Vec2) -> Self {
        return Self {
            origin,
            scale,
            iter: XYIter::new()
        }
    }

    pub fn next(&mut self) -> Vec2 {
        let grid_pos = self.iter.next().as_vec2();
        let scaled = grid_pos * self.scale;
        let from_origin = self.origin + scaled;
        return from_origin;
    }
}

pub trait WaypointOrder {
    fn waypoint(&self) -> Vec2;
    fn from_waypoint(waypoint: Vec2) -> Self;
}

#[derive(SystemParam)]
pub struct SpiralCommander<'w, 's, Terminal, Order: 'static>
where 
    Terminal: Component + TUnitOrder<Order>,
    Order: Copy + WaypointOrder,
{
    control_q: Query<'w, 's, (&'static mut Terminal, &'static mut TActiveOrderType), (With<Selected>, With<Commandable>)>,
    phantom: PhantomData<Order> // This is here to satisfy the Order generic parameter
}
impl<'w, 's, Terminal, Order> SpiralCommander<'w, 's, Terminal, Order> 
where 
    Terminal: Component + TUnitOrder<Order>,
    Order: Copy + WaypointOrder,
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
        let mut formation_iter = FormationIter::new(FORMATION_SCALE, order.waypoint());

        for (mut data_terminal, mut type_terminal) in self.control_q.iter_mut() {
            let waypoint = formation_iter.next();
            let order = Order::from_waypoint(waypoint);

            data_terminal.command(order);
            type_terminal.command(TypeId::of::<Terminal>());
        }
    }

    pub fn set_command(&mut self, order: &Order) {
        let mut formation_iter = FormationIter::new(FORMATION_SCALE, order.waypoint());

        for (mut data_terminal, mut type_terminal) in self.control_q.iter_mut() {
            let waypoint = formation_iter.next();
            let order = Order::from_waypoint(waypoint);

            data_terminal.clear();
            type_terminal.clear();
            data_terminal.command(order);
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