use super::*;

pub struct PlayerUnitPlugin;

impl Plugin for PlayerUnitPlugin {
    fn build(&self, app: &mut App) {
        // State to root
        app.add_systems(Update, (
            imca_mapper_sys,
            aggro_to_tree_root_sys,
            attack_closest_to_tree_root_sys,
            attack_target_to_tree_root_sys,
            detection_to_state_sys, 
        ));

        // Idle
        app.add_systems(Update, (
            idle_logic_sys,
            idle_actuator_sys,
        ));

        // Move
        app.add_systems(Update, (
            move_aggro_logic_sys,
            move_actuator_sys,
        ));

        // Chase
        app.add_systems(Update, (
            chase_logic_sys,
            chase_actuator_sys,
            referenced_aggro_to_referenced_nav_sys,
            bang_to_switched_aggro_to_nav,
        ));

        // Attack
        app.add_systems(Update, (
            attack_behav_sys,
            target_update_sys,
            attack_timer_reset_sys,
            attack_timer_sys,
            attack_execution_sys,
            attack_end_sys,
            attack_actuator_sys,
            attack_reset_sys,
        ));

        // Common
        app.add_systems(Update, (             
            refd_mover_is_zero_when_bang_sys,
        ));

        app.add_plugins((
            // State to root
            ControlOrdersToStatePlugin,

            // Move
            BMoveNavToMoverPlugin,
            BMoveControlToNavPlugin,

            // Chase
            BChaseNavToMoverPlugin,
            BChaseControlToNavPlugin
        ));
    }
}


#[derive(Event)]
pub struct SpawnPlayerUnitEvent(pub Vec2); // spawn location 

pub fn spawn_player_unit_event_sys(
    mut event: EventReader<SpawnPlayerUnitEvent>,
    mut commands: Commands,
    asset_server: Res<AssetServer>, 
) {
    for ev in event.read() {
        spawn_player_unit(ev.0, &mut commands, &asset_server);
    }
}

