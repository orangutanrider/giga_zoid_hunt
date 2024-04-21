pub mod chase_persona; pub use chase_persona::*;
pub mod defend_persona; pub use defend_persona::*;

use std::any::TypeId;
use bevy_rapier2d::prelude::*;
use bevy::prelude::*;
use bevy_rand::prelude::*;
use rand_core::*;

use random::*;
use attack_laser::*;
use rts_unit_death::*;
use rts_unit_health::*;
use rts_unit_movers::*;
use rts_unit_nav::*;
use rts_unit_team::*;

use super::*;

// The wildcard head takes control when both heads are at high frenzy.
// The wildcard head moves the body to random waypoints, it is fairly quick.

// When the beast is at low-health, the wildcard head will gain a new behaviour.
// It will be able to assume the persona of either head randomly (but overidden by the main wildcard head behaviour).
// When it does this it will use the frenzy value of the opposite head, as the frenzy for itself.

#[derive(Component, Default)]
pub struct WildcardHead;

#[derive(Bundle, Default)]
pub struct BundWildcard {
    pub to_motif: ToMotif,
    pub to_mover: ToMover,
    pub to_hub: ToHub,
    pub flag: WildcardHead,

    pub mover_in: TMoveAggregator,
    pub mover_process: LocalTransformMovement,
    pub speed: MoveSpeed,

    pub in_nav: TNavWaypoint, // Randomised waypoint
    pub nav_process: DirectNav,
    pub out_nav: NavVectorOutput,

    pub attack: DirectAttackBang,
    pub laser: LaserVisualsOnAttack,
    pub damage: DirectAttackPower,

    // Wildcard
    pub wild_frenzy: WildcardFrenzy,
    pub timer: WildcardTimer,

    // Persona General
    pub persona: WildcardPersona,
    pub persona_frenzy: PersonaFrenzy,
    pub activate: PersonaActivateTimer,
    pub duration: PersonaDurationTimer,

    // Persona Defend
    pub defend_target: DefendPersonaTarget,
    pub defend_attack_timer: DefendPersonaAttackTimer,

    // Persona Chase
    pub chase_target: ChasePersonaTarget,
    pub chase_attack_timer: ChasePersonaAttackTimer,
}

#[derive(Component, Default)]
pub struct WildcardTimer(pub f32);

// If wildcard persona
pub fn wildcard_waypoint_sys(
    mut q: Query<(&mut TNavWaypoint, &mut WildcardTimer, &GlobalTransform, &WildcardPersona), With<WildcardHead>>,
    time: Res<Time>,
    mut rng: ResMut<GlobalEntropy<WyRand>>,
) {
    for (nav, mut timer, head_position, persona) in q.iter_mut() {
        match persona.0 {
            WildcardPersonas::Wildcard => (),
            WildcardPersonas::Chase => continue,
            WildcardPersonas::Defend => continue,
        }

        timer.0 = timer.0 - time.delta_seconds();
        if timer.0 <= 0.0 {
            reset_wildcard_nav(nav, timer, &mut rng);
            continue;
        }

        let head_position = head_position.translation().truncate();
        let distance = head_position.distance(nav.0);
        if distance < WILDCARD_WAYPOINT_COMPLETION_DISTANCE {
            reset_wildcard_nav(nav, timer, &mut rng);
            continue;
        }
    }
}

fn reset_wildcard_nav(
    mut nav: Mut<TNavWaypoint>,
    mut timer: Mut<WildcardTimer>,
    rng: &mut ResMut<GlobalEntropy<WyRand>>,
) {
    // Timer random
    let random = random_0_to_1_f32(rng);

    let min = WILDCARD_WAYPOINT_REFRESH_TIMER_RANG.x;
    let max = WILDCARD_WAYPOINT_REFRESH_TIMER_RANG.y;
    let new_timer = f32::lerp(min, max, random);

    timer.0 = new_timer; // SET

    // Waypoint random
    let x_random = random_0_to_1_f32(rng);
    let y_random = random_0_to_1_f32(rng);

    let min_x = -(WILDCARD_RANDOM_WAYPOINT_BOUNDS.x * 0.5);
    let max_x = -min_x;

    let min_y = -(WILDCARD_RANDOM_WAYPOINT_BOUNDS.y * 0.5);
    let max_y = -min_y;

    let x = f32::lerp(min_x, max_x, x_random);
    let y = f32::lerp(min_y, max_y, y_random);
    let new_waypoint = Vec2::new(x, y);

    nav.0 = new_waypoint; // SET
}

