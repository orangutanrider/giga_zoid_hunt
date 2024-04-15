use bevy::prelude::*;
use rts_direct_attack::*;

pub struct LaserVisualsPlugin;

impl Plugin for LaserVisualsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostUpdate,(
            laser_visuals_sys,
            spawn_laser_visuals_sys
        ));
    }
}

#[derive(Component)] // Could be replaced by a named fade component
pub struct LaserVisuals{
    fade: f32,
}
impl LaserVisuals {
    pub fn new(
        fade: f32
    ) -> Self {
        return Self {
            fade,
        }
    }
}

#[derive(Component)]
pub struct LaserVisualsOnAttack { // Could probably be replaced by a spawn on attack thing
    color: Color,
    fade: f32,
    width: f32,
}
impl Default for LaserVisualsOnAttack {
    fn default() -> Self {
        Self { color: Default::default(), fade: Default::default(), width: Default::default() }
    }
}
impl LaserVisualsOnAttack {
    pub fn new(
        color: Color,
        fade: f32,
        width: f32,
    ) -> Self {
        return Self {
            color,
            fade,
            width,
        }
    }
}

pub fn laser_visuals_sys(
    mut q: Query<(Entity, &mut Sprite, &mut Transform, &LaserVisuals)>,
    mut commands: Commands,
    time: Res<Time>,
) {
    for (entity, mut sprite, mut transform, laser) in q.iter_mut() {
        let alpha = sprite.color.a();

        if alpha <= 0.01 {
            commands.entity(entity).despawn();
            continue;
        }

        let time_adjusted_fade = laser.fade * time.delta_seconds();
        sprite.color.set_a(alpha - time_adjusted_fade);
        transform.scale = Vec3::new(
            transform.scale.x, 
            transform.scale.y - time_adjusted_fade,
            transform.scale.z
        );
    }
}


// Commands could be extracted to a detector and a creation system
pub fn spawn_laser_visuals_sys(
    q: Query<(&DirectAttackBang, &LaserVisualsOnAttack, &GlobalTransform), Changed<DirectAttackBang>>,
    target_q: Query<&GlobalTransform>,
    mut commands: Commands,
    asset_server: Res<AssetServer>, 
) {
    for (attack, laser, origin) in q.iter() {
        let Some(target) = attack.read() else {
            continue;
        };
        let Ok(target) = target_q.get(target) else {
            continue;
        };

        let origin = origin.translation().truncate();
        let target = target.translation().truncate();
        spawn_laser_visuals(origin, target, laser, &mut commands, &asset_server);
    }
}

#[derive(Bundle)]
pub struct LaserBundle {
    pub laser: LaserVisuals,
    pub sprite: SpriteBundle
}

pub fn spawn_laser_visuals(
    origin: Vec2,
    target: Vec2,
    params: &LaserVisualsOnAttack,
    commands: &mut Commands,
    asset_server: &Res<AssetServer>, 
) {
    // position = diff as
    // rotation = from origin point to target
    // scale = (width, distance) 

    let texture: Handle<Image> = asset_server.load("sprite\\primitive\\1px_square.png");

    let distance = origin.distance(target);
    let diff = target - origin;
    let direction = diff.normalize();

    let translation = (origin + (direction * distance * 0.5)).extend(-0.5);
    let rotation = Quat::from_rotation_z(direction.to_angle());

    let scale = Vec3::new(distance, params.width, 1.0);

    commands.spawn(LaserBundle{
        laser: LaserVisuals::new(params.fade),
        sprite: SpriteBundle{
            sprite: Sprite { color: params.color, ..Default::default() },
            transform: Transform {
                translation,
                rotation,
                scale,
            },
            texture,
            ..Default::default()
        },
    });
}