mod unit_selector;

use bevy::prelude::*;

pub struct InitializePlugin;
impl Plugin for InitializePlugin {
    fn build(&self, app: &mut App) {
        println!("");
        println!("Initializing player_controller");
        app
           .add_plugins(unit_selector::InitializePlugin)
           .add_systems(PostStartup, spawn_test_units)
           .add_systems(Update, movement_test);
    }
}

#[derive(Component)]
struct TestUnit{
    pub target_position: Vec2,
    pub selected: bool,
}

#[derive(Bundle)]
struct TestUnitBundle{
    pub test_unit: TestUnit,
    pub sprite_bundle: SpriteBundle,
}

fn spawn_test_unit(mut commands: Commands, asset_server: Res<AssetServer>){
    commands.spawn((
        TestUnitBundle{ 
            sprite_bundle: SpriteBundle { texture: asset_server.load("sprite\\basics\\64px_square.png"), ..default() },
            test_unit: TestUnit{ target_position: Vec2 { x: (0.0), y: (0.0) }, selected: false},
        }, 
    ));
}

fn spawn_test_units(mut commands: Commands, asset_server: Res<AssetServer>){
    let spawn_point_1 = Vec3{x: 100.0, y: 100.0, z: 0.0};
    let spawn_point_2 = Vec3{x: -100.0, y: -100.0, z: 0.0};
    let spawn_point_3 = Vec3{x: -100.0, y: 100.0, z: 0.0};
    let spawn_point_4 = Vec3{x: 100.0, y: -100.0, z: 0.0};

    commands.spawn((
        TestUnitBundle{ 
            sprite_bundle: SpriteBundle { 
                texture: asset_server.load("sprite\\basics\\64px_square.png"), 
                transform: Transform { translation: spawn_point_1, ..default() },
                ..default() },
            test_unit: TestUnit{ target_position: spawn_point_1.truncate(), selected: false },
        }, 
    ));
    commands.spawn((
        TestUnitBundle{ 
            sprite_bundle: SpriteBundle { 
                texture: asset_server.load("sprite\\basics\\64px_square.png"), 
                transform: Transform { translation: spawn_point_2, ..default() },
                ..default() },
            test_unit: TestUnit{ target_position: spawn_point_2.truncate(), selected: false },
        }, 
    ));
    commands.spawn((
        TestUnitBundle{ 
            sprite_bundle: SpriteBundle { 
                texture: asset_server.load("sprite\\basics\\64px_square.png"), 
                transform: Transform { translation: spawn_point_3, ..default() },
                ..default() },
            test_unit: TestUnit{ target_position: spawn_point_3.truncate(), selected: false },
        }, 
    ));
    commands.spawn((
        TestUnitBundle{ 
            sprite_bundle: SpriteBundle { 
                texture: asset_server.load("sprite\\basics\\64px_square.png"), 
                transform: Transform { translation: spawn_point_4, ..default() },
                ..default() },
            test_unit: TestUnit{ target_position: spawn_point_4.truncate(), selected: false },
        }, 
    ));
}

// hmm
// this is the main difficulty for me rn
// all this foreign lingo
// stuff like &, and *
// this stuff is new to me, but yeah, it's all abreviated and stuff
// not a bad thing, I can see why you'd want something core in a language to be abreviated
// everyone will learn it eventually 
// but yeah, it's kinda annoying, I don't see why you couldn't have it both ways too
// hmm, maybe you could, maybe I'm just clueless though, as they do have other stuff in here like Ref and ref, weird.

fn movement_test(
    mouse_world: Res<bevy_mouse_tracking_plugin::MousePosWorld>,
    buttons: Res<Input<MouseButton>>,

    keys: Res<Input<KeyCode>>,
    commands: Commands, 
    asset_server: Res<AssetServer>,

    mut unit_query: Query<(&mut Sprite, &mut Transform, &mut TestUnit), With<TestUnit>>
){
    //let (mut transform, mut test_unit) = unit_query.single_mut();

    if keys.just_pressed(KeyCode::Space){
        spawn_test_unit(commands, asset_server);
    }

    for (mut sprite, mut transform, mut test_unit) in unit_query.iter_mut(){
        unit_movement(&mut test_unit, &mut transform);
        unit_state_colourizer(&test_unit, &mut sprite);

        if !buttons.just_pressed(MouseButton::Left) 
        {
            continue;
        }

        unit_move_to(&mut test_unit,mouse_world.truncate());
    }

    if !buttons.just_pressed(MouseButton::Left) 
    {
        return;
    }
    println!("mouse world position: {}", *mouse_world);
}

fn unit_move_to(unit: &mut TestUnit, position: Vec2){
    if unit.selected == false {
        return;
    }

    unit.target_position = position;
}

fn unit_movement(unit: &mut TestUnit, unit_transform: &mut Transform){
    const MOVE_SPEED: f32 = 1.1;

    let vec2_position = Vec2::new(unit_transform.translation.x, unit_transform.translation.y);

    let new_position = unit.target_position - vec2_position; // difference of vectors
    let new_position = new_position.normalize_or_zero(); // movement direction
    let new_position = new_position * MOVE_SPEED; // movement vector
    let new_position = vec2_position + new_position; // new position

    move_transform(unit_transform, new_position);
}

fn unit_state_colourizer(unit: &TestUnit, unit_sprite: &mut Sprite){
    if(unit.selected == true){
        unit_sprite.color = Color::BLUE;
    }
    else{
        unit_sprite.color = Color::WHITE;
    }
}

fn move_transform(transform: &mut Transform, position: Vec2){
    transform.translation = Vec3::new(position.x, position.y, transform.translation.z);
}

/* 
fn mouse_click(
    mouse: Res<bevy_mouse_tracking_plugin::MousePos>, 
    mouse_world: Res<bevy_mouse_tracking_plugin::MousePosWorld>,
    buttons: Res<Input<MouseButton>>
){
    if buttons.just_pressed(MouseButton::Left) {
        println!("mouse position: {}, mouse world position: {}", *mouse, *mouse_world);
    }
}
*/