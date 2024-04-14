pub mod prelude;

pub mod commandable;
pub mod selectable;
pub mod commander;

use bevy::prelude::*;
use ref_paths::*;

use commandable::*;
use commandable::orders::{
    *,
    pure_move::{*, processing::*},
    attack_move::{*, processing::*},
    attack_target::{*, processing::*},
};
use selectable::*;

pub struct ControlPlugin;
impl Plugin for ControlPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            CommandablePlugin,
            BuiltInOrdersPlugin
        ));
    }
}

#[derive(Default)]
#[derive(Bundle)]
pub struct ControlCoreBundle {
    pub flag: Control,
    pub transform: TransformBundle,

    pub selectable: Selectable,
    pub commandable: Commandable,

    pub orders: TActiveOrderType,
    pub clear: ClearOrdersBang,
}

#[derive(Bundle)]
pub struct ControlBundle {
    pub core: ControlCoreBundle,
    
    pub pure_move_orders: TPureMoveOrders,
    pub attack_move_orders: TAttackMoveOrders,
    pub target_orders: TAttackTargetOrders,

    pub current_target: TCurrentTarget,

    pub target_processor: UntilTargetGoneProcessor,
    pub pure_move_processor: PMProximityProcessor,
    pub attack_move_processor: AMProximityProcessor,
}

#[derive(Component)]
/// Self flag
pub struct Control;
impl Default for Control {
    fn default() -> Self {
        Self {  }
    }
}

#[derive(Component)]
pub struct ToControl(Entity);
waymark!(ToControl);
