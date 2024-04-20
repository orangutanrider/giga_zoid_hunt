#[cfg(debug_assertions)]
mod debug;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_rand::prelude::*;

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
        spawn_bounds_startup,
        no_gravity_startup,

        spawn_player_units_startup,
        spawn_enemy_startup,
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
            EntropyPlugin::<WyRand>::default(),
        ));

        app.add_plugins((
            rts_unit_health::HealthPlugin,
            enemy::EnemyPlugin,
            behaviour_tree::plugins::AllPlugins,
            health_to_death::HealthToDeathPlugin,
            mouse_pos::CursorTrackingPlugin,
            player_unit::PlayerUnitPlugin,
            rts_controller::RTSControllerPlugin,
            rts_direct_attack::DirectAttackPlugin,
            rts_unit_control::ControlPlugin,
            rts_unit_death::DeathPlugin,
            attack_laser::LaserVisualsPlugin,
            detection_colour::DetectionColourPlugin,
            rts_unit_detectors::RTSUnitDetectorsPlugin,
            rts_unit_movers::MoversPlugin,
            rts_unit_nav::NavPlugin,
        ));

        app.add_plugins((
            bang_colour::BangColourPlugin,
            death_flare::DeathFlarePlugin,
            selection_visuals::SelectionMotifPlugin,
            sprite_sorting::SpriteSorterPlugin,
        ));
    }
}

fn no_gravity_startup(
    mut gravity: ResMut<RapierConfiguration>,
) {
    gravity.gravity = Vec2::ZERO;
}

const BACKGROUND_COLOUR: Color = Color::hsl(0.0, 0.0, 0.05);
fn spawn_main_camera_startup(
    mut commands: Commands,
) {
    commands.spawn((
        MainCamera,
        //Camera2dBundle::default(),
        Camera2dBundle{
            camera: Camera{
                clear_color: ClearColorConfig::Custom(BACKGROUND_COLOUR),
                ..Default::default()
            },
            projection: OrthographicProjection { 
                scale: 1.45, 
                far: 10000.,
                near: -10000.,
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

const N: u32 = 15;
fn spawn_player_units_startup(
    commands: Commands,
    asset_server: Res<AssetServer>, 
) {
    spawn_n_player_units_in_ellipse(N, commands, asset_server);
}

const MAJOR: f32 = 750.0;
const MINOR: f32 = 500.0;
fn spawn_n_player_units_in_ellipse(
    n: u32,
    mut commands: Commands,
    asset_server: Res<AssetServer>, 
) {
    let spawn_locations = distributed_ellipse_perimeter_points(n, MAJOR, MINOR);
    
    for spawn_location in spawn_locations.iter() {
        spawn_player_unit(*spawn_location, &mut commands, &asset_server);
    }
}

fn spawn_enemy_startup(
    mut commands: Commands,
    asset_server: Res<AssetServer>, 
) {
    enemy::spawn_enemy(Vec2::ZERO, &mut commands, &asset_server)
}

fn spawn_bounds_startup(
    mut commands: Commands,
    asset_server: Res<AssetServer>, 
) {
    level::spawn_bounds(&mut commands, &asset_server);
}

// https://mathoverflow.net/questions/28070/finding-n-points-that-are-equidistant-around-the-circumference-of-an-ellipse 
// https://stackoverflow.com/questions/6972331/how-can-i-generate-a-set-of-points-evenly-distributed-along-the-perimeter-of-an
fn distributed_ellipse_perimeter_points(
    n: u32,
    major: f32, // Height
    minor: f32, // Width
) -> Vec<Vec2> {
    use std::f32::consts::PI;
    let n_f32 = n as f32;

    let mut theta: f32 = 0.0;
    let two_pi = PI * 2.0;
    let delta_theta = 0.0001;
    let num_integrals = f32::round(two_pi / delta_theta) as u32;
    let mut circ: f32 = 0.0;

    /* integrate over the elipse to get the circumference */
    let mut index: u32 = 0;
    while index < num_integrals {
        let f32_index = index as f32;

        theta = theta + (f32_index * delta_theta);
        let dpt = calculate_dpt(major, minor, theta);
        circ = circ + dpt;
        
        index = index + 1;
    }

    let mut next = 0;
    let mut run = 0.0;
    let mut theta = 0.0;

    let mut output = Vec::new();

    let mut index: u32 = 0;
    while index < num_integrals {
        theta = theta + delta_theta;
        let sub_integral = n_f32 * run / circ;
        if sub_integral as i32 >= next {
            let x = major * f32::cos(theta);
            let y = minor * f32::sin(theta);

            output.push(Vec2::new(x, y));

            next = next + 1;
        }
        run = run + calculate_dpt(major, minor, theta);

        index = index + 1;
    }

    return output;
}

fn calculate_dpt(
    major: f32,
    minor: f32,
    theta: f32,
) -> f32 {
    let dpt_sin = major * f32::sin(theta);
    let dpt_sin = dpt_sin * dpt_sin;

    let dpt_cos = minor * f32::cos(theta);
    let dpt_cos = dpt_cos * dpt_cos;

    let dp = f32::sqrt(dpt_sin + dpt_cos);

    return dp
}