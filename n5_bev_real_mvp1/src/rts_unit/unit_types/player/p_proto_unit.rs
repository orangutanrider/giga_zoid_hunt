use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{rapier_config::prelude::PRINCE_SOUL_CGROUP, rts_unit::{
    behaviour::{
        detection::{
            circle_cast_detector::CircleCastUnitDetector, single_result_types::{
                arbitrary_unit::ArbitraryUnitDetection,
                closest_unit::ClosestUnitDetection,
                target_unit::{
                    target_from_commandable::TargetFromCommandable, TargetUnitDetection
                },
            }, to_detection::attack_detection::*
        }, navigation::controlled::basic::BasicControlled, order_processing::r#move::basic_completer::BasicMoveOrderCompleter, RTSUnitBehaviourEntity
    }, control::{
        commandable::Commandable, selectable::Selectable, RTSUnitControlEntity, RTSUnitControlID
    }, movement::{
        kinematic_position_movement::KinematicPositionMovement, Mover
    }, soul::RTSUnitSoulEntity, unit_types::RTSTeam::Player, *
}};

use crate::rapier_config::prelude::{
    P_CONTROL_CGROUP,
    P_SOUL_CGROUP,
    RTS_PHYSICS_CGROUP,
};

use super::prince::Prince;

pub struct InitializePlugin;
impl Plugin for InitializePlugin{
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn);
    }
}

#[derive(Component)]
struct PProtoUnit;
const ATTACKABLE_SIZE: f32 = 10.0;
const SELECTABLE_SIZE: f32 = 10.0;
const RANGE: f32 = 100.0;
const MOVE_SPEED: f32 = 1.0;

#[derive(Bundle)]
struct RTSRoot{
    p_proto_unit: PProtoUnit,
    prince: Prince,

    rts_unit: RTSUnit,
    control: RTSUnitControlEntity,
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
struct Control {
    to_root: ToRTSUnitRoot,

    commandable: Commandable,
    selectable: Selectable,
    move_order_completer: BasicMoveOrderCompleter,

    transform: TransformBundle,
    collider: Collider, // Selectable
    sensor: Sensor,
    c_group: CollisionGroups,
}

#[derive(Bundle)]
struct Behaviour {
    to_root: ToRTSUnitRoot,

    controlled_navigation: BasicControlled,
    to_attack_target: ToAttackTargetDetection,
    to_attack_arbitrary: ToAttackArbitraryDetection,

    transform: TransformBundle,
}

#[derive(Bundle)]
struct AttackDetection {
    to_root: ToRTSUnitRoot,

    detector: CircleCastUnitDetector,
    arbitrary_detection: ArbitraryUnitDetection,
    closest_detection: ClosestUnitDetection,
    target_detection: TargetUnitDetection,
    target_from: TargetFromCommandable,

    transform: TransformBundle,
}


fn spawn(
    mut commands: Commands, 
    asset_server: Res<AssetServer>, 
){
    let root = commands.spawn_empty().id();

    let control = commands.spawn_empty().id();
    let soul = commands.spawn_empty().id();
    let behaviour = commands.spawn_empty().id();
    let attack_detection = commands.spawn_empty().id();

    commands.entity(root).insert(RTSRoot{
        rts_unit: RTSUnit::new(root),
        control: RTSUnitControlEntity::new(control),
        behaviour: RTSUnitBehaviourEntity::new(behaviour),
        soul: RTSUnitSoulEntity::new(soul),

        mover: Mover::new(MOVE_SPEED),
        movement: KinematicPositionMovement::new(),
        rb2d: RigidBody::KinematicPositionBased,
        velocity: Velocity::default(),
        transform: TransformBundle::default(),
        c_group: RTS_PHYSICS_CGROUP,
        p_proto_unit: PProtoUnit,
        prince: Prince,
    });

    commands.entity(control).insert(Control{
        to_root: ToRTSUnitRoot::new(root),

        commandable: Commandable::new(),
        selectable: Selectable::new(),
        move_order_completer: BasicMoveOrderCompleter,

        transform: TransformBundle::default(),
        collider: Collider::cuboid(SELECTABLE_SIZE, SELECTABLE_SIZE),
        sensor: Sensor,
        c_group: P_CONTROL_CGROUP,
    });

    commands.entity(soul).insert(Soul{
        to_root: ToRTSUnitRoot::new(root),

        transform: TransformBundle::default(),
        collider: Collider::ball(ATTACKABLE_SIZE),
        sensor: Sensor,
        c_group: PRINCE_SOUL_CGROUP,
    });

    commands.entity(behaviour).insert(Behaviour{
        to_root: ToRTSUnitRoot::new(root),

        controlled_navigation: BasicControlled,
        to_attack_target: ToAttackTargetDetection::new(attack_detection),
        to_attack_arbitrary: ToAttackArbitraryDetection::new(attack_detection),

        transform: TransformBundle::default(),
    });

    commands.entity(attack_detection).insert(AttackDetection{
        to_root: ToRTSUnitRoot::new(root),

        detector: CircleCastUnitDetector::new(RANGE, unit_types::RTSTeam::Enemy),
        arbitrary_detection: ArbitraryUnitDetection::new(),
        closest_detection: ClosestUnitDetection::new(),
        target_detection: TargetUnitDetection::new(),
        target_from: TargetFromCommandable::new(RTSUnitControlID::new(control)),

        transform: TransformBundle::default(),
    });

    // Create parent child heirarchy
    commands.entity(root).push_children(&[control, soul, behaviour]);
    commands.entity(behaviour).push_children(&[attack_detection]);
}