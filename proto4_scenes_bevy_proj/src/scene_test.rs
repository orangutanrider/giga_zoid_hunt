//https://www.youtube.com/watch?v=4uASkH-FUWk

use std::fs::*;
use std::io::*;
use std::io::prelude::*;

use bevy::ecs::entity::EntityMap;
use bevy::prelude::*;
use bevy::scene::*;
use bevy::tasks::*;
use bevy::utils::*;

pub struct InitializePlugin;
impl Plugin for InitializePlugin {
    fn build(&self, app: &mut App) {
        println!("Initializing scene_test");
        app
           .init_resource::<SpriteSceneData>()
           .add_systems(Startup, (
            create_and_save_empty_scene,
            ))
           .add_systems(Update, (
            unload_unused_assets_on_f_press,
            clear_entities_on_d_press,
            despawn_all_sprite_scene_on_k_press,
            spawn_sprite_scene_using_spawner_on_l_press,
            load_and_spawn_empty_scene_via_spawner_on_o_press,
           ));
    }
}

fn despawn_all_sprite_scene_on_k_press(
    input: Res<Input<KeyCode>>,
    mut data: ResMut<SpriteSceneData>,
    mut spawner: ResMut<SceneSpawner>,
){
    if !input.just_pressed(KeyCode::K) {
        return;
    }

    println!("despawn_all_sprite_scene_on_k_press");

    let mut count = 0;
    for instance in data.instances.iter(){
        count+=1;
        spawner.despawn_instance(*instance);
    }
    println!("despawned {}, sprite scenes", count);

    data.instances.clear();
}

fn unload_unused_assets_on_f_press(
    input: Res<Input<KeyCode>>,
    asset_server: Res<AssetServer>,
){
    if !input.just_pressed(KeyCode::F) {
        return;
    }

    // Okay I can't seem to make this do anything, I don't know what it's really for, but it makes me think of Unity addresables
    // Not that I have ever used those
    // But yeah the point is, that I don't think ot matters if I do or don't use this, in the context of my project
    println!("unload_unused_assets_on_f_press");
    asset_server.mark_unused_assets();
    asset_server.free_unused_assets();
}

fn clear_entities_on_d_press(
    world: &mut World,
){
    let input = world.resource::<Input<KeyCode>>();
    if !input.just_pressed(KeyCode::D) {
        return;
    }

    println!("clear_entities_on_d_press");
    
    world.clear_entities();
    // This closes the app
    // Does that mean the app itself is an entity and in the world?
}

const EMPTY_TEST_SCENE_PATH: &'static str = "scenes/empty_test_scene.scn.ron";
fn create_and_save_empty_scene(
    world: &mut World,
) {
    println!("(Startup) create_and_save_empty_scene");

    let mut scene_world = World::new();
    scene_world.init_resource::<AppTypeRegistry>();
    let type_registry = world.resource::<AppTypeRegistry>();
    let scene = DynamicScene::from_world(&scene_world);
    let serialized_scene = scene.serialize_ron(type_registry).unwrap();

    println!("Saving empty_test_scene to a file");
    println!("empty_test_scene contents: ");
    println!("{}", serialized_scene);

    #[cfg(not(target_arch = "wasm32"))]
    IoTaskPool::get()
        .spawn(
            async move {
                File::create(format!("assets/{EMPTY_TEST_SCENE_PATH}"))
                    .and_then(|mut file| file.write(serialized_scene.as_bytes()))
                    .expect("Error while writing empty_test_scene to file");
    }).detach();
}

fn load_and_spawn_empty_scene_via_spawner_on_o_press(
    input: Res<Input<KeyCode>>,
    mut scene_spawner: ResMut<SceneSpawner>,
    asset_server: Res<AssetServer>,
) {
    if !input.just_pressed(KeyCode::O){
        return;
    }

    println!("load_and_spawn_empty_scene_via_spawner_on_o_press");

    // Load scene into the runtime's assets
    let scene_handle = asset_server.load(EMPTY_TEST_SCENE_PATH);

    // Spawn scene
    scene_spawner.spawn(scene_handle);
}

#[derive(Resource)]
struct SpriteSceneData{
    scene: Handle<Scene>,
    instances: Vec<InstanceId>,
}
impl FromWorld for SpriteSceneData {
    // create_sprite_scene_without_saving
    fn from_world(world: &mut World) -> Self {
        println!("(Res<SpriteSceneContainer> Initialize), create_sprite_scene_without_saving");

        // Create empty scene
        let mut scene_world = World::new();
    
        // Get asset server (if you have a world parameter in a system, you can't basically can't have anything else)
        let asset_server = world.resource::<AssetServer>();
    
        // Add sprite to scene
        scene_world.spawn(SpriteBundle{
            texture: asset_server.load("sprite\\primitive\\64px_square.png"), 
            ..default()
        });
    
        // Add empty scene to the runtime's assets
        let mut assets = world.resource_mut::<Assets<Scene>>();
        let scene = assets.add(Scene {world: scene_world});
    
        // Store the scene in the resource, by returning the scene handle
        SpriteSceneData{
            scene,
            instances: Vec::new(),
        }
    }
}

fn spawn_sprite_scene_using_spawner_on_l_press(
    input: Res<Input<KeyCode>>,
    mut scene_container: ResMut<SpriteSceneData>,
    mut scene_spawner: ResMut<SceneSpawner>,
){
    if !input.just_pressed(KeyCode::L){
        return;
    }

    println!("spawn_sprite_scene_using_spawner_on_l_press");

    // Spawn scene
    let instance = scene_spawner.spawn(scene_container.scene.clone());

    // Add scene to Vec, so it can be despawned in the despawn_all_sprite_scene_on_k_press function
    scene_container.instances.push(instance);
}