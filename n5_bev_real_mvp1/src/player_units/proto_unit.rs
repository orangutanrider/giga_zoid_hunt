use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use mouse_tracking::*;
use crate::unit::*;
use crate::unit::selectable::*;
use crate::unit::commandable::*;
use crate::unit::commandable::orders::*;
use super::*;

pub struct InitializePlugin;
impl Plugin for InitializePlugin {
    fn build(&self, app: &mut App) {
        println!("Initializing unit::player_types::proto_unit");
        app
        .add_systems(Startup, startup)
        .add_systems(Update, (
            prnt_orders_debug,
        ));
    }
}

fn startup(
    mut commands: Commands, 
    asset_server: Res<AssetServer>, 
    mut unit_q: Query<&mut Unit>,
){
    spawn_proto_unit(&mut commands, &asset_server, &mut unit_q);
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
    commandable: Commandable,

    sprite_bundle: SpriteBundle,

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
            commandable: Default::default(), 

            sprite_bundle: SpriteBundle{
                sprite: Default::default(),
                transform: Default::default(),
                global_transform: Default::default(),
                texture: Default::default(), //asset_server.load("sprite\\primitive\\64px_square.png"),
                visibility: Default::default(),
                computed_visibility: Default::default(),
            },

            rigid_body: RigidBody::KinematicPositionBased, 
            locked_axes: LockedAxes::ROTATION_LOCKED, 
            collider: Collider::ball(32.0),  
        }
    }
}

pub fn spawn_proto_unit(
    commands: &mut Commands, 
    asset_server: &Res<AssetServer>, 
    unit_q: &mut Query<&mut Unit>,
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
}