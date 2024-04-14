#[cfg(debug_assertions)]
mod debug;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use mouse_pos::MainCamera;
use player_unit::spawn_player_unit;
use rts_controller::selection::r#box::visuals::spawn_selection_box;

fn main() {
    println!("Hello World");

    let mut app = App::new();

    app.add_plugins((
        MainPlugin,
    ));

    app.add_systems(Startup, (
        spawn_main_camera_startup,
        spawn_player_units_startup,
        spawn_selection_box_startup,
    ));

    #[cfg(debug_assertions)]
    app.add_plugins(
        //RapierDebugRenderPlugin{mode: DebugRenderMode::all(),..default()},
        debug::DebugPlugin
    );


    app.run();
}

pub struct MainPlugin;

impl Plugin for MainPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            DefaultPlugins,
            RapierPhysicsPlugin::<()>::default(),

            behaviour_tree::plugins::AllPlugins,
            health_to_death::HealthToDeathPlugin,
            mouse_pos::CursorTrackingPlugin,
            player_unit::PlayerUnitPlugin,
            rts_controller::RTSControllerPlugin,
            rts_direct_attack::DirectAttackPlugin,
            rts_unit_control::ControlPlugin,
            rts_unit_death::DeathPlugin,
            rts_unit_detectors::RTSUnitDetectorsPlugin,
            rts_unit_movers::MoversPlugin,
            rts_unit_nav::NavPlugin,
            death_flare::DeathFlarePlugin
        ));
    }
}

fn spawn_main_camera_startup(
    mut commands: Commands,
) {
    commands.spawn((
        MainCamera,
        Camera2dBundle::default(),
    ));
}

fn spawn_selection_box_startup(
    mut commands: Commands,
    asset_server: Res<AssetServer>, 
) {
    spawn_selection_box(&mut commands, &asset_server);
}

fn spawn_player_units_startup(
    mut commands: Commands,
    asset_server: Res<AssetServer>, 
) {
    spawn_player_unit(Vec2::new(-10.0, 0.0), &mut commands, &asset_server);
    spawn_player_unit(Vec2::new(-5.0, 0.0), &mut commands, &asset_server);
    spawn_player_unit(Vec2::new(0.0, 0.0), &mut commands, &asset_server);
    spawn_player_unit(Vec2::new(5.0, 0.0), &mut commands, &asset_server);
    spawn_player_unit(Vec2::new(10.0, 0.0), &mut commands, &asset_server);
}