use bevy::prelude::*;
use bevy::ecs::system::SystemParam;
use mouse_tracking::MousePosWorld;

#[derive(SystemParam)]
pub struct RtsMouse<'w> {
    position: Res<'w, MousePosWorld>,
}
impl<'w> RtsMouse<'w> {
    pub fn position(&self) -> Vec2 {
        return self.position.truncate();
    }
}