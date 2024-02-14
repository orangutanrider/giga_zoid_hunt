pub mod current_to_terminal;
pub mod basic_attack_move;
pub mod basic_attack_target;
pub mod basic_pure_move;
pub mod basic_no_order;

use bevy::prelude::*;

use crate::rts_unit::control::parts::*;

#[derive(Component)]
pub struct TFollowedOrder {
    order: Option<RTSUnitOrder>,
}
impl Default for TFollowedOrder {
    fn default() -> Self {
        Self { 
            order: None,
    }}
}
impl TFollowedOrder {
    pub fn new() -> Self {
        return Self { 
            order: None,
    }}
}

impl TFollowedOrder {
    pub fn read(&self) -> Option<RTSUnitOrder>{
        return self.order
    }
    
    pub fn input(&mut self, order: Option<RTSUnitOrder>) {
        self.order = order;
    }
}