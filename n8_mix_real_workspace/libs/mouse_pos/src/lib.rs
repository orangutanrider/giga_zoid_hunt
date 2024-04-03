use bevy::prelude::*;

pub struct CursorTrackingPlugin;
impl Plugin for CursorTrackingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, cursor_tracking_sys);
    }
}

#[derive(Resource)]
pub struct CursorWorldPos(Vec2);
impl CursorWorldPos {
    pub fn pos(&self) -> Vec2 {
        return self.0
    }
}
impl Default for CursorWorldPos {
    fn default() -> Self {
        Self(Vec2::ZERO)
    }
}

#[derive(Resource)]
pub struct CursorIsLocated(bool);
impl Default for CursorIsLocated {
    fn default() -> Self {
        Self(false)
    }
}
impl CursorIsLocated {
    pub fn is_located(&self) -> bool {
        return self.0
    }
}

#[derive(Component)]
pub struct MainCamera;
impl Default for MainCamera {
    fn default() -> Self {
        Self {  }
    }
}
impl MainCamera {
    pub fn new() -> Self {
        Self {  }
    }
}

/// https://github.com/bevyengine/bevy/discussions/7970#discussioncomment-5241020
pub fn cursor_tracking_sys(
    mut world_pos: ResMut<CursorWorldPos>,
    mut is_located: ResMut<CursorIsLocated>,
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    let window = windows.single();
    let (camera, camera_transform) = camera_q.single();

    if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor))
    {
        world_pos.0 = world_position;
        is_located.0 = true;
    } else {
        is_located.0 = false;
    }
}