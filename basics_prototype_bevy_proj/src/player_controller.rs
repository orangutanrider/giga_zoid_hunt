use bevy::prelude::*;

pub struct InitializePlugin;
impl Plugin for InitializePlugin {
    fn build(&self, app: &mut App) {
        println!("");
        println!("Initializing player_controller");
        app
           .add_systems(PostStartup, spawn_test_unit)
           .add_systems(Update, movement_test);
    }
}

#[derive(Component)]
struct TestUnit;

#[derive(Bundle)]
struct TestUnitBundle{
    pub sprite_bundle: SpriteBundle,
}

fn spawn_test_unit(mut commands: Commands, asset_server: Res<AssetServer>){
    commands.spawn((
        TestUnitBundle{ 
            sprite_bundle: SpriteBundle { texture: asset_server.load("sprite\\basics\\64px_square.png"), ..default() }
        }, 
        TestUnit
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

    mut unitQuery: Query<&mut Transform, With<TestUnit>>
){
    if !buttons.just_pressed(MouseButton::Left) 
    {
        return;
    }

    /* 
    for transform in unitQuery.iter() {
        move_test_unit_to(transform, mouse_world.truncate())
    }
    */

    move_test_unit_to(&mut unitQuery.single_mut(), mouse_world.truncate());

    println!("mouse world position: {}", *mouse_world);
}

fn move_test_unit_to(unit: &mut Transform, position: Vec2){
    unit.translation = Vec3::new(position.x, position.y, unit.translation.z);
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