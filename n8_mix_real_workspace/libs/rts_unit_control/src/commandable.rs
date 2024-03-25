pub mod orders;

use std::any::TypeId;

use bevy::prelude::*;

pub struct CommandablePlugin;
impl Plugin for CommandablePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, (
            clear_bang_reset_sys,
            process_bang_reset_sys
        ));
        app.add_systems(Update, active_terminal_clear_sys);
    }
}

#[derive(Component)]
pub struct Commandable;
impl Default for Commandable {
    fn default() -> Self {
        return Self;
    }
}
impl Commandable {
    pub fn new() -> Self{
        return Self;
    }
}

#[derive(Component)]
/// Stores the stack of local order terminals (as types), that have orders.
/// The currently active terminal, and following terminals, can be inferred through this.
pub struct ActiveOrderTerminal(Vec<TypeId>);
impl Default for ActiveOrderTerminal {
    fn default() -> Self {
        return Self(Vec::new());
    }
}
impl ActiveOrderTerminal {
    // It is, in a literal sense, the same blueprint as normal order terminals.
    // Except, it contains TypeID.
    // Despite that, I didn't decide to use the trait here.

    pub fn new() -> Self {
        return Self(Vec::new());
    }

    pub fn clear(&mut self) {
        self.0.clear();
    }
    pub fn clear_current(&mut self) {
        self.0.pop();
    }
    pub fn command(&mut self, terminal: TypeId) {
        self.0.push(terminal);
    }

    pub fn current(&self) -> Option<TypeId> {
        let index = self.0.len().wrapping_sub(1);
        return self.0.get(index).copied()
    }
    pub fn count(&self) -> usize {
        return self.0.len()
    }
    pub fn iter(&self) -> core::slice::Iter<'_, TypeId> {
        return self.0.iter()
    }
}

pub fn active_terminal_clear_sys(
    mut control_q: Query<&mut ActiveOrderTerminal, Changed<ClearOrdersBang>>,
) {
    for mut order_types in control_q.iter_mut() {
        order_types.clear();
    }
}

#[derive(Component)]
/// Send the signal the entity's order terminals, to clear
pub struct ClearOrdersBang(bool);
impl Default for ClearOrdersBang {
    fn default() -> Self {
        return Self(false)
    }
}
impl ClearOrdersBang {
    pub fn new() -> Self{
        return Self(false)
    }

    pub fn bang(&mut self) {
        self.0 = true;
    }
}

/// Pre-Update
fn clear_bang_reset_sys(
    mut control_q: Query<&mut ClearOrdersBang, Changed<ClearOrdersBang>>
) {
    for mut bang in control_q.iter_mut() {
        bang.bypass_change_detection();
        bang.0 = false;
    }
}

#[derive(Component)]
/// Send the signal the entity's order terminals, to process their current order.
/// If it is that their order is current, (inferred through ActiveOrderTerminal).
pub struct ProcessCurrentOrderBang(bool);
impl Default for ProcessCurrentOrderBang {
    fn default() -> Self {
        return Self(false)
    }
}
impl ProcessCurrentOrderBang {
    pub fn new() -> Self{
        return Self(false)
    }

    pub fn bang(&mut self) {
        self.0 = true;
    }
}

/// Pre-Update
fn process_bang_reset_sys(
    mut control_q: Query<&mut ProcessCurrentOrderBang, Changed<ProcessCurrentOrderBang>>
) {
    for mut bang in control_q.iter_mut() {
        bang.bypass_change_detection();
        bang.0 = false;
    }
}