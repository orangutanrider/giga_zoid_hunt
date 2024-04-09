use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use mouse_pos::MainCamera;

fn main() {
    println!("Hello World");

    let mut app = App::new();
    
    app.add_systems(Startup, spawn_main_camera);

    app.add_plugins((
        MainPlugin,
    ));

    #[cfg(debug_assertions)]
    app.add_plugins(
        RapierDebugRenderPlugin{
            mode: DebugRenderMode::all(),
            ..default()
    });

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
        ));

        // #[cfg(debug_assertions)]
        // app.add_plugins(
        //     RapierDebugRenderPlugin{
        //         mode: DebugRenderMode::all(),
        //         ..default()
        // });
    }
}

fn spawn_main_camera(
    mut commands: Commands,
) {
    commands.spawn((
        MainCamera,
        Camera2dBundle::default(),
    ));
}