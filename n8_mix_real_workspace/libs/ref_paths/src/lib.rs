// Editor only (add cfg)
mod edit;

use bevy::prelude::*;


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