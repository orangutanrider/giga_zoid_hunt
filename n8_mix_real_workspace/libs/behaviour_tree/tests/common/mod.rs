use bevy::prelude::*;

use behaviour_tree::bundles::{
    TreeNodeBundle,
    TreeRootBundle,
};
use behaviour_tree::prelude::*;

pub fn spawn_empty_root(
    bang: bool,
    world: &mut World
) -> Entity {
    return 
    world.spawn(TreeRootBundle{
        tree_bang: RootBang::new(bang),
        ..default()
    }).id()
}

pub fn spawn_empty_node(
    bang: bool,
    root: Entity, parent: Entity, world: &mut World
) -> Entity {
    return 
    world.spawn(TreeNodeBundle{
        bang: Bang::new(bang),
        to_parent: ToParentNode::new(parent),
        to_root: ToBehaviourRoot::new(root),
        ..default()
    }).set_parent(parent).id()
}

pub fn spawn_x_empty_nodes(
    bang: bool,
    mut x: usize, output: &mut Vec<Entity>, root: Entity, parent: Entity, world: &mut World
) {
    x = x - 1;

    let spawn = spawn_empty_node(bang, root, parent, world);
    output.push(spawn);

    if x == 0 {
        return;
    }

    spawn_x_empty_nodes(bang, x, output, root, parent, world);
}