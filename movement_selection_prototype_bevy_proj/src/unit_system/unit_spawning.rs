use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use super::*;

#[derive(Component)]
pub struct UnitSpawnList{
    pub spawn_points: Vec<Vec3>,
}

#[derive(Bundle)]
struct UnitBundle{
    pub unit: Unit,
    pub sprite_bundle: SpriteBundle,

    // Physics
    pub rigid_body: RigidBody,
    pub locked_axes: LockedAxes,
    pub collider: Collider,
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
    commands.spawn(UnitSpawnList{spawn_points: Vec::new()});
}

fn update(mut commands: Commands, asset_server: Res<AssetServer>, mut q: Query<&mut UnitSpawnList>){
    let spawn_points: &mut Vec<Vec3> = &mut q.single_mut().spawn_points;
    if spawn_points.len() == 0{
        return;
    }

    for spawn_point in spawn_points.iter(){
        spawn_unit_internal(&mut commands, &asset_server, *spawn_point);
    }

    spawn_points.clear();
}

// There is a anchor point (i.e. pivot point) for sprites, I will need to add somekind of implementation for that
fn spawn_unit_internal(commands: &mut Commands, asset_server: &Res<AssetServer>, spawn_point: Vec3){
    commands.spawn((
        UnitBundle{ 
            sprite_bundle: SpriteBundle { 
                texture: asset_server.load("sprite\\primitive\\64px_square.png"), 
                transform: Transform{translation: spawn_point, ..default()},
                ..default() 
            },
            unit: Unit{},

            // Physics
            rigid_body: RigidBody::KinematicPositionBased,
            locked_axes: LockedAxes::ROTATION_LOCKED,
            collider: Collider::ball(32.0),
        }, 
    ))
        .insert(Sensor); // (This makes it a trigger collider)
}

// I wanted this to just use the spawn_point: Vec3 field and it only, but it seems impossible

/* 
pub fn spawn_unit(spawn_point: Vec3, mut q: Query<&mut UnitSpawnList>){
    q.single_mut().spawn_points.push(spawn_point);
} */

pub fn spawn_unit(spawn_point: Vec3, spawn_list: &mut UnitSpawnList){
    spawn_list.spawn_points.push(spawn_point);
}

// So my question at this point
// Is this better?
// I think it is more organised, but it isn't as good as I wanted it to be.
// I'm thinking again about the System Params, could I use them to do this but more cleanly?
// The thing is I got the impression that you weren't even supposed to create custom ones.