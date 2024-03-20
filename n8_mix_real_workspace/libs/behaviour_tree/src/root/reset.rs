use bevy::prelude::*;

#[derive(Component)]
/// Signal to reset and send propogation wave for reference export
pub struct ResetBang(bool);
impl Default for ResetBang {
    fn default() -> Self {
        return Self::new()
    }
}
impl ResetBang {
    pub fn new() -> Self {
        return Self (false)
    }

    pub fn is_active(&self) -> bool {
        return self.0
    }

    pub fn bang(&mut self) {
        self.0 = true;
    }
}

/// A component, that can be used with the reset_behaviour_sys system.
/// The system will ping the component, when the reset signal has been recieved.
pub trait ResetBehaviour: Component {
    fn go(&mut self);
}

/// Prefab system for reset_behaviour traited components
/// It will call .go() on a behaviour tree reset/update
pub fn reset_behaviour_sys<R: ResetBehaviour>(
    mut root_q: Query<(&ResetBang, &mut R), Changed<ResetBang>>
) {
    for (reset, mut export) in root_q.iter_mut() {
        if !reset.is_active() {
            continue;
        }

        export.go();
    }
}