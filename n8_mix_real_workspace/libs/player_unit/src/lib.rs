#[cfg(debug_assertions)]
pub mod test_enemy;

pub mod public; pub use public::*;

pub(crate) mod params;          pub(crate) use params::*;
pub(crate) mod state;           pub(crate) use state::*;

pub(crate) mod chase_behav;     pub(crate) use chase_behav::*;
pub(crate) mod common;          pub(crate) use common::*;
pub(crate) mod move_behav;      pub(crate) use move_behav::*;
pub(crate) mod state_to_root;   pub(crate) use state_to_root::*;
pub(crate) mod attack_behav;    pub(crate) use attack_behav::*;
pub(crate) mod idle_behav;      pub(crate) use idle_behav::*; 

pub(crate) use std::any::*;
pub(crate) use std::marker::*;
pub(crate) use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub(crate) use ref_caravan::*;
pub(crate) use ref_paths::*;
pub(crate) use ref_marks::*;

pub(crate) use behaviour_tree::{
    prelude::*, 
    state::State as TreeState
};
use rapier_config::*;

pub(crate) use rts_unit_control::prelude::*;
pub(crate) use rts_unit_detectors::prelude::*;
pub(crate) use rts_unit_nav::*;
pub(crate) use sprite_sorting::*;
use rts_direct_attack::*;
use rts_unit_death::*;
use rts_unit_soul::*;
use rts_unit_team::*;
use rts_unit_health::*;
use rts_unit_movers::*;
use death_flare::*;
use detection_colour::*;
use attack_laser::*;
use selection_visuals::*;

use control_to_detector::*;
use health_to_death::*;

#[derive(Component, Default)]
pub(crate) struct Root;
#[derive(Bundle, Default)]
struct BRoot {
    pub flag: Root,

    // Physics body
    pub collider: Collider,
    pub rigidbody: RigidBody,
    pub grouping: CollisionGroups,
    pub locking: LockedAxes,
    pub velocity: Velocity,

    // Mover
    pub move_terminal: TMoveVector,
    pub move_process: LocalVelocityMovement,
    pub speed: MoveSpeed,
    pub inactivity: Inactivity,
    pub to_health: ToHealth,

    pub team_affiliation: PlayerTeam,

    // Z sorting
    pub sorter: SpriteSorter,
}

#[derive(Component, Default)]
struct TreeRoot;
ref_signature!(TreeRoot);
#[derive(Bundle, Default)]
struct BTreeRoot {
    pub flag: TreeRoot,
    pub tree_bang: RootBang,
    pub reset_bang: ResetBang,
    pub export_bang: ExportBang,

    pub team_affiliation: PlayerTeam,
}

#[derive(Component, Default)]
pub(crate) struct Hub;
ref_signature!(Hub);
#[derive(Bundle, Default)]
struct BHub {
    pub flag: Hub,

    // Tree
    pub bang: Bang,
    pub state: TState,

    // Behaviour
    pub imca_mapper: TUnitIMCAMapper,
    pub aggro_close: AggroDetectorClosest,
    pub attack_close: AttackDetectorClosest,
    pub attack_targeted: AttackDetectorTargeted,
    pub control_to_state: ControlOrdersToState,
    pub detection_to_state: DetectionToState,

    // Control
    pub selectable: Selectable,
    pub commandable: Commandable,
    pub orders: TActiveOrderType,
    pub clear: ClearOrdersBang,
    pub pure_move_orders: TPureMoveOrders,
    pub attack_move_orders: TAttackMoveOrders,
    pub target_orders: TAttackTargetOrders,
    pub current_target: TCurrentTarget,
    pub target_processor: UntilTargetGoneProcessor,
    pub pure_move_processor: PMProximityProcessor,
    pub attack_move_processor: AMProximityProcessor,
    pub agar: OrderProcessedAgar, 

    pub to_selected_motif: ToSelectionMotif,

    // Nav
    pub nav_terminal: TNavWaypoint,
    pub nav_process: DirectNav,
    pub nav_output: NavVectorOutput,

    // Body/Soul
    pub collider: Collider,
    pub rigidbody: RigidBody,
    pub sensor: Sensor,
    pub grouping: CollisionGroups,

    pub at_here_targeting: TargetedBy,

    // Mortality
    pub health: THealth,
    pub max_health: MaxHealth,
    pub health_to_death: ZeroHealthMeansDeath,
    pub death_is_local: DeathIsLocal,
    pub health_is_local: HealthIsLocal,
    pub death: DeathBang,
    pub death_to_despawn: DeathToEntityDespawn,
    pub despawn_is_ref: DespawnTargetIsReference,
    pub to_despawn_target: ToDespawnTarget,
    pub team_affiliation: PlayerTeam,
    pub death_flare: DeathFlareOnDeath,
    pub regen: HealthRegeneration,

    pub health_to_colour: HealthToColour,
}

#[derive(Component, Default)]
struct AggroDetection;
#[derive(Bundle)]
#[derive(Default)]
struct BAggroDetection {
    pub flag: AggroDetection,

    pub to_root: ToBehaviourRoot,

    pub aggregate: TIntersectionsAggregate,
    pub closest: DistillationForClosest,
    pub detector: CircleIntersectionsOfEnemy,

    pub detection_colour: DetectionColour,

    pub team_affiliation: PlayerTeam,
}

#[derive(Component, Default)]
struct AttackDetection;
#[derive(Bundle, Default)]
struct BAttackDetection {
    pub flag: AttackDetection,

    pub to_root: ToBehaviourRoot,

    pub aggregate: TIntersectionsAggregate,
    pub closest: DistillationForClosest,
    pub targeted: DistillationForTarget,
    pub detector: CircleIntersectionsOfEnemy,

    pub target: TDetectionTarget,
    pub target_is_local: TargetIsLocal<Hub>,
    pub target_as_control: TargetAsCurrentInControl<Hub>,
    pub control_is_reference: ControlIsReference<Hub>,
    pub to_control: ToControl,

    pub detection_colour: DetectionColour,

    pub team_affiliation: PlayerTeam,
}
