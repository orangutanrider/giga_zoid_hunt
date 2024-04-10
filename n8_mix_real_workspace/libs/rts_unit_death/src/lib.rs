use bevy::prelude::*;

use ref_paths::*;

pub struct DeathPlugin;

impl Plugin for DeathPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostUpdate, referenced_entity_destruction_on_death_sys);
    }
}

#[derive(Component)]
pub struct DeathBang(bool);
impl Default for DeathBang {
    fn default() -> Self {
        Self(false)
    }
}
impl DeathBang {
    pub fn new() -> Self {
        return Self(false);
    }

    pub fn bang(&mut self) {
        self.0 = true;
    }
}

#[derive(Component, Default)]
/// Data transformation flag.
pub struct DeathToEntityDespawn;

#[derive(Component, Default)]
/// Data destination, reference flag.
pub struct DespawnTargetIsReference;

#[derive(Component)]
pub struct ToDespawnTarget(Entity);
waymark!(ToDespawnTarget);

// If you wanted, you could instead add a component to the entity and recursivley to its children.
// That component acting as a flag for another system to despawn the entities.
// That way, stuff can respond to the imminent despawn.
// They can still do that here though anyways, just pay attention to the death bang.
pub fn referenced_entity_destruction_on_death_sys(
    q: Query<(&ToDespawnTarget, &DeathBang), (Changed<DeathBang>, With<DeathToEntityDespawn>, With<DespawnTargetIsReference>)>,
    mut commands: Commands
) {
    for (target, bang) in q.iter() {
        if !bang.0 {
            continue;
        }

        let target = target.go();
        let Some(commands) = commands.get_entity(target) else {
            continue; // Invalid destruction target
        };
    
        commands.despawn_recursive();
    }
}