#[derive(Component, Default)]
pub struct WildcardFrenzy{
    real: f32,
    proxy: f32,
} 
impl WildcardFrenzy {
    pub fn read(&self) -> f32 {
        return self.real
    }
}

pub fn wildcard_frenzy_sys(
    mut q: Query<(&mut WildcardFrenzy, &ToHub), With<WildcardHead>>,
    body_q: Query<(&ToDefend, &ToChase)>,
    chase_q: Query<&ChaseFrenzy>,
    defend_q: Query<&DefendFrenzy>,
) {
    for (frenzy, to_hub) in q.iter_mut() {
        wildcard_frenzy(to_hub, frenzy, &body_q, &chase_q, &defend_q);
    }
}

fn wildcard_frenzy(
    to_hub: &ToHub,
    mut frenzy: Mut<WildcardFrenzy>,
    body_q: &Query<(&ToDefend, &ToChase)>,
    chase_q: &Query<&ChaseFrenzy>,
    defend_q: &Query<&DefendFrenzy>,
) {
    ref_caravan!(
        to_hub::body_q((to_defend, to_chase));
        to_defend::defend_q(defend_frenzy);
        to_chase::chase_q(chase_frenzy);
    );

    let defend_frenzy = defend_frenzy.read();
    let chase_frenzy = chase_frenzy.read();
    let total_frenzy = defend_frenzy + chase_frenzy;

    if !(
        (defend_frenzy > WILDCARD_DEFEND_FRENZY_THRESHOLD)
        &&
        (chase_frenzy > WILDCARD_CHASE_FRENZY_THRESHOLD) 
        &&
        (total_frenzy > WILDCARD_TOTAL_FRENZY_THRESHOLD)
    ) { 
        frenzy.proxy = total_frenzy;
        frenzy.real = 0.0;
        return;
    }

    frenzy.proxy = total_frenzy;
    frenzy.real = total_frenzy;
}

// If wildcard persona
pub fn wildcard_head_movement_sys(
    mut q: Query<(&mut TMoveAggregator, &WildcardFrenzy, &GlobalTransform, &ToHub, &TNavWaypoint, &WildcardPersona), With<WildcardHead>>,
    body_q: Query<&GlobalTransform>,
) {
    for (mut mover, frenzy, head_location, to_hub, waypoint, persona) in q.iter_mut() {
        match persona.0 {
            WildcardPersonas::Wildcard => (),
            WildcardPersonas::Chase => continue,
            WildcardPersonas::Defend => continue,
        }

        // Get
        let hub = to_hub.go();
        let Ok(body) = body_q.get(hub) else { continue; };
        let body = body.translation().truncate();

        let head = head_location.translation().truncate();

        let body_head_distance = body.distance(head);

        let waypoint = waypoint.0;

        // Calculate body authority
        let body_prevelance = (body_head_distance * WILDCARD_BODY_PULL) / 1.0;

        // Calculate head autonomy
        let wildcard = frenzy.proxy;
        let wildcard_prevelance = (wildcard * WILDCARD_NECK_GROWTH).clamp(WILDCARD_NECK_MIN, WILDCARD_NECK_MAX);

        // Calculate move vectors
        let to_body_move = (body - head).normalize_or_zero() * WILDCARD_BODY_AUTHORITY;
        let to_target_move = (waypoint - head).normalize_or_zero() * WILDCARD_HEAD_AUTONOMY;

        // To mover
        use rts_unit_movers::Key as MoveKey;
        mover.inputs.insert(MoveKey::External(hub), (to_body_move, body_prevelance)); // Body
        let local = TypeId::of::<WildcardHead>();
        mover.inputs.insert(MoveKey::Local(local), (to_target_move, wildcard_prevelance)); // Move
    }
}

