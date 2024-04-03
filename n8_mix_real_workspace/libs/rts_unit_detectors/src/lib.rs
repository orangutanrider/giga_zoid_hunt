pub mod enemy_circle_intersections;
pub mod closest;

use bevy::{ecs::system::SystemParam, prelude::*};
use bevy_rapier2d::prelude::*;

pub struct DetectorsPlugin;
impl Plugin for DetectorsPlugin {
    fn build(&self, app: &mut App) {
        
    }
}

#[derive(Component)]
pub struct TIntersectionsAggregate(pub Vec<Entity>);

pub trait ImmutableDetector{
    const FILTER: QueryFilter<'static>;
    fn shape(&self) -> Collider;
}

/* 
macro_rules! immutable_detector {(...) => {
    ...
};}
*/

pub trait DistilledDetection{
    fn detection(&self) -> Option<Entity>;
}

pub trait DetectionEdit{
    fn set(&mut self, v: Option<Entity>);
}

pub trait DistillationLogic{
    fn logic(old: &Option<Entity>, new: &Entity) -> bool;
}

pub fn intersections_distillation_sys<T, Logic>( 
    mut q: Query<(&mut T, & TIntersectionsAggregate)>,
) where 
    T: DistilledDetection + DetectionEdit + Component,
    Logic: DistillationLogic
{
    for (mut distilled, aggregate) in q.iter_mut() {
        let mut distillation: Option<Entity> = None;
        for detection in aggregate.0.iter() {
            if Logic::logic(&distillation, detection) {
                distillation = Some(*detection);
            }
        }
        distilled.set(distillation);
    }
}

// There was an idea with this, seems to make it more complicated though.
// Maybe instead of generic, a macro? Just macro rules with an expression in it? 
/* 
pub fn intersections_distillation_sys<T, Logic>( 
    mut q: Query<(&mut T, & TIntersectionsAggregate)>,
    logic: Logic
) where 
    T: DistilledDetection + DetectionEdit + Component,
    Logic: Fn(&Option<Entity>, &Entity) -> bool + SystemParam // entiy1 = old, entity2 = new
{
    for (mut distilled, aggregate) in q.iter_mut() {
        let mut distillation: Option<Entity> = None;
        for detection in aggregate.0.iter() {
            if logic(&distillation, detection) {
                distillation = Some(*detection);
            }
        }
        distilled.set(distillation);
    }
}
*/

// This is the part where the idea fell apart
/* 
pub struct ClosestLogic;
impl Fn(&Option<Entity>, &Entity) -> bool for ClosestLogic {
    extern "rust-call" fn call(&self, args: Args) -> Self::Output {
        ...
    }
}
impl SystemParam for ClosestLogic {
    type State;

    type Item<'world, 'state>;

    fn init_state(world: &mut World, system_meta: &mut bevy::ecs::system::SystemMeta) -> Self::State {
        ...
    }

    unsafe fn get_param<'world, 'state>(
        state: &'state mut Self::State,
        system_meta: &bevy::ecs::system::SystemMeta,
        world: bevy::ecs::world::unsafe_world_cell::UnsafeWorldCell<'world>,
        change_tick: bevy::ecs::component::Tick,
    ) -> Self::Item<'world, 'state> {
        ...
    }
}
*/