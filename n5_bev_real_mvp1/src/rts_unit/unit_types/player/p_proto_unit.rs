use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::rts_unit::{
    *,
    control::{
        RTSUnitControlEntity,
        selectable::Selectable,
        commandable::Commandable,
    },
    behaviour::{
        RTSUnitBehaviourEntity,
        navigation::controlled::basic_controlled_navigation::BasicControlled,
        order_processing::r#move::basic_completer::BasicMoveOrderCompleter, // This should maybe be in the control section
        detection::{
            circle_cast_detector::CircleCastUnitDetector,
            single_result_types::arbitrary_unit::ArbitraryUnitDetection,
            single_result_types::closest_unit::ClosestUnitDetection,
            single_result_types::target_unit::TargetUnitDetection,
        }
    },
    soul::RTSUnitSoulEntity,
    unit_types::RtsTeam::Player,
    movement::{
        Mover,
        kinematic_position_movement::KinematicPositionMovement,
    }
};

use crate::rapier_config::prelude::{
    P_CONTROL_CGROUP,
    P_SOUL_CGROUP,
    RTS_PHYSICS_CGROUP,
};

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
const MOVE_SPEED: f32 = 10.0;

#[derive(Bundle)]
struct RTSRoot{
    rts_unit: RTSUnit,
    control: RTSUnitControlEntity,
    behaviour: RTSUnitBehaviourEntity,
    soul: RTSUnitSoulEntity,

    mover: Mover,
    movement: KinematicPositionMovement,
    transform: Transform,
    rigidbody: RigidBody,
    c_group: CollisionGroups,
}

#[derive(Bundle)]
struct Soul {
    to_root: ToRTSUnitRoot,

    transform: Transform,
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

    transform: Transform,
    collider: Collider, // Selectable
    sensor: Sensor,
    c_group: CollisionGroups,
}

#[derive(Bundle)]
struct Behaviour {
    to_root: ToRTSUnitRoot,

    controlled_navigation: BasicControlled,

    transform: Transform,
}

#[derive(Bundle)]
struct Detection {
    to_root: ToRTSUnitRoot,
    
    detector: CircleCastUnitDetector,
    arbitrary_detection: ArbitraryUnitDetection,
    closest_detection: ClosestUnitDetection,
    target_detection: TargetUnitDetection,

    transform: Transform,
}

fn spawn(
    mut commands: Commands, 
    asset_server: Res<AssetServer>, 
){
    let root = commands.spawn_empty().id();

    let control = commands.spawn_empty().id();
    let soul = commands.spawn_empty().id();
    let behaviour = commands.spawn_empty().id();
    let detection = commands.spawn_empty().id();

    // Create parent child heirarchy
    commands.entity(root).push_children(&[control, soul, behaviour]);
    commands.entity(behaviour).push_children(&[detection]); // Hmm, I have to ask myself "Should detection be associated with behaviour?" the answer is no, I don't think it should

    commands.entity(root).insert(RTSRoot{
        rts_unit: RTSUnit::new(root),
        control: RTSUnitControlEntity::new(control),
        behaviour: RTSUnitBehaviourEntity::new(behaviour),
        soul: RTSUnitSoulEntity::new(soul),

        mover: Mover::new(MOVE_SPEED),
        movement: KinematicPositionMovement::new(),
        transform: Transform::default(),
        rigidbody: RigidBody::KinematicPositionBased,
        c_group: RTS_PHYSICS_CGROUP,
    });

    commands.entity(control).insert(Control{
        to_root: ToRTSUnitRoot::new(root),

        commandable: Commandable::new(),
        selectable: Selectable::new(),
        move_order_completer: BasicMoveOrderCompleter,

        transform: Transform::default(),
        collider: Collider::cuboid(SELECTABLE_SIZE, SELECTABLE_SIZE),
        sensor: Sensor,
        c_group: P_CONTROL_CGROUP,
    });

    commands.entity(soul).insert(Soul{
        to_root: ToRTSUnitRoot::new(root),

        transform: Transform::default(),
        collider: Collider::ball(ATTACKABLE_SIZE),
        sensor: Sensor,
        c_group: P_SOUL_CGROUP,
    });

    commands.entity(behaviour).insert(Behaviour{
        to_root: ToRTSUnitRoot::new(root),

        controlled_navigation: BasicControlled,

        transform: Transform::default(),
    });

    commands.entity(detection).insert(Detection {
        to_root: ToRTSUnitRoot::new(root),

        detector: CircleCastUnitDetector::new(RANGE, Player),
        arbitrary_detection: ArbitraryUnitDetection::new(),
        closest_detection: ClosestUnitDetection::new(),
        target_detection: TargetUnitDetection::new(),

        transform: Transform::default(),
    });
}