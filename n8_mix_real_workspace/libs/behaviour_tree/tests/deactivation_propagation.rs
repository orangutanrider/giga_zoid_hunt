mod common;
use common::*;

use bevy::prelude::*;

use behaviour_tree::prelude::*;
use behaviour_tree::plugins::internal_systems::*;

#[derive(Component)]
struct Depth1Node;

#[derive(Component)]
struct Depth2Node;

#[test]
fn deactivation_propagation_test() {
    let mut app = App::new();

    // Add systems
    app.add_systems(Update, (
        auto_release_propagation_sys,
        auto_release_sys.after(auto_release_propagation_sys)
    ));

    // Spawn
    let world = &mut app.world;

    // Root and 1st node
    let root = spawn_empty_root(true, world);
    let node1 = spawn_empty_node(true, root, root, world);

    // Node1 children
    let mut depth_1: Vec<Entity> = Vec::new();
    spawn_x_empty_nodes(true, 3, &mut depth_1, root, node1, world);
    for node in depth_1.iter() {
        let mut node = world.entity_mut(*node);
        node.insert(Depth1Node);
    }

    // Another layer of children
    let mut depth_2: Vec<Entity> = Vec::new();
    for node in depth_1.iter() {
        spawn_x_empty_nodes(true, 3, &mut depth_2, root, *node, world);
    }
    for node in depth_2.iter() {
        let mut node = world.entity_mut(*node);
        node.insert(Depth2Node);
    }

    // Deactivate node1
    let Some(mut bang) = world.get_mut::<Bang>(node1) else {
        println!("Failed to get node1's bang");
        return;
    };
    bang.fizzler_deactivate();
    assert_eq!(bang.is_active(), false, "Asserting that node1 was fizzled");

    println!("Frame step 1");
    app.add_systems(PostUpdate, (
        depth1_validator::<false>,
        depth2_validator::<true>,
    ));
    app.update();

    println!("Frame step 2");
    app.add_systems(PostUpdate, (
        depth1_validator::<false>,
        depth2_validator::<false>,
    ));
    app.update();
}

fn depth1_validator<const BANG: bool>(
    q: Query<&Bang, With<Depth1Node>>
) {
    let mut depth1_bang = false;
    for bang in q.iter() {
        if bang.is_active() {
            depth1_bang = true;
        }
    }
    println!("Asserting that depth1 nodes are {}", BANG);
    assert_eq!(depth1_bang, BANG);
}

fn depth2_validator<const BANG: bool>(
    q: Query<&Bang, With<Depth2Node>>
) {
    let mut depth2_bang = false;
    for bang in q.iter() {
        if bang.is_active() {
            depth2_bang = true;
        }
    }
    println!("Asserting that depth2 nodes are {}", BANG);
    assert_eq!(depth2_bang, BANG, "Asserting depth2 nodes are {}", BANG);
}