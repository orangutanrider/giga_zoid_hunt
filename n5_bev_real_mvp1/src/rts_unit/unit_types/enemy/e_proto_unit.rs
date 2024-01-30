mod ai;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use mouse_tracking::*;
use crate::rapier_config::E_NON_SOLID_CGROUP;
use crate::rts_unit::*;
use crate::rts_unit::selectable::*;
use crate::rts_unit::commandable::*;
use crate::rts_unit::commandable::orders::*;
use self::movement::BasicMover;
use self::movement::KinematicPositionBasicMoverAugment;

use super::*;

pub struct InitializePlugin;
impl Plugin for InitializePlugin {
    fn build(&self, app: &mut App) {
        println!("Initializing unit::player_types::proto_unit");
        app
        .add_systems(Startup, startup)
        .add_systems(Update, (
            prnt_orders_debug,
        ))
        .add_plugins(ai::InitializePlugin);
    }
}

fn startup(
    mut commands: Commands, 
    asset_server: Res<AssetServer>, 
){
    spawn_proto_unit(Vec3 { x: 250.0, y: 50.0, z: 0.0 } , &mut commands, &asset_server);
}

fn prnt_orders_debug(
    q: Query<& Commandable, With<ProtoUnit>>,
    keys: Res<Input<KeyCode>>,
){
    if !keys.just_pressed(KeyCode::P) {
        return;
    }

    for commandable in q.iter(){
        println!("proto unit {} has the following orders", commandable.unit.index());
        commandable.println_order_data();
    }
}

#[derive(Bundle)]
struct ProtoUnitBundle {
    proto_unit: ProtoUnit,
    enemy_team: EnemyTeam,

    //selectable: Selectable,

    // Eventually, I'd like the sprites to not be on the main body entity
    sprite_bundle: SpriteBundle,

    mover: BasicMover,
    
    rigid_body: RigidBody,
    locked_axes: LockedAxes,
    collider: Collider,
}
impl Default for ProtoUnitBundle {
    fn default() -> Self {
        Self { 
            proto_unit: ProtoUnit{}, 
            enemy_team: EnemyTeam{}, 

            //selectable: Default::default(), 

            // Eventually, I'd like the sprites to not be on the main body entity
            sprite_bundle: SpriteBundle{
                sprite: Default::default(),
                transform: Default::default(),
                global_transform: Default::default(),
                texture: Default::default(), //asset_server.load("sprite\\primitive\\64px_square.png"),
                visibility: Default::default(),
                computed_visibility: Default::default(),
            },

            mover: BasicMover::new(ProtoUnit::MOVE_SPEED),

            rigid_body: RigidBody::KinematicPositionBased, 
            locked_axes: LockedAxes::ROTATION_LOCKED, 
            collider: Collider::ball(32.0),  
        }
    }
}
impl ProtoUnit {
    const MOVE_SPEED: f32 = 1.0;
}

pub fn spawn_proto_unit(
    position: Vec3,
    commands: &mut Commands, 
    asset_server: &Res<AssetServer>, 
){
    let mut spawn = commands.spawn(
        ProtoUnitBundle {
        sprite_bundle: SpriteBundle{
            transform: Transform{
                translation: position,
                ..Default::default()
            },
            texture: asset_server.load("sprite\\primitive\\64px_square.png"), 
            ..Default::default()
        },
        ..Default::default()
    });

    let id = spawn.id();
    spawn.insert(Unit{id: UnitID(id)});
    spawn.insert(KinematicPositionBasicMoverAugment::new(id));
    spawn.insert(E_NON_SOLID_CGROUP);
    spawn.insert(Sensor);
    //spawn.insert(Commandable::new(id));
}

use bevy::{ecs::system::SystemParam, prelude::*, transform};

use super::{commandable::{self, orders::{OrderType, PureMovementOrder}, Commandable}, movement::BasicMover, ProtoUnit, Unit};

pub struct InitializePlugin;
impl Plugin for InitializePlugin {
    fn build(&self, app: &mut App) {
        println!("Initializing unit::player_units::proto_unit::ai");
        app.add_systems(Update, ai_follow_current_order);
    }
}

// AI
// Follow pure move waypoints via basic movement
// For attack target, move towards target, stop when within attack range distance
// For attack move, move towards waypoint, scan for units in range of an aggro distance, move towards units within that range, stop when within attack range of any unit

// For idle state, do not attack, I will add settings for modifying this behaviour later in development
// The plan is to have those behaviour settings be able to changed during gameplay, and set before playing too, but these options will be hidden by default, as to not overwhelm

#[derive(SystemParam)]
struct ProtoUnitAI<'w, 's> {
    commandable_q: ParamSet<'w, 's, (
        Query<'w, 's, &'static mut Commandable, With<ProtoUnit>>,
        Query<'w, 's, &'static Commandable, With<ProtoUnit>>,
    )>,
    mover_q: Query<'w, 's, &'static mut BasicMover, With<ProtoUnit>>,
    transform_q: Query<'w, 's, &'static Transform, With<ProtoUnit>>,
}

fn follow_pure_move(
    mut mover: Mut<'_, BasicMover>,
    position: Vec2,
    order: & PureMovementOrder,
) {
    let move_vec = (order.waypoint - position).normalize_or_zero();
    //println!("{}", order.waypoint);
    //println!("{}", move_vec);
    mover.input_move_vec(move_vec);
}

fn ai_follow_current_order (
    mut params: ProtoUnitAI,
) {
    for commandable in params.commandable_q.p1().iter() {
        let mover = params.mover_q.get_mut(commandable.unit);
        let mut mover = mover.unwrap();
        let transform = params.transform_q.get(commandable.unit);
        let transform = transform.unwrap();
        let position = transform.translation.truncate();
        let current_order = commandable.current_order();
        match current_order.order_type {
            OrderType::Empty => {
                mover.input_move_vec(Vec2::ZERO);
            },
            OrderType::PureMovement => {
                follow_pure_move(mover, position,&commandable.current_order_as_pure_move());
            },
            OrderType::AttackMove => {

            },
            OrderType::AttackTarget => {},
        }
    }
}