use bevy::prelude::*;

pub trait EditWaymark {
    
}

enum WaymarkKind {
    /// To anywhere internal, can cross branches (use sparingly).
    Direct(Direct), 
    /// To anywhere below, in the heirarchy tree.
    Down(Down), 
    /// To anywhere above, in the heirarchy tree.
    Up(Up), 
}
/// To anywhere internal, can cross branches (use sparingly).
struct Direct;
/// To anywhere below, in the heirarchy tree.
struct Down;
/// To anywhere above, in the heirarchy tree.
struct Up;