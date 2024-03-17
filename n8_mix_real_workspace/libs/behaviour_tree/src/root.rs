use bevy::prelude::*;

#[derive(Component)]
pub(crate) struct RootBang {
    // signal to reset and send propogation wave for reference export
    bang_update: bool, 
}
impl Default for RootBang {
    fn default() -> Self {
        return Self::new()
    }
}
impl RootBang { //! Constructor
    pub fn new() -> Self {
        return Self {
            bang_update: false,
        }
    }
}

impl RootBang { //! Set
    pub fn update(&mut self) {
        self.bang_update = true;
    }
}