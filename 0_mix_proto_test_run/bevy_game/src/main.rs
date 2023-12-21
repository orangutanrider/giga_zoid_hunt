mod player;

use bevy::DefaultPlugins;
use bevy::prelude::*;

fn main() {
    println!("");
    println!("Hello, bevy.");

    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(InitializePlugin)
        .add_plugins(player::InitializePlugin)
        .run();
}

pub struct InitializePlugin;
impl Plugin for InitializePlugin {
    fn build(&self, app: &mut App) {
        println!("");
        println!("Initializing");
        app
            .add_systems(PreStartup, pre_initialize)
            .add_systems(Startup, initialize)
            .add_systems(PostStartup, post_initialize);
    }
}

fn pre_initialize(mut commands: Commands) {
    println!("");
    println!("pre_initialize");
}

fn initialize(mut commands: Commands) {
    println!("");
    println!("initialize");

    println!("Spawning Camera2dBundle");
    commands.spawn(Camera2dBundle::default());
}

fn post_initialize(mut commands: Commands) {
    println!("");
    println!("post_initialize");
    
}