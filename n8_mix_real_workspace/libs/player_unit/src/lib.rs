pub mod test_enemy;

pub mod params; pub use params::*;
pub mod state; pub use state::*;

pub mod chase_behav; pub(crate) use self::chase_behav::*;
pub mod common; pub(crate) use self::common::*;
pub mod move_behav; pub(crate) use self::move_behav::*;
pub mod state_to_root; pub(crate) use self::state_to_root::*;
pub mod attack_behav; pub(crate) use self::attack_behav::*;
pub mod idle_behav; use attack_laser::LaserVisualsOnAttack;
use bang_colour::BangColour;
pub use idle_behav::*;


use rts_unit_control::commandable::OrderProcessedAgar;

pub(crate) use std::any::*;
pub(crate) use std::marker::*;
pub(crate) use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub(crate) use ref_caravan::*;
pub(crate) use ref_paths::*;
pub(crate) use ref_marks::*;

pub(crate) use behaviour_tree::{prelude::*, state::State as TreeState};

use rts_direct_attack::DirectAttackPower;
pub(crate) use rts_unit_control::prelude::*;
pub(crate) use rts_unit_detectors::prelude::*;
pub(crate) use rts_unit_nav::*;
use rts_unit_death::*;
use rts_unit_soul::*;
use rts_unit_team::*;
use rts_unit_health::*;
use rts_unit_movers::*;
use death_flare::*;

pub(crate) use nav_to_mover::*;
use control_to_detector::*;
use health_to_death::*;

use rapier_config::*;

use detection_colour::*;

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

            // Idle
            idle_logic_sys,
            idle_actuator_sys,

            // move
            move_aggro_logic_sys,
            move_actuator_sys,

            // chase
            chase_logic_sys,
            chase_actuator_sys,
            referenced_aggro_to_referenced_nav_sys,
            bang_to_switched_aggro_to_nav,

            // attack
            attack_behav_sys,
            target_update_sys,
            attack_timer_reset_sys,
            attack_timer_sys,
            attack_execution_sys,
            attack_end_sys,
            attack_actuator_sys,
        ));

        app.add_systems(Update,(             
            refd_mover_is_zero_when_bang_sys,
            attack_reset_sys
        ));

        app.add_plugins((
            // state_to_root
            ControlOrdersToStatePlugin,

            // move
            BMoveNavToMoverPlugin,
            BMoveControlToNavPlugin,

            // chase
            BChaseNavToMoverPlugin,
            BChaseControlToNavPlugin
        ));
    }
}


// ================================
// Unit Structure

#[derive(Component, Default)]
pub struct Root;
#[derive(Bundle, Default)]
struct BRoot {
    pub flag: Root,

    // Physics body
    pub collider: Collider,
    pub rigidbody: RigidBody,
    pub grouping: CollisionGroups,

    // Mover
    pub move_terminal: TMoveVector,
    pub move_process: LocalTransformMovement,
    pub speed: MoveSpeed,
    pub inactivity: Inactivity,

    pub team_affiliation: PlayerTeam,
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
pub struct Hub;
ref_signature!(Hub);
#[derive(Bundle, Default)]
struct BHub {
    pub flag: Hub,

