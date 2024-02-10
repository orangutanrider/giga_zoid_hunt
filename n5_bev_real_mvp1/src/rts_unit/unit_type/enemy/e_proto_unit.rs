use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{rapier_config::prelude::P_PRINCE, rts_unit::{
    behaviour::{
        detection::{
            circle_cast_detector::CircleCastUnitDetector, single_result_types::{
                arbitrary_unit::ArbitraryUnitDetection,
                closest_unit::ClosestUnitDetection,
                target_unit::{
                    target_from_commandable::TargetFromCommandable, TargetUnitDetection
                },
            }, to_detection::attack_detection::*
        },  
        RTSUnitBehaviourEntity
    }, control::{
    }, movement::{
        kinematic_position_movement::KinematicPositionMovement, Mover
    }, soul::RTSUnitSoulEntity, unit_type::RTSTeam::Player, *
}};

use crate::rapier_config::prelude::{
    E_SOUL_CGROUP,
    RTS_PHYSICS_CGROUP,
};

use self::behaviour::{detection::detector_filter::AdditionalDetectorFilter, navigation::autonomous::hunt_prince::HuntPrinceAutonomous};

pub struct InitializePlugin;
impl Plugin for InitializePlugin{
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn);
    }
}

#[derive(Component)]
struct EProtoUnit;
const ATTACKABLE_SIZE: f32 = 10.0;
const SELECTABLE_SIZE: f32 = 10.0;
const RANGE: f32 = 100.0;
const MOVE_SPEED: f32 = 0.5;

#[derive(Bundle)]
struct RTSRoot{
    rts_unit: RTSUnit,
    behaviour: RTSUnitBehaviourEntity,
    soul: RTSUnitSoulEntity,

    mover: Mover,
    movement: KinematicPositionMovement,
    rb2d: RigidBody,
    velocity: Velocity,
    transform: TransformBundle,
    c_group: CollisionGroups,
}

#[derive(Bundle)]
struct Soul {
    to_root: ToRTSUnitRoot,

    transform: TransformBundle,
    collider: Collider, // Attackable, Detectable
    sensor: Sensor,
    c_group: CollisionGroups,
}

#[derive(Bundle)]
struct Behaviour {
    to_root: ToRTSUnitRoot,

    autonomous_navigation: HuntPrinceAutonomous,
    to_attack_arbitrary: ToAttackArbitraryDetection,

    transform: TransformBundle,
}

#[derive(Bundle)]
struct AttackDetection {
    to_root: ToRTSUnitRoot,

    detector: CircleCastUnitDetector,
    detector_filter: AdditionalDetectorFilter,
    arbitrary_detection: ArbitraryUnitDetection,

    transform: TransformBundle,
}

fn spawn(
    mut commands: Commands, 
    asset_server: Res<AssetServer>, 
){
    let root = commands.spawn_empty().id();

    let soul = commands.spawn_empty().id();
    let behaviour = commands.spawn_empty().id();
    let attack_detection = commands.spawn_empty().id();

    commands.entity(root).insert(RTSRoot{
        rts_unit: RTSUnit::new(root),
        behaviour: RTSUnitBehaviourEntity::new(behaviour),
        soul: RTSUnitSoulEntity::new(soul),

        mover: Mover::new(MOVE_SPEED),
        movement: KinematicPositionMovement::new(),
        rb2d: RigidBody::KinematicPositionBased,
        velocity: Velocity::default(),
        transform: TransformBundle::default(),
        c_group: RTS_PHYSICS_CGROUP,
    });

    commands.entity(soul).insert(Soul{
        to_root: ToRTSUnitRoot::new(root),

        transform: TransformBundle::default(),
        collider: Collider::ball(ATTACKABLE_SIZE),
        sensor: Sensor,
        c_group: E_SOUL_CGROUP,
    });

    commands.entity(behaviour).insert(Behaviour{
        to_root: ToRTSUnitRoot::new(root),

        autonomous_navigation: HuntPrinceAutonomous,
        to_attack_arbitrary: ToAttackArbitraryDetection::new(attack_detection),

        transform: TransformBundle::default(),
    });

    commands.entity(attack_detection).insert(AttackDetection{
        to_root: ToRTSUnitRoot::new(root),

        detector_filter: AdditionalDetectorFilter::new(P_PRINCE),
        detector: CircleCastUnitDetector::new(RANGE, Player),
        arbitrary_detection: ArbitraryUnitDetection::new(),

        transform: TransformBundle::default(),
    });

    // Create parent child heirarchy
    commands.entity(root).push_children(&[soul, behaviour]);
    commands.entity(behaviour).push_children(&[attack_detection]);
}