// Editor only (add cfg)
mod edit;

use bevy::prelude::*;

// A reference flag is simply a classification of a component's purpose
// It requires no implementation

// A self entity reference or "Cairn" (W.I.P. naming)
// Is simply a classifcation of a component's purpose
// It requires no implementation

/// An internal (internal to an entity heirarchy) entity reference.
pub trait Waymark {
    fn go(&self) -> Entity;
}

#[macro_export]
macro_rules! waymark { ($t: ty) => {
    impl Waymark for $t {
        fn go(&self) -> Entity {
            return self.0
        }
    }
    impl Default for $t {
        fn default() -> Self {
            return Self(Entity::PLACEHOLDER)
        }
    }
    impl $t {
        /// dest as in destination
        pub fn new(dest: Entity) -> Self {
            return Self(dest)
        }
    }
};}