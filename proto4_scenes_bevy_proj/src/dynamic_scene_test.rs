//https://www.youtube.com/watch?v=4uASkH-FUWk
//https://www.youtube.com/watch?v=bbBh3oKibkE

use std::fs::*;
use std::io::*;
use std::io::prelude::*;

use bevy::reflect::*;
use bevy::ecs::entity::EntityMap;
use bevy::prelude::*;
use bevy::scene::*;
use bevy::tasks::*;
use bevy::utils::*;

pub struct InitializePlugin;
impl Plugin for InitializePlugin {
    fn build(&self, app: &mut App) {
        println!("Initializing dynamic_scene_test");
        app
        .register_type::<ComponentA>()
        .register_type::<ComponentB>()
        .register_type::<ResourceA>()
        .add_systems(Startup, (
            create_and_save_test_scene,
            create_and_save_component_test_scene,
        ))
        .add_systems(Update, (
            create_and_save_component_test_scene_on_m_press,
        ));
    }
}

// for comp_test_scene
#[derive(Component, Reflect, Default)]
#[reflect(Component)] // this tells the reflect derive to also reflect component behaviors
struct ComponentA {
    pub x: f32,
    pub y: f32,
}

// for comp_test_scene
#[derive(Component, Reflect)]
#[reflect(Component)]
struct ComponentB {
    pub value: String,
    #[reflect(skip_serializing)]
    pub _time_since_startup: Duration,
}
impl FromWorld for ComponentB {
    fn from_world(world: &mut World) -> Self {
        let time = world.resource::<Time>();
        ComponentB {
            _time_since_startup: time.elapsed(),
            value: "Default Value".to_string(),
        }
    }
}

// for comp_test_scene
#[derive(Resource, Reflect, Default)]
#[reflect(Resource)]
struct ResourceA {
    pub e: u32,
}

// Relative to the asset folder
const COMPONENT_TEST_SCENE_PATH: &'static str = "scenes/comp_test_scene.scn.ron";
fn create_and_save_component_test_scene(
    world: &mut World,
){
    println!("(Startup) create_and_save_component_test_scene");

    let mut scene_world = World::new();

    let registry = world.resource::<AppTypeRegistry>().clone();
    scene_world.insert_resource(registry);

    let mut component_b = ComponentB::from_world(world);
    component_b.value = "el gringo".to_string();
    scene_world.spawn((
        component_b,
        ComponentA {x: 4.0, y: 2.0 },
        Transform::IDENTITY,
    ));
    scene_world.spawn(ComponentA {x: 2.2, y: 2.0 });
    scene_world.insert_resource(ResourceA {e: 300});

    let scene = DynamicScene::from_world(&scene_world);

    let registry = world.resource::<AppTypeRegistry>();
    let serialized_scene = scene.serialize_ron(registry).unwrap();
    
    println!("Saving component test scene to file, serialized data is as follows: ");
    println!("{}", serialized_scene);

    #[cfg(not(target_arch = "wasm32"))]
    IoTaskPool::get()
        .spawn(
            async move {
                File::create(format!("assets/{COMPONENT_TEST_SCENE_PATH}"))
                    .and_then(|mut file| file.write(serialized_scene.as_bytes()))
                    .expect("Error while writing comp_test_scene to file");
    }).detach();
}

fn create_and_save_component_test_scene_on_m_press(
    world: &mut World,
){
    let input = world.resource::<Input<KeyCode>>();
    if !input.just_pressed(KeyCode::M) {
        return;
    }

    println!("create_and_save_component_test_scene_on_m_press");

    let mut scene_world = World::new();

    let registry = world.resource::<AppTypeRegistry>().clone();
    scene_world.insert_resource(registry);

    let mut component_b = ComponentB::from_world(world);
    component_b.value = "el gringo".to_string();
    scene_world.spawn((
        component_b,
        ComponentA {x: 4.0, y: 2.0 },
        Transform::IDENTITY,
    ));
    scene_world.spawn(ComponentA {x: 2.2, y: 2.0 });
    scene_world.insert_resource(ResourceA {e: 300});

    let scene = DynamicScene::from_world(&scene_world);

    let registry = world.resource::<AppTypeRegistry>();
    let serialized_scene = scene.serialize_ron(registry).unwrap();
    
    println!("Saving component test scene to file, serialized data is as follows: ");
    println!("{}", serialized_scene);

    #[cfg(not(target_arch = "wasm32"))]
    IoTaskPool::get()
        .spawn(
            async move {
                File::create(format!("assets/{COMPONENT_TEST_SCENE_PATH}"))
                    .and_then(|mut file| file.write(serialized_scene.as_bytes()))
                    .expect("Error while writing comp_test_scene to file");
    }).detach();
}

const TEST_SCENE_PATH: &'static str = "scenes/test_scene.scn.ron";
fn create_and_save_test_scene(
    world: &mut World,
) {
    println!("(Startup) create_and_save_test_scene");

    let mut scene_world = World::new();
    scene_world.init_resource::<AppTypeRegistry>();
    let registry = world.resource::<AppTypeRegistry>();
    let scene = DynamicScene::from_world(&scene_world);
    let serialized_scene = scene.serialize_ron(registry).unwrap();

    println!("Saving test_scene to a file");
    println!("test_scene contents: ");
    println!("{}", serialized_scene);

    #[cfg(not(target_arch = "wasm32"))]
    IoTaskPool::get()
        .spawn(
            async move {
                File::create(format!("assets/{TEST_SCENE_PATH}"))
                    .and_then(|mut file| file.write(serialized_scene.as_bytes()))
                    .expect("Error while writing test_scene to file");
    }).detach();
}