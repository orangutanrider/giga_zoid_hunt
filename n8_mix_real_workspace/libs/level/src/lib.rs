use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub const BOUNDS: Vec2 = Vec2::new(6250.0, 5600.0);
pub const BOUNDS_COLOUR: Color = Color::BLACK;
pub const BOUNDS_WIDTH: f32 = 10000.0;
pub const BOUNDS_COLLIDER_SCALE: f32 = 0.5;

#[derive(Bundle, Default)]
pub struct BoundsBundle {
    pub sprite: SpriteBundle,

    pub collider: Collider,
    pub grouping: CollisionGroups,
    pub rigidbody: RigidBody,
}


pub fn spawn_bounds(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>, 
) {
    let texture: Handle<Image> = asset_server.load("sprite\\primitive\\1px_square.png");

    let y_collider_size = Vec2::new(BOUNDS.x + BOUNDS_WIDTH, BOUNDS_WIDTH) * BOUNDS_COLLIDER_SCALE;
    // Y+
    commands.spawn((
        BoundsBundle{
            sprite: SpriteBundle{
                sprite: Sprite { 
                    color: BOUNDS_COLOUR, 
                    custom_size: Some(Vec2::new(BOUNDS.x + BOUNDS_WIDTH, BOUNDS_WIDTH)), 
                    ..Default::default() 
                },
                transform: Transform {
                    translation: Vec3::new(0.0, BOUNDS.y, 100.0),
                    ..Default::default()
                },
                texture: texture.clone_weak(),
                ..Default::default()
            },
            collider: Collider::cuboid(y_collider_size.x, y_collider_size.y),
            grouping: rapier_config::BOUNDS_CGROUP,
            rigidbody: RigidBody::Fixed,
        },
    ));

    // Y-
    commands.spawn((
        BoundsBundle{
            sprite: SpriteBundle{
                sprite: Sprite { 
                    color: BOUNDS_COLOUR, 
                    custom_size: Some(Vec2::new(BOUNDS.x + BOUNDS_WIDTH, BOUNDS_WIDTH)), 
                    ..Default::default() 
                },
                transform: Transform {
                    translation: Vec3::new(0.0, -BOUNDS.y, 100.0),
                    ..Default::default()
                },
                texture: texture.clone_weak(),
                ..Default::default()
            },
            collider: Collider::cuboid(y_collider_size.x, y_collider_size.y),
            grouping: rapier_config::BOUNDS_CGROUP,
            rigidbody: RigidBody::Fixed,
        },
    ));

    let x_collider_size = Vec2::new(BOUNDS_WIDTH, BOUNDS.y + BOUNDS_WIDTH) * BOUNDS_COLLIDER_SCALE;
    // X+
    commands.spawn((
        BoundsBundle{
            sprite: SpriteBundle{
                sprite: Sprite { 
                    color: BOUNDS_COLOUR, 
                    custom_size: Some(Vec2::new(BOUNDS_WIDTH, BOUNDS.y + BOUNDS_WIDTH)), 
                    ..Default::default() 
                },
                transform: Transform {
                    translation: Vec3::new(BOUNDS.x, 0.0, 100.0),
                    ..Default::default()
                },
                texture: texture.clone_weak(),
                ..Default::default()
            },
            collider: Collider::cuboid(x_collider_size.x, x_collider_size.y),
            grouping: rapier_config::BOUNDS_CGROUP,
            rigidbody: RigidBody::Fixed,
        },
    ));

    // X-
    commands.spawn((
        BoundsBundle{
            sprite: SpriteBundle{
                sprite: Sprite { 
                    color: BOUNDS_COLOUR, 
                    custom_size: Some(Vec2::new(BOUNDS_WIDTH, BOUNDS.y + BOUNDS_WIDTH)), 
                    ..Default::default() 
                },
                transform: Transform {
                    translation: Vec3::new(-BOUNDS.x, 0.0, 100.0),
                    ..Default::default()
                },
                texture,
                ..Default::default()
            },
            collider: Collider::cuboid(x_collider_size.x, x_collider_size.y),
            grouping: rapier_config::BOUNDS_CGROUP,
            rigidbody: RigidBody::Fixed,
        },
    ));
}