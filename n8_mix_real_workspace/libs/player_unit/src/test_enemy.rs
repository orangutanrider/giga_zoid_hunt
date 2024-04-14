use super::*;

#[derive(Bundle, Default)]
pub struct TestEnemy {
    pub sprite: SpriteBundle,

    // Body/Soul
    pub collider: Collider,
    pub rigidbody: RigidBody,
    pub sensor: Sensor,
    pub grouping: CollisionGroups,

    pub at_here_targeting: TargetedBy,

    // Mortality
    pub health: THealth,
    pub max_health: MaxHealth,
    pub health_to_death: ZeroHealthMeansDeath,
    pub death_is_local: DeathIsLocal,
    pub health_is_local: HealthIsLocal,
    pub death: DeathBang,
    pub death_to_despawn: DeathToEntityDespawn,
    pub despawn_is_ref: DespawnTargetIsReference,
    pub to_despawn_target: ToDespawnTarget,
    pub team_affiliation: EnemyTeam,
    pub death_flare: DeathFlareOnDeath,
}

pub fn spawn_test_enemy(
    location: Vec2,
    commands: &mut Commands,
    asset_server: &Res<AssetServer>, 
) {
    let square: Handle<Image> = asset_server.load("sprite\\primitive\\64px_square.png");

    let root = commands.spawn((
        TransformBundle{
            local: Transform::from_translation(location.extend(0.0)),
            ..Default::default()
        },
        InheritedVisibility::VISIBLE,
    )).id();

    let sprite = SpriteBundle{
        transform: Transform{
            translation: location.extend(0.0),
            rotation: Quat::from_euler(EulerRot::XYZ, 0.0, 0.0, 45.0),
            ..Default::default()
        },
        texture: square,
        sprite: Sprite { color: Color::BLACK, ..Default::default() },
        ..Default::default()
    };

    let body = commands.spawn((
        TestEnemy{
            sprite,

            collider: Collider::ball(32.0),
            rigidbody: RigidBody::Fixed,
            sensor: Sensor,
            grouping: ENEMY_SOUL_CGROUP,

            health: THealth(10.0),
            max_health: MaxHealth::new(10.0),
            to_despawn_target: ToDespawnTarget::new(root),
            ..Default::default()
        }
    )).id();

    commands.entity(root).add_child(body);
}