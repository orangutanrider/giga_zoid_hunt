mod terminal;
mod output;

use std::any::TypeId;
use bevy::prelude::*;

/// A bit mask identifying behaviour state flags.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct State(u32);
impl Default for State {
    fn default() -> Self {
        State::ALL
    }
}

/// Identification types for anything trying to input state into a state terminal.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Key{
    ExternalEntity(Entity),
    LocalComponent(TypeId)
}

bitflags::bitflags! {
    impl State: u32 {
        const N1 = 1 << 0;
        const N2 = 1 << 1;
        const N3 = 1 << 2;
        const N4 = 1 << 3;
        const N5 = 1 << 4;
        const N6 = 1 << 5;
        const N7 = 1 << 6;
        const N8 = 1 << 7;
        const N9 = 1 << 8;
        const N10 = 1 << 9;
        const N11 = 1 << 10;
        const N12 = 1 << 11;
        const N13 = 1 << 12;
        const N14 = 1 << 13;
        const N15 = 1 << 14;
        const N16 = 1 << 15;
        const N17 = 1 << 16;
        const N18 = 1 << 17;
        const N19 = 1 << 18;
        const N20 = 1 << 19;
        const N21 = 1 << 20;
        const N22 = 1 << 21;
        const N23 = 1 << 22;
        const N24 = 1 << 23;
        const N25 = 1 << 24;
        const N26 = 1 << 25;
        const N27 = 1 << 26;
        const N28 = 1 << 27;
        const N29 = 1 << 28;
        const N30 = 1 << 29;
        const N31 = 1 << 30;
        const N32 = 1 << 31;

        const ALL = u32::MAX;
        const NONE = 0;
    }
}