// If wildcard persona
pub fn wildcard_to_body_movement_sys(
    q: Query<(&ToMover, &NavVectorOutput, &WildcardFrenzy, Entity, &WildcardPersona), With<WildcardHead>>,
    mut root_q: Query<&mut TMoveDecider>,
) {
    for (to_mover, move_vec, frenzy, head_entity, persona) in q.iter() {
        match persona.0 {
            WildcardPersonas::Wildcard => (),
            WildcardPersonas::Chase => continue,
            WildcardPersonas::Defend => continue,
        }

        let wildcard = frenzy.read();

        let wildcard_move = move_vec.0.normalize_or_zero();
        let wildcard_move = wildcard_move * ((wildcard * WILDCARD_HEAD_PULL) + WILDCARD_BODY_MOVE_BASE_SPEED);
        let wildcard_move = wildcard_move.clamp_length(0.0, WILDCARD_MOVE_LIMIT);

        let wildcard_prevelance = (wildcard * WILDCARD_FRENZY_DOMINANCE) + WILDCARD_BASE_DOMINANCE; // Move decision prevelance

        // Set
        let hub = to_mover.go();
        let Ok(mut body) = root_q.get_mut(hub) else { continue; };

        use rts_unit_movers::Key as MoveKey;
        body.inputs.insert(MoveKey::External(head_entity), (wildcard_move, wildcard_prevelance));
    }
}

#[derive(Component)]
pub struct WildcardPersona(pub WildcardPersonas);
impl Default for WildcardPersona {
    fn default() -> Self {
        Self(WildcardPersonas::Wildcard)
    }
}

pub enum WildcardPersonas{
    Wildcard,
    Chase,
    Defend
}

#[derive(Component, Default)]
pub struct PersonaActivateTimer(pub f32);

#[derive(Component, Default)]
pub struct PersonaDurationTimer(pub f32);

#[derive(Component, Default)]
pub struct PersonaSwitchable; // Insert once the health threshold is reached

pub fn wildcard_persona_deciding_sys(
    mut q: Query<(&mut WildcardPersona, &mut PersonaActivateTimer, &mut PersonaDurationTimer, &WildcardFrenzy), (With<WildcardHead>, With<PersonaSwitchable>)>,
    time: Res<Time>,
    mut rng: ResMut<GlobalEntropy<WyRand>>,
) {
    for (persona, mut activation, duration, frenzy) in q.iter_mut() {
        match persona.0 {
            WildcardPersonas::Wildcard => {
                if frenzy.real > 0.1 { continue; } // The frenzy overrides the personas

                activation.0 = activation.0 - time.delta_seconds();
                if activation.0 > 0.0 { continue; }

                // New persona + new random activation time + new random duration
                switch_persona(persona, activation, duration, &mut rng);
            },
            WildcardPersonas::Chase => (), WildcardPersonas::Defend => (),
        }
    }
}

fn switch_persona(
    mut persona: Mut<WildcardPersona>,
    mut activation: Mut<PersonaActivateTimer>,
    mut duration: Mut<PersonaDurationTimer>,
    rng: &mut ResMut<GlobalEntropy<WyRand>>,
) {
    let state_random = random_0_to_1_f32(rng);
    let state_random = state_random.round();
    let state_random = state_random > 0.5;
    match state_random {
        true => persona.0 = WildcardPersonas::Chase,
        false => persona.0 = WildcardPersonas::Defend,
    }

    let activation_random = random_0_to_1_f32(rng);
    let activation_random = f32::lerp(WILDCARD_PERSONA_ACTIVATION_TIME_RANGE.x, WILDCARD_PERSONA_ACTIVATION_TIME_RANGE.y, activation_random);
    activation.0 = activation_random;

    let duration_random = random_0_to_1_f32(rng);
    let duration_random = f32::lerp(WILDCARD_PERSONA_DURATION_TIME_RANGE.x, WILDCARD_PERSONA_DURATION_TIME_RANGE.y, duration_random);
    duration.0 = duration_random;
}

