use bevy::prelude::*;

#[derive(Component)]
struct Player;

pub struct InitializePlugin;
impl Plugin for InitializePlugin {
    fn build(&self, app: &mut App) {
        println!("");
        println!("Initializing Player");
        app
            .add_systems(PostStartup, spawn_player)
            .add_systems(Update, move_player);
    }
}

fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    println!("spawn_player");

    // SpriteBundle includes:
    // Sprite, Transform, GlobalTransform, Texture, Visibility, ComputedVisibility

    commands.spawn( (
        Player, 
        SpriteBundle { texture: asset_server.load("playerProto1.png"), ..default()}));
}

const MOVE_SPEED: f32 = 100.0;
fn move_player (
    mut player: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
    input: Res<Input<KeyCode>>
){
    let mut player = player.single_mut();

    if input.any_pressed([KeyCode::A]) {
        player.translation.x -= MOVE_SPEED * time.delta_seconds();
    } else if input.any_pressed([KeyCode::D]){
        player.translation.x += MOVE_SPEED * time.delta_seconds();
    }
}