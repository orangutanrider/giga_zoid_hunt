use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use super::*;

#[derive(Component)]
pub struct UnitSpawnManager{
    spawn_points: Vec<Vec3>,
    current_id: u128,
}

#[derive(Bundle)]
struct UnitBundle{
    unit: Unit,
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
           .add_systems(PreStartup, startup_unit_spawning)
           .add_systems(PostUpdate, update);
    }
}

fn startup_unit_spawning(mut commands: Commands){
    commands.spawn(UnitSpawnManager{
        spawn_points: Vec::new(),
        current_id: 1,
    });
}

fn update(mut commands: Commands, asset_server: Res<AssetServer>, mut q: Query<&mut UnitSpawnManager>){
    let mut manager = q.single_mut();
    if manager.spawn_points.len() == 0{
        return;
    }

    let id = manager.current_id;
    for spawn_point in manager.spawn_points.iter_mut(){
        let id = id + 1;
        spawn_unit_internal(&mut commands, &asset_server, *spawn_point, id);
    }
    manager.current_id = id + manager.spawn_points.len() as u128;
    
    manager.spawn_points.clear();
}

// There is a anchor point (i.e. pivot point) for sprites, I will need to add somekind of implementation for that
fn spawn_unit_internal(commands: &mut Commands, asset_server: &Res<AssetServer>, spawn_point: Vec3, id: u128){
    commands.spawn((
        UnitBundle{ 
            sprite_bundle: SpriteBundle { 
                texture: asset_server.load("sprite\\primitive\\64px_square.png"), 
                transform: Transform{translation: spawn_point, ..default()},
                ..default() 
            },
            unit: Unit{id},

            // Physics
            rigid_body: RigidBody::KinematicPositionBased,
            locked_axes: LockedAxes::ROTATION_LOCKED,
            collider: Collider::ball(32.0),
        }, 
    ))
        .insert(Sensor); // (This makes it a trigger collider)
}

pub fn spawn_unit(spawn_point: Vec3, spawn_list: &mut UnitSpawnManager){
    spawn_list.spawn_points.push(spawn_point);
}