// Editor only (add cfg)
mod edit;

use bevy::prelude::*;

/// An internal (internal to an entity heirarchy) entity reference.
pub trait Waymark {
    fn go(&self) -> Entity;
}

macro_rules! waymark {
    ($t: ty) => {
        impl Waymark for $t {
            fn go(&self) -> Entity {
                return self.0
            }
        }
    };
}