pub fn spawn_player_unit(
    location: Vec2,
    commands: &mut Commands,
    asset_server: &Res<AssetServer>, 
) {
    let texture: Handle<Image> = asset_server.load("sprite\\primitive\\16px_ring.png");
    
    let selection_motif = commands.spawn(
        SpriteBundle{
            sprite: Sprite { custom_size: Some(SELECTION_MOTIF_SIZE), color: SELECTION_MOTIF_COLOUR, ..Default::default() },
            transform: Transform { translation: SELECTION_MOTIF_OFFSET, ..Default::default()},
            texture: texture,
            visibility: Visibility::Hidden,
            ..Default::default()
        }
    ).id();

    let texture: Handle<Image> = asset_server.load("sprite\\primitive\\64px_square.png");

    // Root
    let root = commands.spawn((
        BRoot{
            collider: Collider::cuboid(PHYSICS_SIZE, PHYSICS_SIZE),
            rigidbody: RigidBody::Dynamic,
            grouping: RTS_UNIT_PHYSICS_BODY_CGROUP,
            speed: MoveSpeed::new(MOVE_SPEED),
            locking: LockedAxes::ROTATION_LOCKED,
            sorter: SpriteSorter::new(ROOT_Z_OFFSET),
            ..Default::default()
        },
        SpriteBundle {
            texture: texture.clone_weak(),
            transform: Transform { translation: location.extend(-2.0), ..Default::default()},
            sprite: Sprite { custom_size: Some(ROOT_SIZE), color: ROOT_COLOUR,..Default::default() },
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
            pure_move_processor: PMProximityProcessor::new(ORDER_COMPLETE_DISTANCE),
            attack_move_processor: AMProximityProcessor::new(ORDER_COMPLETE_DISTANCE),
            collider: Collider::ball(BODY_SIZE),
            rigidbody: RigidBody::Fixed,
            sensor: Sensor,
            grouping: PLAYER_SOUL_CGROUP,
            health: THealth(HEALTH),
            max_health: MaxHealth::new(HEALTH),
            to_despawn_target: ToDespawnTarget::new(root),
            regen: HealthRegeneration(HEALTH_REGEN),
            health_to_colour: HealthToColour::new(FULL_HEALTH_COLOUR, LOW_HEALTH_COLOUR),
            death_flare: DeathFlareOnDeath{
                color: DEATH_FLARE_COLOUR,
                fade: DEATH_FLARE_FADE,
                width: DEATH_FLARE_WIDTH,
            },
            to_selected_motif: ToSelectionMotif::new(selection_motif),
            ..Default::default()
        },
        SpriteBundle {
            texture: texture.clone_weak(),
            transform: Transform { translation: HUB_OFFSET, ..Default::default()},
            sprite: Sprite { custom_size: Some(HUB_SIZE), color: FULL_HEALTH_COLOUR, ..Default::default() },
            ..Default::default()
        },
    )).id();

    // Aggro detector
    let aggro_detector = commands.spawn((
        BAggroDetection{
            detector: CircleIntersectionsOfEnemy::new(AGGRO_RANGE),
            to_root: ToBehaviourRoot::new(hub),
            detection_colour: DetectionColour::new(AGGRO_D_ON_COLOUR, AGGRO_D_OFF_COLOUR),
            ..Default::default()
        },
        SpriteBundle {
            texture: texture.clone_weak(),
            transform: Transform { translation: AGGRO_D_OFFSET, ..Default::default()},
            sprite: Sprite { custom_size: Some(AGGRO_D_SIZE), ..Default::default() },
            ..Default::default()
        }
    )).id();

    // Attack detector
    let attack_detector = commands.spawn((
        BAttackDetection{
            detector: CircleIntersectionsOfEnemy::new(ATTACK_RANGE),
            to_root: ToBehaviourRoot::new(hub),
            detection_colour: DetectionColour::new(ATTACK_D_ON_COLOUR, ATTACK_D_OFF_COLOUR),
            ..Default::default()
        },
        SpriteBundle {
            texture: texture.clone_weak(),
            transform: Transform { translation: ATTACK_D_OFFSET, ..Default::default()},
            sprite: Sprite { custom_size: Some(ATTACK_D_SIZE), ..Default::default() },
            ..Default::default()
        }
    )).id();

    // Idle
    let idle_behav = commands.spawn((
        BIdle{
            to_root: ToBehaviourRoot::new(hub),
            to_parent: ToParentNode::new(hub),
            to_control: ToControl::new(hub),
            to_nav: ToNav::new(hub),
            to_mover: ToMover::new(root),
            bang_colour: bang_colour::BangColour::new(IDLE_ON_COLOUR, IDLE_OFF_COLOUR),
            ..Default::default()
        },
        SpriteBundle {
            texture: texture.clone_weak(),
            transform: Transform { translation: IDLE_OFFSET, ..Default::default() },
            sprite: Sprite { custom_size: Some(IDLE_SIZE), ..Default::default() },
            ..Default::default()
        }
    )).id();

    // Move
    let move_behav = commands.spawn((
        BMoveB{
            to_root: ToBehaviourRoot::new(hub),
            to_parent: ToParentNode::new(hub),
            to_control: ToControl::new(hub),
            to_nav: ToNav::new(hub),
            to_mover: ToMover::new(root),
            bang_colour: bang_colour::BangColour::new(MOVE_ON_COLOUR, MOVE_OFF_COLOUR),
            ..Default::default()
        },
        SpriteBundle {
            texture: texture.clone_weak(),
            transform: Transform { translation: MOVE_OFFSET, ..Default::default()},
            sprite: Sprite { custom_size: Some(MOVE_SIZE), ..Default::default() },
            ..Default::default()
        }
    )).id();

    // Chase
    let chase_behav = commands.spawn((
        BChase{
            to_root: ToBehaviourRoot::new(hub),
            to_parent: ToParentNode::new(hub),
            to_control: ToControl::new(hub),
            to_nav: ToNav::new(hub),
            to_mover: ToMover::new(root),
            bang_colour: bang_colour::BangColour::new(CHASE_ON_COLOUR, CHASE_OFF_COLOUR),
            ..Default::default()
        },
        SpriteBundle {
            texture: texture.clone_weak(),
            transform: Transform { translation: CHASE_OFFSET, ..Default::default()},
            sprite: Sprite { custom_size: Some(CHASE_SIZE), ..Default::default() },
            ..Default::default()
        }
    )).id();

    // Attack
    let attack_behav = commands.spawn((
        BAttack{
            to_root: ToBehaviourRoot::new(hub),
            to_parent: ToParentNode::new(hub),
            trigger: AttackTrigger::new(ATTACK_SPEED),
            end: AttackEndTrigger::new(ATTACK_ANIMATION_TIME),
            damage: DirectAttackPower::new(ATTACK_POWER),
            to_control: ToControl::new(hub),
            to_nav: ToNav::new(hub),
            to_mover: ToMover::new(root),
            bang_colour: bang_colour::BangColour::new(ATTACK_ON_COLOUR, ATTACK_OFF_COLOUR),
            attack_laser: LaserVisualsOnAttack::new(LASER_COLOUR, LASER_FADE, LASER_WIDTH),
            ..Default::default()
        },
        SpriteBundle {
            texture,
            transform: Transform { translation: ATTACK_OFFSET, ..Default::default()},
            sprite: Sprite { custom_size: Some(ATTACK_SIZE), ..Default::default() },
            ..Default::default()
        }
    )).id();

    commands.entity(root)
        .push_children(&[tree_root, selection_motif])
        .insert(ToHealth::new(hub));
    commands.entity(tree_root).add_child(hub);
    commands.entity(hub).push_children(&[aggro_detector, attack_detector, idle_behav, move_behav, chase_behav, attack_behav]);
}