pub fn persona_duration_sys(
    mut q: Query<(&mut WildcardPersona, &mut PersonaDurationTimer), (With<WildcardHead>, With<PersonaSwitchable>)>,
    time: Res<Time>,
) {
    for (mut persona, mut duration) in q.iter_mut() {
        match persona.0 {
            WildcardPersonas::Chase => {
                duration.0 = duration.0 - time.delta_seconds();
                if duration.0 > 0.0 { continue; }

                persona.0 = WildcardPersonas::Wildcard;
            },
            WildcardPersonas::Defend => {
                duration.0 = duration.0 - time.delta_seconds();
                if duration.0 > 0.0 { continue; }

                persona.0 = WildcardPersonas::Wildcard;
            },
            WildcardPersonas::Wildcard => (),
        }
    }
}

pub fn frenzy_override_sys(
    mut q: Query<(&mut WildcardPersona, &WildcardFrenzy), (With<WildcardHead>, With<PersonaSwitchable>)>,
) {
    for (mut persona, frenzy) in q.iter_mut() {
        match persona.0 {
            WildcardPersonas::Chase => {
                if frenzy.real < 0.1 { continue; } 
                persona.0 = WildcardPersonas::Wildcard;
            },
            WildcardPersonas::Defend => {
                if frenzy.real < 0.1 { continue; } 
                persona.0 = WildcardPersonas::Wildcard;
            },
            WildcardPersonas::Wildcard => (),
        }
    }
}

#[derive(Component, Default)]
pub struct PersonaFrenzy(pub f32);
impl PersonaFrenzy {
    pub fn read(&self) -> f32 {
        return self.0
    }
}

pub fn persona_frenzy_sys(
    mut q: Query<(&mut PersonaFrenzy, &WildcardPersona, &ToHub), (With<WildcardHead>, With<PersonaSwitchable>)>,
    body_q: Query<(&ToDefend, &ToChase)>,
    chase_q: Query<&ChaseFrenzy>,
    defend_q: Query<&DefendFrenzy>,
) {
    for (mut frenzy, persona, to_hub) in q.iter_mut() {
        match persona.0 {
            WildcardPersonas::Wildcard => {
                frenzy.0 = 0.0;
                continue;
            },
            WildcardPersonas::Chase => {
                defend_frenzy_as_persona_frenzy(frenzy, to_hub, &body_q, &defend_q);
            },
            WildcardPersonas::Defend =>{
                chase_frenzy_as_persona_frenzy(frenzy, to_hub, &body_q, &chase_q);
            },
        }
    }
}

fn defend_frenzy_as_persona_frenzy(
    mut frenzy: Mut<PersonaFrenzy>,
    to_hub: &ToHub,
    body_q: &Query<(&ToDefend, &ToChase)>,
    defend_q: &Query<&DefendFrenzy>,
) {
    ref_caravan!(
        to_hub::body_q((to_defend, _)) -> to_defend::defend_q(defend_frenzy);
    );

    let defend_frenzy = defend_frenzy.read();
    frenzy.0 = defend_frenzy * WILDCARD_CHASE_PERSONA_FRENZY_SCALAR;
}

fn chase_frenzy_as_persona_frenzy(
    mut frenzy: Mut<PersonaFrenzy>,
    to_hub: &ToHub,
    body_q: &Query<(&ToDefend, &ToChase)>,
    chase_q: &Query<&ChaseFrenzy>,
) {
    ref_caravan!(
        to_hub::body_q((_, to_chase)) -> to_chase::chase_q(chase_frenzy);
    );

    let chase_frenzy = chase_frenzy.read();
    frenzy.0 = chase_frenzy * WILDCARD_DEFEND_PERSONA_FRENZY_SCALAR;
}

