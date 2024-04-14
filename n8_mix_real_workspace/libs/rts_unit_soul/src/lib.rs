//! Bundles and collider stuff, to create the detectable body of the unit.

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use rapier_config::*; 

use rts_unit_health::*;
use rts_unit_death::*;
use rts_unit_team::*;

use health_to_death::*;

#[derive(Bundle)]
pub struct BSoulCore {
    pub collider: Collider,
    pub rigidbody: RigidBody,
    pub sensor: Sensor,
    pub grouping: CollisionGroups,

    pub health: THealth,
    pub max_health: MaxHealth,
    pub health_to_death: ZeroHealthMeansDeath,
    pub death_is_local: DeathIsLocal,
    pub health_is_local: HealthIsLocal,

    pub death: DeathBang,
    pub death_to_despawn: DeathToEntityDespawn,
    pub despawn_is_ref: DespawnTargetIsReference,
    pub to_despawn_target: ToDespawnTarget,
}
impl BSoulCore {
    pub fn new(
        health: f32,
        size: f32,
        root: Entity,
        grouping: CollisionGroups,
    ) -> Self {
        return Self { 
            collider: Collider::ball(size), 
            rigidbody: RigidBody::Fixed, 
            sensor: Sensor, 
            grouping, 

            health: THealth(health), 
            max_health: MaxHealth::new(health), 
            health_to_death: ZeroHealthMeansDeath,
            death_is_local: DeathIsLocal,
            health_is_local: HealthIsLocal,
            
            death: DeathBang::new(),
            death_to_despawn: DeathToEntityDespawn,
            despawn_is_ref: DespawnTargetIsReference,
            to_despawn_target: ToDespawnTarget::new(root),
        }
    }
}

#[derive(Bundle)]
pub struct BEnemyAffiliatedSoul {
    pub core: BSoulCore,
    pub team_affiliation: EnemyTeam,
}
impl BEnemyAffiliatedSoul {
    pub fn new(
        health: f32,
        size: f32,
        root: Entity,
    ) -> Self {
        return Self {
            core: BSoulCore::new(health, size, root, ENEMY_SOUL_CGROUP),
            team_affiliation: EnemyTeam,
        }
    }
}

#[derive(Bundle)]
pub struct BPlayerAffiliatedSoul {
    pub core: BSoulCore,
    pub team_affiliation: PlayerTeam,
}
impl BPlayerAffiliatedSoul {
    pub fn new(
        health: f32,
        size: f32,
        root: Entity,
    ) -> Self {
        return Self {
            core: BSoulCore::new(health, size, root, PLAYER_SOUL_CGROUP),
            team_affiliation: PlayerTeam,
        }
    }
}

pub const PLAYER_SOUL_CGROUP: CollisionGroups = CollisionGroups::new(
    SELECTABLE.union(P_DETECTABLE).union(P_HITTABLE), 
    SELECTABLE.union(P_DETECTABLE).union(P_HITTABLE), 
);
pub const ENEMY_SOUL_CGROUP: CollisionGroups = CollisionGroups::new(
    E_DETECTABLE.union(E_HITTABLE), 
    E_DETECTABLE.union(E_HITTABLE), 
);