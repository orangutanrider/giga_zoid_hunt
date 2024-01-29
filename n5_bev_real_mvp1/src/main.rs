mod rts_unit;
mod gameplay_controller;
mod player_units;
mod enemy_units;
mod rapier_config;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use mouse_tracking::prelude::*;

use bevy::DefaultPlugins;
use mouse_tracking::prelude::MousePosPlugin;

fn main() {
    println!("Hello, bevy.");

    let mut app = App::new();
    app.add_plugins((
        DefaultPlugins,
        RapierPhysicsPlugin::<()>::default(),
        MousePosPlugin,
        InitializePlugin,
    ));

    #[cfg(debug_assertions)]
    app.add_plugins(
        RapierDebugRenderPlugin{
            mode: DebugRenderMode::all(),
            ..default()
    });

    app.run();
}

pub struct InitializePlugin;
impl Plugin for InitializePlugin{
    fn build(&self, app: &mut App) {
        println!("Initializing main");
        app
        .add_plugins((
            rts_unit::InitializePlugin,
            gameplay_controller::InitializePlugin,
            player_units::InitializePlugin,
            enemy_units::InitializePlugin,
        ))
        .add_systems(Startup, (
            spawn_main_camera,
        ));
    }
}

#[derive(Component)]
pub struct MainCamera;
fn spawn_main_camera(mut commands: Commands) {
    commands
    .spawn((
        MainCamera,
        Camera2dBundle::default(),
    ))
    .add(InitMouseTracking)
    .add(InitWorldTracking)
    .insert(mouse_tracking::MainCamera);
}