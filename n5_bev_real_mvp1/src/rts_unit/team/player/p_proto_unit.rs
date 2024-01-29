use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
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
    spawn_proto_unit(&mut commands, &asset_server);
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
    player_team: PlayerTeam,

    selectable: Selectable,

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
            player_team: PlayerTeam{}, 

            selectable: Default::default(), 

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
    const ATTACK_RANGE: f32 = 100.0;
    const MOVE_SPEED: f32 = 1.0;
}

pub fn spawn_proto_unit(
    commands: &mut Commands, 
    asset_server: &Res<AssetServer>, 
){
    let mut spawn = commands.spawn(
        ProtoUnitBundle {
        sprite_bundle: SpriteBundle{
            texture: asset_server.load("sprite\\primitive\\64px_square.png"), 
            ..Default::default()
        },
        ..Default::default()
    });

    let id = spawn.id();
    spawn.insert(Unit{id: UnitID(id)});
    spawn.insert(KinematicPositionBasicMoverAugment::new(id));
    spawn.insert(Commandable::new(id));
}

use bevy::{ecs::system::SystemParam, prelude::*, transform};
use bevy_rapier2d::{geometry::{Collider, Toi}, plugin::RapierContext, rapier::pipeline::QueryFilter};

use super::{commandable::{self, orders::{AttackMoveOrder, AttackTargetOrder, OrderType, PureMovementOrder}, Commandable}, movement::BasicMover, ProtoUnit, Unit};

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
    rapier_context: Res<'w, RapierContext>,

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
    mover.input_move_vec(move_vec);
}

use crate::rapier_groups::*;
fn follow_attack_move(
    rapier_context: & Res<RapierContext>,
    mut mover: Mut<'_, BasicMover>,
    position: Vec2,
    order: & AttackMoveOrder,
) {
    let attack_range_cast = rapier_context.cast_shape(
        position, 
        0.0, 
        Vec2::ZERO, 
        &Collider::ball(ProtoUnit::ATTACK_RANGE), 
        0.0, 
        E_NON_SOLID_FILTER
    );

    if attack_range_cast.is_none() {
        let move_vec = (order.waypoint - position).normalize_or_zero();
        mover.input_move_vec(move_vec);
    }
    else {
        mover.input_move_vec(Vec2::ZERO);
    }
}

fn follow_attack_target(
    transform_q: & Query<&Transform>,
    collider_q: & Query<&Collider>,
    mut mover: Mut<'_, BasicMover>,
    position: Vec2,
    order: & AttackTargetOrder,
) {
    let target = order.target_unit;
    let collider = collider_q.get(target);
    let collider = collider.unwrap();
    let collider = collider.as_ball().unwrap();
    let enemy_radius = collider.radius();

    let transform = transform_q.get(target);
    let transform = transform.unwrap();
    let target_position = transform.translation.truncate();

    let distance = position.distance(target_position) - enemy_radius;

    if distance <= ProtoUnit::ATTACK_RANGE {
        mover.input_move_vec(Vec2::ZERO);
    }
    else {
        let move_vec = (target_position - position).normalize_or_zero();
        mover.input_move_vec(move_vec);
    }
}

fn ai_follow_current_order (
    mut params: ProtoUnitAI,
    transform_q: Query<&Transform>,
    collider_q: Query<&Collider>,
) {
    for commandable in params.commandable_q.p1().iter() {
        let mover = params.mover_q.get_mut(commandable.unit);
        let mut mover = mover.unwrap();

        let transform = params.transform_q.get(commandable.unit);
        let transform = transform.unwrap();
        let position = transform.translation.truncate();

        let rapier_context = & params.rapier_context;

        let current_order = commandable.current_order();
        match current_order.order_type {
            OrderType::Empty => {
                mover.input_move_vec(Vec2::ZERO);
            },
            OrderType::PureMovement => {
                follow_pure_move(mover, position, &commandable.current_order_as_pure_move());
            },
            OrderType::AttackMove => {
                follow_attack_move(& rapier_context, mover, position, &commandable.current_order_as_attack_move());
            },
            OrderType::AttackTarget => {
                follow_attack_target(&transform_q, &collider_q, mover, position, &commandable.current_order_as_attack_target());
            },
        }
    }
}