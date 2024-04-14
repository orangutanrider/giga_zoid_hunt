use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::rapier::PhysicsQueries;
use super::{
    attack::AttackInput,
    pure_move::PureMoveInput,
    //stop::StopInput,
};

// For the behaviour of these visuals (fading out and self-destructing) it could be fully composition based.
// Also that'd probably help performance, cause I don't think it is efficient to use commands in a ton of systems like this.

#[derive(Component, Default)]
pub struct AttackMoveVisual;

#[derive(Bundle)]
pub struct BundAttackMoveVisual {
    pub flag: AttackMoveVisual,
    pub sprite: SpriteBundle,
}

#[derive(Component, Default)]
pub struct PureMoveVisual;

#[derive(Bundle)]
pub struct BundPureMoveVisual {
    pub flag: PureMoveVisual,
    pub sprite: SpriteBundle,
}
 
#[derive(Component)]
pub struct AttackTargetVisual(Entity);
impl AttackTargetVisual {
    pub fn new(target: Entity) -> Self {
        return Self(target)
    }
}

#[derive(Bundle)]
pub struct BundAttackTargetVisual {
    pub flag: AttackTargetVisual,
    pub sprite: SpriteBundle,
}

const FADE_SPEED: f32 = 2.0;
const GROUND_ORDER_Z: f32 = -1.0;
const GROUND_ORDER_SCALE: Vec3 = Vec3::new(0.66, 0.33, 1.0);
const TARGET_ORDER_Z: f32 = 1.0;

pub fn pure_move_visual_sys(
    mut q: Query<(Entity, &mut Sprite), With<PureMoveVisual>>,
    mut commands: Commands,
    time: Res<Time>
) {
    for (entity, mut sprite) in q.iter_mut() {
        let alpha = sprite.color.a();

        if alpha <= 0.01 {
            commands.entity(entity).despawn();
            continue;
        }

        let time_adjusted_fade = FADE_SPEED * time.delta_seconds();
        sprite.color.set_a(alpha - time_adjusted_fade);
    }
}

pub fn attack_move_visual_sys(
    mut q: Query<(Entity, &mut Sprite), With<AttackMoveVisual>>,
    mut commands: Commands,
    time: Res<Time>
) {
    for (entity, mut sprite) in q.iter_mut() {
        let alpha = sprite.color.a();

        if alpha <= 0.01 {
            commands.entity(entity).despawn();
            continue;
        }

        let time_adjusted_fade = FADE_SPEED * time.delta_seconds();
        sprite.color.set_a(alpha - time_adjusted_fade);
    }
}

pub fn attack_target_visual_sys(
    mut q: Query<(Entity, &mut Sprite, &AttackTargetVisual, &mut Transform)>,
    target_q: Query<&GlobalTransform>,
    mut commands: Commands,
    time: Res<Time>
) {
    for (entity, mut sprite, to_target, mut transform) in q.iter_mut() {
        // Target track
        let Ok(target_transform) = target_q.get(to_target.0) else {
            commands.entity(entity).despawn();
            continue;  
        };

        transform.translation = target_transform.translation();
        
        // Colour fade
        let alpha = sprite.color.a();

        if alpha <= 0.01 {
            commands.entity(entity).despawn();
            continue;
        }

        let time_adjusted_fade = FADE_SPEED * time.delta_seconds();
        sprite.color.set_a(alpha - time_adjusted_fade);
    }
}

pub fn create_attack_visuals_sys(
    input: AttackInput, 
    rapier: PhysicsQueries,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    if !input.just_pressed() {
        return;
    }

    let location = input.pos();
    match rapier.cast_for_e_attackable(location) {
        Some((target, _)) => create_attack_target_visuals(location, target, &mut commands, &asset_server),
        None => create_attack_move_visuals(location, &mut commands, &asset_server),
    }
}

/// Creates an entity that follows the target and then self destructs after a bit.
fn create_attack_target_visuals(
    location: Vec2,
    target: Entity,
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
) {
    let texture: Handle<Image> = asset_server.load("sprite\\primitive\\16px_corners.png");
    commands.spawn(BundAttackTargetVisual{
        flag: AttackTargetVisual(target),
        sprite: SpriteBundle { 
            sprite: Sprite{
                color: Color::RED,
                ..Default::default()
            }, 
            transform: Transform::from_translation(location.extend(TARGET_ORDER_Z)), 
            texture, 
            ..Default::default()
        },
    });
}

/// Creates an entity at the waypoint that self destructs after a bit.
fn create_attack_move_visuals(
    location: Vec2,
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
) {
    let texture: Handle<Image> = asset_server.load("sprite\\primitive\\64px_square.png");
    commands.spawn(BundAttackMoveVisual{
        flag: AttackMoveVisual,
        sprite: SpriteBundle { 
            sprite: Sprite{
                color: Color::ORANGE_RED,
                ..Default::default()
            }, 
            transform: Transform { translation: location.extend(GROUND_ORDER_Z), scale: GROUND_ORDER_SCALE, ..Default::default() }, 
            texture, 
            ..Default::default()
        },
    });
}

/// Creates an entity at the waypoint that self destructs after a bit.
pub fn create_pure_move_visuals_sys(
    input: PureMoveInput, 
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    if !input.just_pressed() {
        return;
    }

    let location = input.pos();
    let texture: Handle<Image> = asset_server.load("sprite\\primitive\\64px_square.png");
    commands.spawn(BundPureMoveVisual{
        flag: PureMoveVisual,
        sprite: SpriteBundle { 
            sprite: Sprite{
                color: Color::YELLOW,
                ..Default::default()
            }, 
            transform: Transform { translation: location.extend(GROUND_ORDER_Z), scale: GROUND_ORDER_SCALE, ..Default::default() }, 
            texture, 
            ..Default::default()
        },
    });
}