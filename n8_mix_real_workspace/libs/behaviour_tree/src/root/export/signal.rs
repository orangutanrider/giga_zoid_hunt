use bevy::prelude::*;

use super::*;

#[derive(Component)]
/// Signal to export bang values to references
pub struct ExportBang(bool);
impl Default for ExportBang {
    fn default() -> Self {
        return Self::new()
    }
}
impl ExportBang { 
    pub fn new() -> Self {
        return Self(false)
    }

    pub fn is_active(&self) -> bool {
        return self.0
    }

    pub fn bang(&mut self) {
        self.0 = true;
    }
}

#[derive(Component)]
/// Upon recieving the reset signal, it will start counting.
/// When the count reaches the internal value, it will signal the export bang, to signal to export.
/// 
/// Usage: The internal count, is usually set to the longest branch in the tree.
/// 
/// Effect: Upon recieving the reset signal, exports will recieve their value from the tree, and then one export will happen.
/// Upon change in the tree, after the delay, the tree is exported all at once.
pub struct ExportWhenCount{
    active: bool,
    when_count_eq: u32,
    count: u32,
}
impl ExportWhenCount {
    pub fn new(when_count_eq: u32) -> Self {
        Self {
            active: false,
            when_count_eq,
            count: 0,
        }
    }

    /// Increments, and returns true, if the count has been reached.
    pub fn count(&mut self) -> bool {
        self.count = self.count + 1;
        if self.count >= self.when_count_eq {
            self.count = 0;
            self.active = false;
            return true;
        }
        return false;
    }
}
impl ResetBehaviour for ExportWhenCount {
    fn go(&mut self) {
        self.count = 0;
        self.active = true;
    }
}

/// When a ExportWhenCount component is set to active, this system will start incrementing its count every frame.
/// Once the count ends, it'll activate the export bang.
pub fn export_when_count_sys(
    mut root_q: Query<(&mut ExportBang, &mut ExportWhenCount), Changed<ExportWhenCount>>,
) {
    for (mut bang, mut when_c) in root_q.iter_mut() {
        if !when_c.active {
            continue;
        }

        if !when_c.count() { // If count hasn't ended
            continue;
        }
        // If count has ended
        bang.bang();
    }
}

#[derive(Component)]
/// Upon recieving the reset signal, it will start counting.
/// For each count (until the count reaches the internal value) it will signal the export bang, to signal to export.
/// 
/// Usage: The internal count, is usually set to the longest branch in the tree.
/// 
/// Effect: Upon recieving the reset signal, all exports will export inactive, and then the tree will export the bangs, as the propagation wave travels across it.
/// Upon change in the tree, all are reset, and then exported in steps, causing a flicker of inactivty.
pub struct ExportForCount{
    active: bool,
    for_count_eq: u32,
    count: u32,
}
impl ExportForCount {
    pub fn new(for_count_eq: u32) -> Self {
        Self {
            active: false,
            for_count_eq,
            count: 0,
        }
    }

    /// Increments, returning true, until the count has been surpassed.
    pub fn count(&mut self) -> bool {
        self.count = self.count + 1;
        if self.count > self.for_count_eq {
            self.count = 0;
            self.active = false;
            return false;
        }
        return true;
    }
}
impl ResetBehaviour for ExportForCount {
    fn go(&mut self) {
        self.count = 0;
        self.active = true;
    }
}

/// When a ExportForCount component is set to active, this system will start incrementing its count every frame.
/// It'll signal the export bang every every time, until the count ends.
pub fn export_for_count_sys(
    mut root_q: Query<(&mut ExportBang, &mut ExportWhenCount), Changed<ExportWhenCount>>,
) {
    for (mut bang, mut for_c) in root_q.iter_mut() {
        if !for_c.active {
            continue;
        }

        bang.bang();
        for_c.count();
    }
}