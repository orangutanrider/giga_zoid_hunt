use bevy::prelude::*;
use crate::*;

#[derive(Component)]
pub struct DistillationForClosest(Option<Entity>);
impl Default for DistillationForClosest {
    fn default() -> Self {
        Self(None)
    }
}
impl DistillationColumn for DistillationForClosest {
    fn read_detection(&self) -> Option<Entity> {
        return self.0
    }

    fn distiller_set(&mut self, v: Option<Entity>) {
        self.0 = v;
    }
}

pub fn closest_detection_distillation_sys(
    mut q: Query<(&mut DistillationForClosest, &GlobalTransform, &TIntersectionsAggregate)>,
    aggregate_q: Query<&GlobalTransform>
) {
    for (column, transform, aggregate) in q.iter_mut() {
        let local_position = transform.translation().truncate();
        let mut closest: Option<Entity> = None;
        let mut distance = f32::MAX;
        let distillation_logic = |agg| -> Option<Entity> {
            let Ok(agg_transform) = aggregate_q.get(agg) else {
                return closest;
            };

            let agg_distane = agg_transform.translation().truncate().distance(local_position);
            if agg_distane < distance {
                closest = Some(agg);
                distance = agg_distane;
            }
            return closest;
        };

        distill(column, aggregate, distillation_logic);
    }
}