use bevy::prelude::*;
use crate::*;

#[derive(Component)]
pub struct ClosestDetection(Option<Entity>);
impl DistilledDetection for ClosestDetection {
    fn detection(&self) -> Option<Entity> {
        return self.0
    }
}
impl DetectionEdit for ClosestDetection {
    fn set(&mut self, v: Option<Entity>) {
        self.0 = v;
    }
}

fn closest_logic(old: &Option<Entity>, new: &Entity) -> bool {
    false
}

pub struct ClosestLogic;
impl Fn(&Option<Entity>, &Entity) -> bool for ClosestLogic {
    extern "rust-call" fn call(&self, args: Args) -> Self::Output {
        
    }
}

pub struct ClosestPlugin;
impl Plugin for DetectorsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostUpdate, intersections_distillation_sys::<ClosestDetection, closest_logic>);
    }
}