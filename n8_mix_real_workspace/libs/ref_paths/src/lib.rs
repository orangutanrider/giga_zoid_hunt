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

// Editor only (add cfg)
pub trait EditWaymark {
    
}

// Editor only (add cfg)
enum WaymarkKind {
    /// To anywhere internal, can cross branches (use sparingly).
    Direct(Direct), 
    /// To anywhere below, in the heirarchy tree.
    Down(Down), 
    /// To anywhere above, in the heirarchy tree.
    Up(Up), 
}
// Editor only (add cfg)
/// To anywhere internal, can cross branches (use sparingly).
struct Direct;
// Editor only (add cfg)
/// To anywhere below, in the heirarchy tree.
struct Down;
// Editor only (add cfg)
/// To anywhere above, in the heirarchy tree.
struct Up;