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
    app.add_systems(Update, deactivation_propagation_sys);

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
        return;
    };
    bang.fizzler_deactivate();

    // Depth1 validate
    app.add_systems(PostUpdate, depth1_validator);
    app.update();

    // Depth2 validate
    app.add_systems(PostUpdate, depth2_validator);
    app.update();
}

fn depth1_validator(
    q: Query<&Bang, With<Depth1Node>>
) {
    let mut depth1_bang = false;
    for bang in q.iter() {
        if bang.is_active() {
            depth1_bang = true;
        }
    }
    assert_eq!(depth1_bang, false);
}

fn depth2_validator(
    q: Query<&Bang, With<Depth2Node>>
) {
    let mut depth2_bang = false;
    for bang in q.iter() {
        if bang.is_active() {
            depth2_bang = true;
        }
    }
    assert_eq!(depth2_bang, false);
}