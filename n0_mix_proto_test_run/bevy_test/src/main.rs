use bevy::prelude::*;

fn main() {
    println!("Hello, bevy!");

    App::new()
    .add_plugins(DefaultPlugins)
    .add_plugins(HelloPlugin)
    .run();
}

pub struct HelloPlugin;
impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app
        .insert_resource(TimePrinter(Timer::from_seconds(2.0, TimerMode::Repeating)))
        .add_systems(PreStartup, hello_world)
        .add_systems(Startup, add_test_hellos)
        .add_systems(PostStartup, print_hellos)
        .add_systems(Update, print_time);
    }
}

fn hello_world(){
    println!("Hello, world!");
}

#[derive(Component)]
struct HelloComponent;

#[derive(Component)]
struct  Name(String);

fn add_test_hellos(mut commands: Commands) {
    commands.spawn((HelloComponent, Name("El Gringo".to_string())));
    commands.spawn((HelloComponent, Name("Gringus".to_string())));
    commands.spawn((HelloComponent, Name("Glongus".to_string())));
}

fn print_hellos(query: Query<&Name, With<HelloComponent>>){
    for name in &query {
        println!("{}", name.0);
    }
}

#[derive(Resource)]
struct TimePrinter(Timer);

fn print_time(time: Res<Time>, mut timer: ResMut<TimePrinter>) {
    if timer.0.tick(time.delta()).just_finished()
    {
        println!("time");
    }
}