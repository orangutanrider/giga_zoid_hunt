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
        .add_systems(Startup, (
            create_and_save_test_scene,
        ))
        .add_systems(Update, (
            save_world_as_dynamic_scene_on_w_press,
        ));
    }
}

const WORLD_TEST_SCENE_PATH: &'static str = "scenes/world_test_scene.scn.ron";
fn save_world_as_dynamic_scene_on_w_press(
    world: &mut World,
){
    // Is this even possible?
    // Okay no I think it isn't
    // There's no automatic thing for getting a dynamic world with all dynamics from a world that contains non-dynamics
    // Agh, hmm...
    // It seems you can do something kind of like this though
    // https://www.youtube.com/watch?v=WDsFT8DcyGc&t=458s
    // At 4:22 he shows pseudo code for something that'd do something similar to this
    // Hmm, it makes me question whether I should be doing this to begin with though.

    // Hmm...
    // I think I should right? Cause most things aren't serialized.
    // I suppose they don't necessarily need to be serialized though? Yeah I could have one dynamic scene for editing
    // And then the camera and everything is non-dynamic, just serving to host the editing process.
    // Hmmm... Yeah I think that should be possible.

    // Yeah, so then I can have dynamic scenes for the stuff that is specifically made to be saved and edited
    // And then everything else is not like that.
    // Hmm... 
    // How this can be used and should be used is open-ended.
    // In my game, what is relevant?
    // Hmm, yeah, hmm...

    // I think for now, I should just test saving a position for a sprite or something, yeah.
    // I do have a wonder with this though, hmm, what happens when I have a reflected component that contains components that aren't reflected?
    // You can do that right? I think so?
    // Or, hmm, can you? I think you can't actually.

    // Okay so yeah, that's a bundle.
    // Hmm, yeah okay, there is a thing with this actually, to have fully dynamic scenes is strange isn't it?
    // Because doesn't it make more sense to have dynamic components or something?
    // Like having the transform be dynamic?

    // Hmm, okay yeah lets think of this by imagining how I would make a way to save and edit the position of a sprite.
    // If there was a fully dynamic scene for the stuff being saved (the position of the sprite).
    // Then it'd just have to contain a invisible entity that follows this sprite around, right?
    // How would it do that? Yeah through query<>
    // But doesn't that seem ineffecient?
    // why shouldn't it just be that the transform is already a reflected thing?
    // And that I can just scan that scene and then save the data that is reflected.

    // I think you can do that, yeah?
    // But it is strange too, is a dynamic scene even a thing?
    // Is it just a data type then?
    // Do you just work with normal scenes and then convert them to dynamic for saving or something?

    // Hmm... I feel I'm not understanding this.
    // Hmm...

    // Okay I watched the video again.
    // It seems like dynamic scenes are what you save as .scn.ron files
    // That you have to convert things to dynamic scene to save them
    // So yeah?
    
    // But? Hmm... Yeah? It makes sense I think.
    // Yeah...

    // Okay though? What is a dynamic scene in the context of the runtime?
    // Is it anything? 
    // Hmm? I think it isn't.
    // But there is a thing here man, cause then all scenes you save are the dynamic ones.
    // So, hmm... I guess that still makes sense and works though
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