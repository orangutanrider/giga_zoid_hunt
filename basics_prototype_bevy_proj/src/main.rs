mod player_controller;

use bevy::DefaultPlugins;
use bevy::prelude::*;

// https://docs.rs/bevy_mouse_tracking_plugin/latest/bevy_mouse_tracking_plugin/
use bevy_mouse_tracking_plugin::prelude::*;

fn main() {
    println!("Hello, bevy.");

    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(MousePosPlugin)
        .add_plugins(InitializePlugin)
        .add_plugins(player_controller::InitializePlugin)
        .run();
}

pub struct InitializePlugin;
impl Plugin for InitializePlugin {
    fn build(&self, app: &mut App) {
        println!("");
        println!("Initializing");
        app
           .add_systems(Startup, spawn_main_camera)
           .add_systems(Update, dbg_mouse_pos);
    }
}


#[derive(Component)]
pub struct MainCamera;

fn spawn_main_camera(mut commands: Commands) {
    println!("Startup: Spawning MainCamera");
    commands
       .spawn((MainCamera, Camera2dBundle::default()))
       .add(InitMouseTracking)
       .add(InitWorldTracking)
       .insert(bevy_mouse_tracking_plugin::MainCamera);
}

fn dbg_mouse_pos(
    mouse: Res<bevy_mouse_tracking_plugin::MousePos>, 
    mouse_world: Res<bevy_mouse_tracking_plugin::MousePosWorld>,
    buttons: Res<Input<MouseButton>>
) {
    if buttons.just_pressed(MouseButton::Left) {
        println!("mouse position: {}, mouse world position: {}", *mouse, *mouse_world);
    }
}