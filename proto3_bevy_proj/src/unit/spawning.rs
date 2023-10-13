use std::collections::VecDeque;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use super::*;
use super::movement::*;

#[derive(Component)]
pub struct UnitSpawnManager{
    spawn_requests: Vec<UnitSpawnRequest>,
}

#[derive(Clone, Copy)]
pub struct UnitSpawnRequest{
    pub spawn_location: Vec3,
}

#[derive(Bundle)]
struct UnitBundle{
    unit: Unit,
    unit_movement: UnitMovement,

    sprite_bundle: SpriteBundle,

    // Physics
    rigid_body: RigidBody,
    locked_axes: LockedAxes,
    collider: Collider,
}

pub struct InitializePlugin;
impl Plugin for InitializePlugin {
    fn build(&self, app: &mut App) {
        println!("");
        println!("Initializing unit_spawning.rs");
        app
           .add_systems(PreStartup, spawn_unit_spawn_manager)
           .add_systems(PostUpdate, process_spawn_requests);
    }
}

// Startup
fn spawn_unit_spawn_manager(mut commands: Commands){
    commands.spawn(UnitSpawnManager{
        spawn_requests: Vec::new(),
    });
}

// Callback Processing
fn process_spawn_requests(
    mut commands: Commands, 
    asset_server: Res<AssetServer>, 
    mut manager_q: Query<&mut UnitSpawnManager>,
    mut unit_q: Query<&mut Unit>,
) {
    let mut manager = manager_q.single_mut();
    if manager.spawn_requests.len() == 0{
        return;
    }

    for spawn_request in manager.spawn_requests.iter_mut(){
        spawn_unit_internal(&mut commands, &asset_server, *spawn_request);
    }
    
    manager.spawn_requests.clear();
}

// Internal
fn spawn_unit_internal(
    commands: &mut Commands, 
    asset_server: &Res<AssetServer>, 

    spawn_request: UnitSpawnRequest, 
    ) {
    let mut spawn = commands.spawn((
        UnitBundle { 
            unit: Unit{

            },

            unit_movement: UnitMovement { 
                waypoints:  VecDeque::new(),
            },

            sprite_bundle: SpriteBundle { 
                texture: asset_server.load("sprite\\primitive\\64px_square.png"), 
                transform: Transform{translation: spawn_request.spawn_location, ..default()},
                ..default() 
            },

            // Physics
            rigid_body: RigidBody::KinematicPositionBased,
            locked_axes: LockedAxes::ROTATION_LOCKED,
            collider: Collider::ball(32.0),
        }, 

        Selectable {
            is_selected: false,   
        }
    ));
    
    let spawn = spawn.insert(Sensor); // (This makes it a trigger collider)

    let entity = spawn.id(); // Get unit entity, and insert component to store it
    spawn.insert(UnitEntity(entity));
}

// Callbacks
pub fn spawn_unit(
    manager: &mut UnitSpawnManager,
    spawn_request: UnitSpawnRequest
) {
    manager.spawn_requests.push(spawn_request);
}