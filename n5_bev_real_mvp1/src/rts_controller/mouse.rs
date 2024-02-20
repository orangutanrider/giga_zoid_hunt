use bevy::prelude::*;
use bevy::ecs::system::SystemParam;
use bevy_cursor::CursorLocation;

#[derive(SystemParam)]
pub struct RtsMouse<'w> {
    position: Res<'w, CursorLocation>,
}
impl<'w> RtsMouse<'w> {
    pub fn position(&self) -> Vec2 {
        let val = self.position.world_position();
        match val {
            Some(vec2) => return vec2,
            None => return Vec2::ZERO, // Could change it to hold the last known position and use that instead
        }
    }
}