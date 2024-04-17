pub(crate) use behaviour_tree::{prelude::*, state::State as TreeState};

pub const IN_AGGRO: TreeState = TreeState::N8;
pub const IN_ATTACK: TreeState = TreeState::N9;
pub const IN_ATTACK_RANGE: TreeState = TreeState::N9.union(IN_AGGRO);

pub const PURE_MOVE: TreeState = TreeState::N4;
pub const ATTACK_MOVE: TreeState = TreeState::N5;
pub const ATTACK_TARGET: TreeState = TreeState::N6;

pub const MOVE: TreeState = TreeState::N1;
pub const CHASE: TreeState = TreeState::N2;
pub const ATTACK: TreeState = TreeState::N3;
pub const IDLE: TreeState = TreeState::N7; 