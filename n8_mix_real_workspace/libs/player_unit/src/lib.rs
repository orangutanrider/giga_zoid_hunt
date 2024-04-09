pub(crate) mod chase_behav; pub(crate) use self::chase_behav::*;
pub(crate) mod common; pub(crate) use self::common::*;
pub(crate) mod move_behav; pub(crate) use self::move_behav::*;
pub(crate) mod state_to_root; pub(crate) use self::state_to_root::*;
pub(crate) mod attack_behav; pub(crate) use self::attack_behav::*;

pub(crate) use std::any::*;
pub(crate) use std::marker::*;
pub(crate) use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub(crate) use ref_caravan::*;
pub(crate) use ref_paths::*;
pub(crate) use ref_marks::*;

pub(crate) use behaviour_tree::{prelude::*, state::State as TreeState};

pub(crate) use rts_unit_control::prelude::*;
pub(crate) use rts_unit_detectors::prelude::*;
pub(crate) use rts_unit_nav::*;
use rts_unit_death::*;
use rts_unit_soul::*;
use rts_unit_team::*;
use rts_unit_health::*;
use rts_unit_movers::*;

pub(crate) use nav_to_mover::*;
use control_to_detector::*;
use health_to_death::*;

// Note:
// There are reference definitions in this that could be upgraded to be more flexible.

pub struct PlayerUnitPlugin;

impl Plugin for PlayerUnitPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update,(
            // state_to_root
            imca_mapper_sys,
            aggro_to_tree_root_sys,
            attack_closest_to_tree_root_sys,
            attack_target_to_tree_root_sys,
            detection_to_state_sys, 

            // move
            move_aggro_logic_sys,
            move_actuator_sys,

            // chase
            chase_logic_sys,
            chase_actuator_sys,
            referenced_aggro_to_referenced_nav_sys,

            // attack
            target_update_sys,
            attack_timer_reset_sys,
            attack_timer_sys,
            attack_execution_sys,
            attack_end_sys,
            attack_actuator_sys,
        ));

        app.add_plugins((
            // state_to_root
            ControlOrdersToStatePlugin,

            // move
            BMoveNavToMoverPlugin,
            BMoveControlToNavPlugin,

            // chase
            BChaseNavToMoverPlugin,

            // common
            BangToSwitchedMoveAsNavPlugin,
            BangToSwitchedControlAsNavPlugin,
        ));
    }
}


// ================================
// Unit Structure

#[derive(Component)]
struct Hub;
ref_signature!(Hub);
#[derive(Bundle)]
struct BHub {
    pub flag: Hub,

    // Tree
    pub tree_bang: RootBang,
    //pub reset_bang: ResetBang,
    //pub export_bang: ExportBang,
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
    pub orders: ActiveOrderTerminal,
    pub clear: ClearOrdersBang,
    pub pure_move_orders: TPureMoveOrders,
    pub attack_move_orders: TAttackMoveOrders,
    pub target_orders: TAttackTargetOrders,
    pub current_target: CurrentTarget,
    pub target_processor: UntilTargetGoneProcessor,
    pub pure_move_processor: PMProximityProcessor,
    pub attack_move_processor: AMProximityProcessor,

    // Nav
    pub nav_terminal: TNavWaypoint,
    pub nav_process: DirectNav,
    pub nav_output: NavVectorOutput,

    // Body/Soul
    pub collider: Collider,
    pub rigidbody: RigidBody,
    pub sensor: Sensor,
    pub grouping: CollisionGroups,

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
}

#[derive(Component)]
struct Root;
#[derive(Bundle)]
struct BRoot {
    pub flag: Root,

    // Physics body
    pub collider: Collider,
    pub rigidbody: RigidBody,
    pub grouping: CollisionGroups,

    // Mover
    pub move_terminal: TMoveVector,
    pub move_process: LocalTransformMovement,
}

#[derive(Component)]
struct AggroDetection;
#[derive(Bundle)]
struct BAggroDetection {
    pub flag: AggroDetection,

    pub aggregate: TIntersectionsAggregate,
    pub closest: DistillationForClosest,
    pub detector: CircleIntersectionsOfEnemy,
}

#[derive(Component)]
struct AttackDetection;
#[derive(Bundle)]
struct BAttackDetection {
    pub flag: AttackDetection,

    pub aggregate: TIntersectionsAggregate,
    pub closest: DistillationForClosest,
    pub targeted: DistillationForTarget,
    pub detector: CircleIntersectionsOfEnemy,

    pub target: TDetectionTarget,
    pub target_is_local: TargetIsLocal<Hub>,
    pub target_as_control: TargetAsCurrentInControl<Hub>,
    pub control_is_reference: ControlIsReference<Hub>,
    pub to_control: ToControl,
}