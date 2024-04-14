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
pub struct DeathFlareOnDeath;
pub const DEATH_FLARE_Z: f32 = 3.0;
pub const DEATH_FLARE_SCALE: Vec3 = Vec3::new(0.66, 50.0, 1.0);
pub const FADE_SPEED: f32 = 1.0;

// Could be re-created as a composition, rather than all the functionality being derived from DeathFlare.
#[derive(Component)]
pub struct DeathFlare;

#[derive(Bundle)]
pub struct BundDeathFlare {
    pub flag: DeathFlare,
    pub sprite: SpriteBundle,
}

// Better to use an event handler for spawning the flare than directly using commands, if upgrading this do that.
pub fn death_flare_on_death_sys(
    q: Query<(&DeathBang, &GlobalTransform), (Changed<DeathBang>, With<DeathFlareOnDeath>)>,
    mut commands: Commands,
    asset_server: Res<AssetServer>, 
) {
    for (death, transform) in q.iter() {
        if !death.is_active() {
            continue;
        }

        let location = transform.translation().truncate();
        spawn_death_flare(location, &mut commands, &asset_server)
    }
}

pub fn death_flare_sys(
    mut q: Query<(Entity, &mut Sprite), With<DeathFlare>>,
    mut commands: Commands,
    time: Res<Time>,
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

pub fn spawn_death_flare(
    location: Vec2,
    commands: &mut Commands, 
    asset_server: &Res<AssetServer>, 
) {
    let texture: Handle<Image> = asset_server.load("sprite\\primitive\\64px_square.png");

    commands.spawn(BundDeathFlare{
        flag: DeathFlare,
        sprite: SpriteBundle { 
            sprite: Sprite{
                color: Color::PURPLE, ..Default::default()
            }, 
            transform: Transform { translation: location.extend(DEATH_FLARE_Z), scale: DEATH_FLARE_SCALE, ..Default::default() }, 
            texture, 
            ..Default::default()
        },
    });
}
