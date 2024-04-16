pub mod chase;
pub mod params; use attack_laser::LaserVisualsOnAttack;
pub use params::*;

use bevy_rapier2d::prelude::*;
use bevy::prelude::*;

use chase::BundChase;
use rapier_config::RTS_UNIT_PHYSICS_BODY_CGROUP;
use ref_caravan::*;
use ref_marks::*;
use ref_paths::*;

use behaviour_tree::prelude::*;

use rts_unit_control::commandable::orders::attack_target::processing::TargetedBy;
use rts_unit_soul::ENEMY_SOUL_CGROUP;
use rts_unit_team::EnemyTeam;

use rts_unit_nav::*;
use rts_unit_health::*;
use rts_unit_detectors::prelude::*;
use rts_direct_attack::*;
use rts_unit_death::*;
use rts_unit_movers::*;
use death_flare::*;
use health_to_death::*;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update,(
            chase_to_body_movement_sys,
            chase::head_movement_sys,
            chase_attack_sys,
            chase::rotate_to_face_target_sys,
            death_spike_decay_sys,
            chase_factor_sys,
            chase_target_selection_sys,
        ));
    }
}

#[derive(Component, Default)]
pub struct Root;
#[derive(Bundle, Default)]
struct BRoot {
    pub team: EnemyTeam,
    pub flag: Root,

    // Physics body
    pub collider: Collider,
    pub rigidbody: RigidBody,
    pub grouping: CollisionGroups,

    // Mover
    pub move_terminal: TMoveAggregator,
    pub move_process: LocalTransformMovement,
}

#[derive(Component, Default)]
struct TreeRoot;
ref_signature!(TreeRoot);
#[derive(Bundle, Default)]
struct BTreeRoot {
    pub flag: TreeRoot,
    pub tree_bang: RootBang,
    pub reset_bang: ResetBang,
    pub export_bang: ExportBang,
}

#[derive(Component)]
pub struct ToHub(Entity);
waymark!(ToHub);

#[derive(Component, Default)]
pub struct Hub;
ref_signature!(Hub);
#[derive(Bundle, Default)]
struct BHub {
    pub flag: Hub,

    // Tree
    pub bang: Bang,
    pub state: TState,

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

pub fn spawn_enemy(
    location: Vec2,
    commands: &mut Commands,
    asset_server: &Res<AssetServer>, 
) {
    let texture: Handle<Image> = asset_server.load("sprite\\primitive\\64px_square.png");

    // Root
    let root = commands.spawn((
        BRoot{
            collider: Collider::ball(PHYSICS_SIZE),
            rigidbody: RigidBody::KinematicPositionBased,
            grouping: RTS_UNIT_PHYSICS_BODY_CGROUP,
            ..Default::default()
        },
        SpriteBundle {
            texture: texture.clone_weak(),
            transform: Transform { translation: location.extend(0.0), ..Default::default()},
            sprite: Sprite { custom_size: Some(ROOT_SIZE), color: ROOT_COLOUR, ..Default::default() },
            ..Default::default()
        }
    )).id();

    // Tree Root
    let tree_root = commands.spawn((
        BTreeRoot{
            ..Default::default()
        },
        SpriteBundle {
            texture: texture.clone_weak(),
            transform: Transform { translation: TREE_ROOT_OFFSET, ..Default::default()},
            sprite: Sprite { custom_size: Some(TREE_ROOT_SIZE), color: TREE_ROOT_COLOUR, ..Default::default() },
            ..Default::default()
        }
    )).id();

    // Hub
    let hub = commands.spawn((
        BHub{
            collider: Collider::ball(BODY_SIZE),
            rigidbody: RigidBody::Fixed,
            sensor: Sensor,
            grouping: ENEMY_SOUL_CGROUP,
            health: THealth(HEALTH),
            max_health: MaxHealth::new(HEALTH),
            to_despawn_target: ToDespawnTarget::new(root),
            ..Default::default()
        },
        SpriteBundle {
            texture: texture.clone_weak(),
            transform: Transform { translation: HUB_OFFSET.extend(0.0), ..Default::default()},
            sprite: Sprite { custom_size: Some(HUB_SIZE), color: HUB_COLOUR, ..Default::default() },
            ..Default::default()
        },
    )).id();

    let chase = commands.spawn((
        BundChase{
            to_mover: ToMover::new(root),
            to_hub: ToHub(hub),
            damage: DirectAttackPower::new(ATTACK_DAMAGE),
            laser: LaserVisualsOnAttack::new(LASER_COLOUR, LASER_FADE, LASER_WIDTH),
            ..Default::default()
        },
        SpriteBundle{
            texture: texture.clone_weak(),
            transform: Transform { translation: CHASE_OFFSET.extend(0.0), ..Default::default()},
            sprite: Sprite { custom_size: Some(CHASE_SIZE), color: CHASE_COLOUR, ..Default::default() },
            ..Default::default()
        }
    )).id();

    commands.entity(root).add_child(tree_root);
    commands.entity(tree_root).add_child(hub);
    commands.entity(hub).push_children(&[chase]);
}

// Heads to body movement
// Aggregate Movement
use chase::*;
fn chase_to_body_movement_sys(
    chase_q: Query<(&ToMover, &ChaseTarget, &ChaseFactor, &GlobalTransform, Entity), With<Chase>>,
    target_q: Query<&GlobalTransform>,
    mut root_q: Query<&mut TMoveAggregator>,
) {
    for (to_mover, target, chase, head, chase_entity) in chase_q.iter() {
        // Get
        let head = head.translation().truncate();

        let target = target.read();
        let Ok(target) = target_q.get(target) else { continue; };
        let target = target.translation().truncate();

        let chase = chase.read();
        let chase_prevelance = chase * CHASE_SCALAR;
        let chase_move = (head - target).normalize_or_zero() * CHASE_POWER * CHASE_BODY_MOVE;

        // Set
        let hub = to_mover.go();
        let Ok(mut body) = root_q.get_mut(hub) else { continue; };

        use rts_unit_movers::Key as MoveKey;
        body.inputs.insert(MoveKey::External(chase_entity), (chase_move, chase_prevelance));
    }
}