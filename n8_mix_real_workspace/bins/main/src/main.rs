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
        spawn_selection_box_startup,

        //spawn_player_units_startup, 
        spawn_x_player_units_startup,
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
        ));

        app.add_plugins((
            attack_laser::LaserVisualsPlugin,
            detection_colour::DetectionColourPlugin,
            rts_unit_detectors::RTSUnitDetectorsPlugin,
            rts_unit_movers::MoversPlugin,
            rts_unit_nav::NavPlugin,
            death_flare::DeathFlarePlugin,
            bang_colour::BangColourPlugin,
        ));
    }
}

fn spawn_main_camera_startup(
    mut commands: Commands,
) {
    commands.spawn((
        MainCamera,
        //Camera2dBundle::default(),
        Camera2dBundle{
            projection: OrthographicProjection { 
                scale: 1.5, 
                far: 1000.,
                near: -1000.,
                ..Default::default() 
            },
            ..Default::default()
        }
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
    spawn_player_unit(Vec2::new(-90.0, 0.0), &mut commands, &asset_server);
    spawn_player_unit(Vec2::new(-45.0, 0.0), &mut commands, &asset_server);
    spawn_player_unit(Vec2::new(0.0, 0.0), &mut commands, &asset_server);
    spawn_player_unit(Vec2::new(45.0, 0.0), &mut commands, &asset_server);
    spawn_player_unit(Vec2::new(90.0, 0.0), &mut commands, &asset_server);
}

const X: usize = 15;
fn spawn_x_player_units_startup(
    commands: Commands,
    asset_server: Res<AssetServer>, 
) {
    spawn_x_player_units(X, commands, asset_server);
}

fn spawn_x_player_units(
    x: usize,
    mut commands: Commands,
    asset_server: Res<AssetServer>, 
) {
    spawn_player_unit(Vec2::ZERO, &mut commands, &asset_server);
    if x <= 0 {
        return;
    }
    spawn_x_player_units(x - 1, commands, asset_server);
}