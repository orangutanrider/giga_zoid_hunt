use bevy::prelude::*;
use rts_unit_death::*;
 
pub struct DeathFlarePlugin;
impl Plugin for DeathFlarePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            death_flare_on_death_sys,
            death_flare_sys,
        ));
    }
}

#[derive(Component, Default)]
/// Creates a visual flare when a unit dies.
pub struct DeathFlareOnDeath{
    pub color: Color,
    pub fade: f32,
    pub width: f32,
}
pub const DEATH_FLARE_Z: f32 = 3.0;

// Could be re-created as a composition, rather than all the functionality being derived from DeathFlare.
#[derive(Component)]
pub struct DeathFlare(pub f32);

#[derive(Bundle)]
pub struct BundDeathFlare {
    pub flag: DeathFlare,
    pub sprite: SpriteBundle,
}

// Better to use an event handler for spawning the flare than directly using commands, if upgrading this do that.
pub fn death_flare_on_death_sys(
    q: Query<(&DeathBang, &GlobalTransform, &DeathFlareOnDeath), (Changed<DeathBang>)>,
    mut commands: Commands,
    asset_server: Res<AssetServer>, 
) {
    for (death, transform, flare) in q.iter() {
        if !death.is_active() {
            continue;
        }

        let location = transform.translation().truncate();
        spawn_death_flare(location, &mut commands, &asset_server, flare)
    }
}

pub fn death_flare_sys(
    mut q: Query<(Entity, &mut Sprite, &DeathFlare)>,
    mut commands: Commands,
    time: Res<Time>,
) {
    for (entity, mut sprite, flare) in q.iter_mut() {
        let alpha = sprite.color.a();

        if alpha <= 0.01 {
            commands.entity(entity).despawn();
            continue;
        }

        let time_adjusted_fade = flare.0 * time.delta_seconds();
        sprite.color.set_a(alpha - time_adjusted_fade);
    }
}

pub fn spawn_death_flare(
    location: Vec2,
    commands: &mut Commands, 
    asset_server: &Res<AssetServer>, 
    flare: &DeathFlareOnDeath,
) {
    let texture: Handle<Image> = asset_server.load("sprite\\primitive\\64px_square.png");

    commands.spawn(BundDeathFlare{
        flag: DeathFlare(flare.fade),
        sprite: SpriteBundle { 
            sprite: Sprite{
                color: flare.color, ..Default::default()
            }, 
            transform: Transform { 
                translation: location.extend(DEATH_FLARE_Z), 
                scale: Vec3::new(flare.width, 90.0, 1.0), 
                ..Default::default() 
            }, 
            texture, 
            ..Default::default()
        },
    });
}