pub fn persona_switching_activation_sys(
    q: Query<(Entity, &ToHub), (With<WildcardHead>, Without<PersonaSwitchable>)>,
    body_q: Query<(&THealth, &MaxHealth)>,
    mut commands: Commands,
) {
    for (wildcard, to_hub) in q.iter() {
        let Ok((health, max_health)) = body_q.get(to_hub.go()) else { continue; };

        let health = health.0;
        let max_health = max_health.read();

        let percentage_current_health = health / max_health;

        if percentage_current_health > WILDCARD_PERSONA_SWITCHING_HEALTH_THRESHOLD { continue; }
        
        commands.entity(wildcard).insert(PersonaSwitchable);
    }
}

// Neck
#[derive(Component)]
pub struct WildcardNeck{
    pub hub: Entity,
    pub wildcard: Entity,
}

pub fn wildcard_neck_sys(
    mut q: Query<(&mut Transform, &WildcardNeck)>,
    transform_q: Query<&GlobalTransform>,
) {
    for (mut transform, neck) in q.iter_mut() {
        let origin = neck.hub;
        let Ok(origin) = transform_q.get(origin) else {continue;};
        let origin = origin.translation().truncate();

        let target = neck.wildcard;
        let Ok(target) = transform_q.get(target) else {continue;};
        let target = target.translation().truncate();

        let distance = origin.distance(target);
        let diff = target - origin;
        let direction = diff.normalize();
    
        let translation = (origin + (direction * distance * 0.5)).extend(-0.5);
        let rotation = Quat::from_rotation_z(direction.to_angle());
    
        let scale = Vec3::new(distance, NECK_WIDTH, 0.1);

        transform.scale = scale;
        transform.translation = translation;
        transform.rotation = rotation;
    }
}

// Colour
pub fn wildcard_frenzy_to_colour(
    mut q: Query<(&mut Sprite, &WildcardFrenzy, &PersonaFrenzy, &WildcardPersona), With<WildcardHead>>,
) {
    for (sprite, wild_frenzy, persona_frenzy, persona) in q.iter_mut() {
        match persona.0 {
            WildcardPersonas::Wildcard => {
                wildcard_persona_frenzy_colour(sprite, wild_frenzy);
            },
            WildcardPersonas::Chase => {
                chase_persona_frenzy_colour(sprite, persona_frenzy);
            },
            WildcardPersonas::Defend => {
                defend_persona_frenzy_colour(sprite, persona_frenzy);
            },
        }
    }
}

fn wildcard_persona_frenzy_colour(
    mut sprite: Mut<Sprite>,
    frenzy: &WildcardFrenzy,
) {
    let min = WILDCARD_FRENZY_COLOUR_MIN_MAX.x;
    let max = WILDCARD_FRENZY_COLOUR_MIN_MAX.y;
    
    let current = frenzy.read();
    
    let t = (current + min) / max;
    let colour_min: Vec3 = WILDCARD_COLOUR.hsl_to_vec3();
    let colour_max = WILDCARD_FRENZY_COLOUR.hsl_to_vec3();
    let colour = Vec3::lerp(colour_min, colour_max, t);
    
    sprite.color = Color::hsl(colour.x, colour.y, colour.z);
}

fn chase_persona_frenzy_colour(
    mut sprite: Mut<Sprite>,
    frenzy: &PersonaFrenzy,
) {
    let min = WILDCARD_CHASE_PERSONA_FRENZY_COLOUR_MIN_MAX.x;
    let max = WILDCARD_CHASE_PERSONA_FRENZY_COLOUR_MIN_MAX.y;
    
    let current = frenzy.read();
    
    let t = (current + min) / max;
    let colour_min: Vec3 = WILDCARD_CHASE_PERSONA_COLOUR.hsl_to_vec3();
    let colour_max = WILDCARD_CHASE_PERSONA_FRENZY_COLOUR.hsl_to_vec3();
    let colour = Vec3::lerp(colour_min, colour_max, t);
    
    sprite.color = Color::hsl(colour.x, colour.y, colour.z);
}