    // Tree
    pub bang: Bang,
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

#[derive(Event)]
pub struct SpawnPlayerUnitEvent(pub Vec2); // spawn location 

pub fn spawn_player_unit_event_sys(
    mut event: EventReader<SpawnPlayerUnitEvent>,
    mut commands: Commands,
    asset_server: Res<AssetServer>, 
) {
    for ev in event.read() {
        spawn_player_unit(ev.0, &mut commands, &asset_server);
    }
}

pub fn spawn_player_unit(
    location: Vec2,
    commands: &mut Commands,
    asset_server: &Res<AssetServer>, 
) {
    let square: Handle<Image> = asset_server.load("sprite\\primitive\\64px_square.png");

    // Root
    let root = commands.spawn((
        BRoot{
            collider: Collider::ball(PHYSICS_SIZE),
            rigidbody: RigidBody::KinematicPositionBased,
            grouping: RTS_UNIT_PHYSICS_BODY_CGROUP,
            speed: MoveSpeed::new(MOVE_SPEED),
            ..Default::default()
        },
        SpriteBundle {
            texture: square.clone_weak(),
            transform: Transform { translation: location.extend(0.0), ..Default::default()},
            sprite: Sprite { custom_size: Some(ROOT_SIZE), color: ROOT_COLOUR,..Default::default() },
            ..Default::default()
        }
    )).id();

    // Tree Root
    let tree_root = commands.spawn((
        BTreeRoot{
            ..Default::default()
        },
        SpriteBundle {
            texture: square.clone_weak(),
            transform: Transform { translation: TREE_ROOT_OFFSET, ..Default::default()},
            sprite: Sprite { custom_size: Some(TREE_ROOT_SIZE), color: TREE_ROOT_COLOUR, ..Default::default() },
            ..Default::default()
        }
    )).id();

    // Hub
    let hub = commands.spawn((
        BHub{
            pure_move_processor: PMProximityProcessor::new(ORDER_COMPLETE_DISTANCE),
            attack_move_processor: AMProximityProcessor::new(ORDER_COMPLETE_DISTANCE),
            collider: Collider::ball(BODY_SIZE),
            rigidbody: RigidBody::Fixed,
            sensor: Sensor,
            grouping: PLAYER_SOUL_CGROUP,
            health: THealth(HEALTH),
            max_health: MaxHealth::new(HEALTH),
            to_despawn_target: ToDespawnTarget::new(root),
            regen: HealthRegeneration(HEALTH_REGEN),
            health_to_colour: HealthToColour::new(FULL_HEALTH_COLOUR, LOW_HEALTH_COLOUR),

            ..Default::default()
        },
        SpriteBundle {
            texture: square.clone_weak(),
            transform: Transform { translation: HUB_OFFSET.extend(0.0), ..Default::default()},
            sprite: Sprite { custom_size: Some(HUB_SIZE), color: FULL_HEALTH_COLOUR, ..Default::default() },
            ..Default::default()
        },
    )).id();

    // Aggro detector
    let aggro_detector = commands.spawn((
        BAggroDetection{
            detector: CircleIntersectionsOfEnemy::new(AGGRO_RANGE),
            to_root: ToBehaviourRoot::new(hub),
            detection_colour: DetectionColour::new(Color::ORANGE_RED, Color::GRAY),
            ..Default::default()
        },
        SpriteBundle {
            texture: square.clone_weak(),
            transform: Transform { translation: Vec2::new(NODES_X_OFFSET * -2.5, NODES_Y_OFFSET).extend(0.0), ..Default::default()},
            sprite: Sprite { custom_size: Some(NODES_SIZE), ..Default::default() },
            ..Default::default()
        }
    )).id();

    // Attack detector
    let attack_detector = commands.spawn((
        BAttackDetection{
            detector: CircleIntersectionsOfEnemy::new(ATTACK_RANGE),
            to_root: ToBehaviourRoot::new(hub),
            detection_colour: DetectionColour::new(Color::PURPLE, Color::GRAY),
            ..Default::default()
        },
        SpriteBundle {
            texture: square.clone_weak(),
            transform: Transform { translation: Vec2::new(NODES_X_OFFSET * -1.5, NODES_Y_OFFSET).extend(0.0), ..Default::default()},
            sprite: Sprite { custom_size: Some(NODES_SIZE), ..Default::default() },
            ..Default::default()
        }
    )).id();

    // Idle
    let idle_behav = commands.spawn((
        BIdle{
            to_root: ToBehaviourRoot::new(hub),
            to_parent: ToParentNode::new(hub),
            to_control: ToControl::new(hub),
            to_nav: ToNav::new(hub),
            to_mover: ToMover::new(root),
            bang_colour: bang_colour::BangColour::new(Color::YELLOW, Color::GRAY),
            ..Default::default()
        },
        SpriteBundle {
            texture: square.clone_weak(),
            transform: Transform { translation: Vec2::new(NODES_X_OFFSET * -0.5, NODES_Y_OFFSET).extend(0.0), ..Default::default()},
            sprite: Sprite { custom_size: Some(NODES_SIZE), ..Default::default() },
            ..Default::default()
        }
    )).id();

    // Move
    let move_behav = commands.spawn((
        BMoveB{
            to_root: ToBehaviourRoot::new(hub),
            to_parent: ToParentNode::new(hub),
            to_control: ToControl::new(hub),
            to_nav: ToNav::new(hub),
            to_mover: ToMover::new(root),
            bang_colour: bang_colour::BangColour::new(Color::LIME_GREEN, Color::GRAY),
            ..Default::default()
        },
        SpriteBundle {
            texture: square.clone_weak(),
            transform: Transform { translation: Vec2::new(NODES_X_OFFSET * 0.5, NODES_Y_OFFSET).extend(0.0), ..Default::default()},
            sprite: Sprite { custom_size: Some(NODES_SIZE), ..Default::default() },
            ..Default::default()
        }
    )).id();

    // Chase
    let chase_behav = commands.spawn((
        BChase{
            to_root: ToBehaviourRoot::new(hub),
            to_parent: ToParentNode::new(hub),
            to_control: ToControl::new(hub),
            to_nav: ToNav::new(hub),
            to_mover: ToMover::new(root),
            bang_colour: bang_colour::BangColour::new(Color::ORANGE, Color::GRAY),
            ..Default::default()
        },
        SpriteBundle {
            texture: square.clone_weak(),
            transform: Transform { translation: Vec2::new(NODES_X_OFFSET * 1.5, NODES_Y_OFFSET).extend(0.0), ..Default::default()},
            sprite: Sprite { custom_size: Some(NODES_SIZE), ..Default::default() },
            ..Default::default()
        }
    )).id();

    // Attack
    let attack_behav = commands.spawn((
        BAttack{
            to_root: ToBehaviourRoot::new(hub),
            to_parent: ToParentNode::new(hub),
            trigger: AttackTrigger::new(ATTACK_SPEED),
            end: AttackEndTrigger::new(ATTACK_ANIMATION_TIME),
            damage: DirectAttackPower::new(ATTACK_POWER),
            to_control: ToControl::new(hub),
            to_nav: ToNav::new(hub),
            to_mover: ToMover::new(root),
            bang_colour: bang_colour::BangColour::new(Color::PINK, Color::GRAY),
            attack_laser: LaserVisualsOnAttack::new(LASER_COLOUR, LASER_FADE, LASER_WIDTH),
            ..Default::default()
        },
        SpriteBundle {
            texture: square,
            transform: Transform { translation: Vec2::new(NODES_X_OFFSET * 2.5, NODES_Y_OFFSET).extend(0.0), ..Default::default()},
            sprite: Sprite { custom_size: Some(NODES_SIZE), ..Default::default() },
            ..Default::default()
        }
    )).id();

    commands.entity(root).add_child(tree_root);
    commands.entity(tree_root).add_child(hub);
    commands.entity(hub).push_children(&[aggro_detector, attack_detector, idle_behav, move_behav, chase_behav, attack_behav]);
}