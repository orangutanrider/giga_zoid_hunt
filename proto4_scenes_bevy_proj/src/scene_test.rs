//https://www.youtube.com/watch?v=4uASkH-FUWk

use std::fs::*;
use std::io::*;
use std::io::prelude::*;

use bevy::ecs::entity::EntityMap;
use bevy::prelude::*;
use bevy::tasks::*;
use bevy::utils::*;

pub struct InitializePlugin;
impl Plugin for InitializePlugin {
    fn build(&self, app: &mut App) {
        println!("Initializing scene_test");
        app
            .init_resource::<AppTypeRegistry>()
            .register_type::<TestComponentA>()  
            .register_type::<TestComponentB>()
            .add_systems(Startup, (
                create_and_save_empty_scene,
                create_test_file,
                create_and_save_sprite_scene,
                create_and_save_component_test_scene,
            ))
            .add_systems(Update, (
                load_empty_test_scene_on_p_press,
                load_empty_test_scene_on_o_press_as_normal,
                load_sprite_test_scene_on_l_press,
                load_component_test_scene_on_k_press,
            ))
            ;
    }
}

// Okay, so for any component that is able to be loaded via a scene, it has to implement reflect
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct TestComponentA{
    pub e: f32,
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct TestComponentB{
    pub gringus: String,
    #[reflect(skip_serializing)]
    pub time_since_startup: Duration,
}
impl FromWorld for TestComponentB {
    fn from_world(world: &mut World) -> Self {
        let time = world.resource::<Time>();
        TestComponentB { 
            gringus: TestComponentB::DEFAULT_GRINGUS.to_string(), 
            time_since_startup: time.elapsed(),
        }
    }
}
impl TestComponentB {
    pub const DEFAULT_GRINGUS: &'static str = "default gringus";
}

const EMPTY_TEST_SCENE_PATH: &'static str = "scenes/empty_test_scene.scn.ron";

fn create_and_save_empty_scene(world: &mut World) {
    println!("Creating empty_test_scene");
    let mut scene_world = World::new();
    scene_world.init_resource::<AppTypeRegistry>();
    let type_registry = world.resource::<AppTypeRegistry>();
    let scene = DynamicScene::from_world(&scene_world);

    let serialized_scene = scene.serialize_ron(type_registry).unwrap();

    info!("{}", serialized_scene); // Show scene data (dunno what info! is, but I think it is a variant of print)

    // Okay so I think this is currently doing a thing
    // Where it only will update a file with the path specified, and not make a new file
    println!("Saving empty_test_scene");
    /* 
    #[cfg(not(target_arch = "wasm32"))]
    IoTaskPool::get()
        .spawn(
            async move {
                File::create(format!("{}", EMPTY_TEST_SCENE_PATH))
                    .and_then(|mut file| file.write(serialized_scene.as_bytes()))
                    .expect("Error while writing empty_test_scene to file");
        })
        .detach();
    */

    /* 
    let scene_file = File::create(EMPTY_TEST_SCENE_PATH);
    let mut scene_file = scene_file.unwrap();
    scene_file.write_all(serialized_scene.as_bytes());
    */

    /* 
    let mut file = File::create("foo.txt")?;
    file.write_all(b"Hello, world!")?;
    */

    /* 
    let mut scene_file = File::create("assets/{EMPTY_TEST_SCENE_PATH}")
    .expect("Couldn't create the test_scene file");
    scene_file.write_all(serialized_scene.as_bytes()).expect("Couldn't write to the test_scene file");
    */

    #[cfg(not(target_arch = "wasm32"))]
    IoTaskPool::get()
        .spawn(
            async move {
                File::create(format!("assets/{EMPTY_TEST_SCENE_PATH}"))
                    .and_then(|mut file| file.write(serialized_scene.as_bytes()))
                    .expect("Error while writing empty_test_scene to file");
        })
        .detach();
}

fn load_empty_test_scene_on_p_press(
    mut commands: Commands, 
    asset_server: Res<AssetServer>,
    input: Res<Input<KeyCode>>,
) {
    if input.just_pressed(KeyCode::P) == false {
        return;
    }
    println!("Loading empty_test_scene as dynamic");
    commands.spawn(DynamicSceneBundle{
        scene: asset_server.load(EMPTY_TEST_SCENE_PATH),
        ..default()
    });
}

fn load_empty_test_scene_on_o_press_as_normal(
    mut commands: Commands, 
    asset_server: Res<AssetServer>,
    input: Res<Input<KeyCode>>,
) {
    if input.just_pressed(KeyCode::O) == false {
        return;
    }
    println!("Loading empty_test_scene as normal");
    commands.spawn(SceneBundle{
        scene: asset_server.load(EMPTY_TEST_SCENE_PATH),
        ..default()
    });
}

fn create_test_file(){
    let mut file = File::create("assets/test_file.txt")
    .expect("Couldn't create test file");

    file.write_all(b"buf").expect("Couldn't write to test file");
}

const SPRITE_TEST_SCENE: &'static str = "scenes/sprite_test_scene.scn.ron";

fn create_and_save_sprite_scene(
    world: &mut World,
) {
    println!("Creating sprite_test_scene");
    let mut scene_world = World::new();
    scene_world.init_resource::<AppTypeRegistry>();
    let type_registry = world.resource::<AppTypeRegistry>();
    let scene = DynamicScene::from_world(&scene_world);
    let asset_server = world.resource::<AssetServer>();

    // This doesn't work, I think sprites or something aren't reflected
    println!("Populating sprite_test_scene");
    scene_world.spawn(SpriteBundle{
        texture:  asset_server.load("sprite\\primitive\\64px_square.png"),
        transform: Transform {translation: Vec3::ZERO, ..default()},
        ..default()
    });

    println!("Saving sprite_test_scene");
    let serialized_scene = scene.serialize_ron(type_registry).unwrap();
    info!("{}", serialized_scene); // Show scene data (dunno what info! is, but I think it is a variant of print)

    #[cfg(not(target_arch = "wasm32"))]
    IoTaskPool::get()
        .spawn(
            async move {
                File::create(format!("assets/{SPRITE_TEST_SCENE}"))
                    .and_then(|mut file| file.write(serialized_scene.as_bytes()))
                    .expect("Error while writing sprite_test_scene to file");
        })
        .detach();
}

fn load_sprite_test_scene_on_l_press(
    mut commands: Commands, 
    asset_server: Res<AssetServer>,
    input: Res<Input<KeyCode>>,
) {
    if input.just_pressed(KeyCode::L) == false {
        return;
    }
    println!("Loading sprite_test_scene as normal");
    commands.spawn(SceneBundle{
        scene: asset_server.load(SPRITE_TEST_SCENE),
        ..default()
    });
}

const TEST_COMPONENET_TEST_SCENE: &'static str = "scenes/component_test_scene.scn.ron";

// This doesn't work
// Though, it does give something different
// But it doesn't give what it's supposed to
fn create_and_save_component_test_scene(
    world: &mut World,
) {
    println!("Creating component_test_scene");
    let mut scene_world = World::new();
    
    scene_world.init_resource::<AppTypeRegistry>();
    world.init_resource::<AppTypeRegistry>();

    let mut component_b = TestComponentB::from_world(world);
    component_b.gringus = "el gringo".to_string();
    scene_world.spawn((
        component_b,
        TestComponentA{e: 42.0},
        Transform::IDENTITY,
    ));
    
    let type_registry = scene_world.resource::<AppTypeRegistry>();
    let scene = DynamicScene::from_world(&scene_world);
    
    let serialized_scene = scene.serialize_ron(type_registry).unwrap();
    println!("{}", serialized_scene);

    #[cfg(not(target_arch = "wasm32"))]
    IoTaskPool::get()
        .spawn(
            async move {
                File::create(format!("assets/{TEST_COMPONENET_TEST_SCENE}"))
                    .and_then(|mut file| file.write(serialized_scene.as_bytes()))
                    .expect("Error while writing component_test_scene to file");
        })
        .detach();
}

fn load_component_test_scene_on_k_press(
    mut commands: Commands, 
    asset_server: Res<AssetServer>,
    input: Res<Input<KeyCode>>,
) {
    if input.just_pressed(KeyCode::K) == false {
        return;
    }
    println!("Loading component_test_scene as normal");
    commands.spawn(SceneBundle{
        scene: asset_server.load(TEST_COMPONENET_TEST_SCENE),
        ..default()
    });
}