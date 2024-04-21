pub mod chase; use defend::*;
pub mod defend; use chase::*;
pub mod wildcard; use wildcard::*;
pub mod params; pub use params::*;

use bevy_rapier2d::prelude::*;
use bevy::prelude::*;

use rapier_config::*;
use ref_caravan::*;
use ref_marks::*;
use ref_paths::*;
use behaviour_tree::prelude::*;

use rts_unit_control::prelude::*;
use rts_unit_soul::*;
use rts_unit_team::*;
use rts_unit_nav::*;
use rts_unit_health::*;
use rts_unit_detectors::prelude::*;
use rts_direct_attack::*;
use rts_unit_death::*;
use rts_unit_movers::*;
use death_flare::*;
use health_to_death::*;
use attack_laser::*;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        // Chase
        app.add_systems(Update,(
            chase_to_body_movement_sys,
            chase_head_movement_sys,
            chase_attack_sys,
            death_spike_decay_sys,
            chase_factor_sys,
            chase_target_selection_sys,
            chase_frenzy_to_colour,
            chase_neck_sys,
        ));

        // Defend
        app.add_systems(Update,(
            defend_attack_sys,
            defend_factor_sys,
            defend_target_selection_sys,
            defend_head_movement_sys,
            defend_factor_damaged_decay,
            defend_frenzy_to_colour,
            defend_to_body_movement_sys,
            defend_neck_sys,
        ));

        // Wildcard
        app.add_systems(Update,(
            wildcard_waypoint_sys,
            wildcard_frenzy_sys,
            wildcard_head_movement_sys,
            wildcard_to_body_movement_sys,
            head_motif_rotate_sys,

            wildcard_persona_deciding_sys,
            persona_duration_sys,
            frenzy_override_sys,
            persona_frenzy_sys,
            persona_switching_activation_sys,
            wildcard_neck_sys,
            wildcard_frenzy_to_colour,
        ));
        app.add_systems(Update,(
            wc_persona_chase_target_selection_sys,
            wc_persona_chase_head_movement_sys,
            wc_persona_chase_attack_sys,
            wc_persona_chase_to_body_movement_sys,
        ));
        app.add_systems(Update,(
            wc_persona_defend_target_selection_sys,
            wc_persona_defend_head_movement_sys,
            wc_persona_defend_attack_sys,
            wc_persona_defend_to_body_movement_sys,
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
    pub velocity: Velocity,

    // Mover
    pub move_terminal: TMoveDecider,
    pub move_process: LocalVelocityMovement,
    pub speed: MoveSpeed,
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

    // Defend detection
    pub aggregate: TIntersectionsAggregate,
    pub detector: CircleIntersectionsOfPlayer,
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
            rigidbody: RigidBody::KinematicVelocityBased,
            grouping: RTS_UNIT_PHYSICS_BODY_CGROUP,
            speed: MoveSpeed::new(MOVE_SPEED),
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
            detector: CircleIntersectionsOfPlayer::new(DEFEND_SAFE_SPACE_RADIUS),
            death_flare: DeathFlareOnDeath{
                color: DEATH_FLARE_COLOUR,
                fade: DEATH_FLARE_FADE,
                width: DEATH_FLARE_WIDTH,
            },
            ..Default::default()
        },
        SpriteBundle {
            texture: texture.clone_weak(),
            transform: Transform { translation: HUB_OFFSET, ..Default::default()},
            sprite: Sprite { custom_size: Some(HUB_SIZE), color: HUB_COLOUR, ..Default::default() },
            ..Default::default()
        },
    )).id();

    let chase = commands.spawn((
        BundChase{
            to_mover: ToMover::new(root),
            to_hub: ToHub(hub),
            damage: DirectAttackPower::new(ATTACK_DAMAGE),
            laser: LaserVisualsOnAttack::new(CHASE_LASER_COLOUR, CHASE_LASER_FADE, CHASE_LASER_WIDTH),
            speed: MoveSpeed::new(CHASE_MOVE_SPEED),
            ..Default::default()
        },
        SpriteBundle{
            texture: texture.clone_weak(),
            transform: Transform { translation: CHASE_OFFSET, ..Default::default()},
            sprite: Sprite { custom_size: Some(CHASE_SIZE), color: CHASE_COLOUR, ..Default::default() },
            ..Default::default()
        }
    )).id();

    let defend = commands.spawn((
        BundDefend{
            to_mover: ToMover::new(root),
            to_hub: ToHub(hub),
            damage: DirectAttackPower::new(ATTACK_DAMAGE),
            laser: LaserVisualsOnAttack::new(DEFEND_LASER_COLOUR, DEFEND_LASER_FADE, DEFEND_LASER_WIDTH),
            speed: MoveSpeed::new(DEFEND_MOVE_SPEED),
            ..Default::default()
        },
        SpriteBundle{
            texture: texture.clone_weak(),
            transform: Transform { translation: DEFEND_OFFSET, ..Default::default()},
            sprite: Sprite { custom_size: Some(DEFEND_SIZE), color: DEFEND_COLOUR, ..Default::default() },
            ..Default::default()
        }
    )).id();

    commands.entity(hub).insert((
        ToDefend::new(defend),
        ToChase::new(chase),
    ));

    let motif = commands.spawn((
        Motif,
        SpriteBundle{
            texture: texture.clone_weak(),
            transform: Transform { translation: MOTIF_OFFSET, ..Default::default()},
            sprite: Sprite { 
                custom_size: Some(MOTIF_SIZE), 
                color: MOTIF_COLOUR, 
                anchor: bevy::sprite::Anchor::Custom(MOTIF_PIVOT),
                ..Default::default() 
            },
            ..Default::default()
        }
    )).id();

    let wildcard = commands.spawn((
        BundWildcard{
            to_motif: ToMotif::new(motif),
            to_mover: ToMover::new(root),
            to_hub: ToHub(hub),
            speed: MoveSpeed::new(WILDCARD_MOVE_SPEED),
            laser: LaserVisualsOnAttack::new(WILDCARD_LASER_COLOUR, WILDCARD_LASER_FADE, WILDCARD_LASER_WIDTH),
            damage: DirectAttackPower::new(ATTACK_DAMAGE),
            ..Default::default()
        },
        SpriteBundle{
            texture: texture,
            transform: Transform { translation: WILDCARD_OFFSET, ..Default::default()},
            sprite: Sprite { custom_size: Some(WILDCARD_SIZE), color: WILDCARD_COLOUR, ..Default::default() },
            ..Default::default()
        }
    )).id();

    let texture: Handle<Image> = asset_server.load("sprite\\primitive\\1px_square.png");

    let chase_neck = commands.spawn((
        ChaseNeck{
            hub,
            chase,
        },
        SpriteBundle{
            texture: texture.clone_weak(),
            transform: Transform { translation: CHASE_NECK_OFFSET, ..Default::default()},
            sprite: Sprite { color: CHASE_COLOUR, ..Default::default() },
            ..Default::default()
        }
    )).id();

    let defend_neck = commands.spawn((
        DefendNeck{
            hub,
            defend,
        },
        SpriteBundle{
            texture: texture.clone_weak(),
            transform: Transform { translation: DEFEND_NECK_OFFSET, ..Default::default()},
            sprite: Sprite { color: DEFEND_COLOUR, ..Default::default() },
            ..Default::default()
        }
    )).id();

    let wildcard_neck = commands.spawn((
        WildcardNeck{
            hub,
            wildcard,
        },
        SpriteBundle{
            texture: texture,
            transform: Transform { translation: WILDCARD_NECK_OFFSET, ..Default::default()},
            sprite: Sprite { color: WILDCARD_COLOUR, ..Default::default() },
            ..Default::default()
        }
    )).id();

    commands.entity(hub).insert(AdditionalDespawnTargets(vec![chase_neck, defend_neck, wildcard_neck]));

    commands.entity(root).add_child(tree_root);
    commands.entity(tree_root).add_child(hub);
    commands.entity(hub).push_children(&[chase, defend, wildcard]);
    
    commands.entity(wildcard).add_child(motif);
}