fn defend_persona_frenzy_colour(
    mut sprite: Mut<Sprite>,
    frenzy: &PersonaFrenzy,
) {
    let min = WILDCARD_DEFEND_PERSONA_FRENZY_COLOUR_MIN_MAX.x;
    let max = WILDCARD_DEFEND_PERSONA_FRENZY_COLOUR_MIN_MAX.y;
    
    let current = frenzy.read();
    
    let t = (current + min) / max;
    let colour_min: Vec3 = WILDCARD_DEFEND_PERSONA_COLOUR.hsl_to_vec3();
    let colour_max = WILDCARD_DEFEND_PERSONA_FRENZY_COLOUR.hsl_to_vec3();
    let colour = Vec3::lerp(colour_min, colour_max, t);
    
    sprite.color = Color::hsl(colour.x, colour.y, colour.z);
}

#[derive(Component, Default)]
pub struct Motif;

#[derive(Component)]
pub struct ToMotif(Entity);
waymark!(ToMotif);

pub fn head_motif_rotate_sys(
    q: Query<(&ToMotif, &WildcardPersona, &NavVectorOutput, &DefendPersonaTarget, &ChasePersonaTarget, &GlobalTransform), With<WildcardHead>>,
    mut motif_q: Query<&mut Transform, With<Motif>>,
    target_q: Query<&GlobalTransform, (Without<Motif>, Without<WildcardHead>)>,
) {
    for (to_motif, persona, wildcard, defend, chase, head_pos) in q.iter() {
        let Ok(motif) = motif_q.get_mut(to_motif.go()) else { continue; };

        match persona.0 {
            WildcardPersonas::Wildcard => {
                wildcard_motif_rotate(motif, wildcard);
            },
            WildcardPersonas::Chase => {
                chase_persona_motif_rotate(motif, head_pos, &chase, &target_q);
            },
            WildcardPersonas::Defend => {
                defend_persona_motif_rotate(motif, head_pos, &defend, &target_q);
            },
        }
    }
}

fn wildcard_motif_rotate(
    mut motif: Mut<Transform>,
    wildcard: &NavVectorOutput,
) {
    let rotation = Quat::from_rotation_z(wildcard.0.to_angle());
    motif.rotation = rotation;
}

fn chase_persona_motif_rotate(
    mut motif: Mut<Transform>,
    head_pos: &GlobalTransform,
    target: &ChasePersonaTarget,
    target_q: &Query<&GlobalTransform, (Without<Motif>, Without<WildcardHead>)>,
) {
    // Get
    let target = target.read();
    let Ok(target) = target_q.get(target) else { return; };
    let target = target.translation().truncate();
    
    let head_pos = head_pos.translation().truncate();

    let diff = target - head_pos;
    let direction = diff.normalize();

    let rotation = Quat::from_rotation_z(direction.to_angle());
    
    // Set
    motif.rotation = rotation;
}

fn defend_persona_motif_rotate(
    mut motif: Mut<Transform>,
    head_pos: &GlobalTransform,
    target: &DefendPersonaTarget,
    target_q: &Query<&GlobalTransform, (Without<Motif>, Without<WildcardHead>)>,
) {
    // Get
    let target = target.read();
    let Ok(target) = target_q.get(target) else { return; };
    let target = target.translation().truncate();
    
    let head_pos = head_pos.translation().truncate();

    let diff = target - head_pos;
    let direction = diff.normalize();

    let rotation = Quat::from_rotation_z(direction.to_angle());
    
    // Set
    motif.rotation = rotation;
}