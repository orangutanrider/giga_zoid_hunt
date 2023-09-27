use bevy::prelude::*;
use super::*;

#[derive(Component)]
struct UnitSpawnList{
    pub spawn_points: Vec<Vec3>,
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

fn spawn_unit_internal(commands: &mut Commands, asset_server: &Res<AssetServer>, spawn_point: Vec3){
    commands.spawn((
        UnitBundle{ 
            sprite_bundle: SpriteBundle { 
                texture: asset_server.load("sprite\\primitive\\64px_square.png"), 
                transform: Transform{translation: spawn_point, ..default()},
                ..default() 
            },
            unit: Unit{},
        }, 
    ));
}

// I wanted this to just use the spawn_point: Vec3 field and it only, but I can't find anyway of avoiding Query
pub fn spawn_unit(mut q: Query<&mut UnitSpawnList>, spawn_point: Vec3){
    q.single_mut().spawn_points.push(spawn_point);
}