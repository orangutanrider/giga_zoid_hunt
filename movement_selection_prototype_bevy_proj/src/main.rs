mod player_controller;
mod unit_system;

use bevy::prelude::*;

use bevy::DefaultPlugins;
use mouse_tracking::prelude::MousePosPlugin;
fn main() {
    println!("Hello, bevy.");

    App::new()
        .add_plugins((
            DefaultPlugins,
            MousePosPlugin,
            InitializePlugin,
        ))
        .run();
}

pub struct InitializePlugin;
impl Plugin for InitializePlugin {
    fn build(&self, app: &mut App) {
        println!("");
        println!("Initializing main.rs");
        app
           .add_plugins((
                unit_system::InitializePlugin, 
                player_controller::InitializePlugin,
            ))
           .add_systems(Startup, spawn_main_camera);
    }
}

use mouse_tracking::prelude::*;
#[derive(Component)]
pub struct MainCamera;
fn spawn_main_camera(mut commands: Commands) {
    println!("Startup: Spawning MainCamera");
    commands
       .spawn((MainCamera, Camera2dBundle::default()))
       .add(InitMouseTracking)
       .add(InitWorldTracking)
       .insert(mouse_tracking::MainCamera);
}