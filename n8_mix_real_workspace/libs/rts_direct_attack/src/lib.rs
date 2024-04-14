use bevy::prelude::*;

use rts_unit_health::*;

pub struct DirectAttackPlugin;

impl Plugin for DirectAttackPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, direct_attack_sys);
    }
}

#[derive(Component)]
pub struct DirectAttackBang(Option<Entity>);
impl Default for DirectAttackBang {
    fn default() -> Self {
        Self(None)
    }
}
impl DirectAttackBang {
    pub fn new() -> Self {
        return Self(None)
    }

    pub fn bang(&mut self, target: Entity) {
        self.0 = Some(target);
    }
}

#[derive(Component)]
pub struct DirectAttackPower(f32);
impl Default for DirectAttackPower {
    fn default() -> Self {
        Self(0.0)
    }
}
impl DirectAttackPower {
    pub fn new(power: f32) -> Self {
        return Self(power);
    }

    pub fn read(&self) -> f32 {
        return self.0
    }
}

// Ideally you upgrade this.
// Add a damage to health data handling point (probably damage as a component), but this is good enough for my purposes.

pub fn direct_attack_sys(
    mut q: Query<(&mut DirectAttackBang, &DirectAttackPower), Changed<DirectAttackBang>>,
    mut target_q: Query<&mut THealth>
) {
    for (mut bang, power) in q.iter_mut() {
        let Some(target) = bang.0 else {
            continue;
        };
        bang.bypass_change_detection();
        bang.0 = None;

        let Ok(mut target) = target_q.get_mut(target) else {
            continue;
        };

        target.0 = target.0 - power.0;
    }
}