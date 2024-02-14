#[macro_use]
mod control;
#[macro_use]
mod behaviour;
#[macro_use]
mod soul;
#[macro_use]
mod soul_detection;

mod movement;
mod unit_type;
mod block;

pub mod parts;
pub mod blocks;

use std::any::TypeId;
use bevy::prelude::*;

pub struct InitializePlugin;
impl Plugin for InitializePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            control::InitializePlugin,
            unit_type::InitializePlugin,
            behaviour::InitializePlugin,
            movement::InitializePlugin,
        ));
    }
}

pub trait TypeIdGet{
    const TYPE_ID: TypeId = TypeId::of::<Self>();
}

/// This would be editor only or debug mode information
/// See this commit: 27474abb3e03d8ff9436123ca3dc3d21a17c64d7
pub trait EntityReferenceFlag<const N: usize, Output: InternalEntityRef>: TypeIdGet {
    const REFERENCE_PATH: [TypeId; N]; 
    const REF_TYPE: EntityRefFlagRefType;

    fn print_err(step: usize) {
        println!("{:?}, Entity reference error, fail at step {}, for type {:?}.", Self::TYPE_ID, step, Self::REFERENCE_PATH[step]);
    }

    fn print_err_descript(step: usize, msg: &str) {
        println!("{:?}, Entity reference error, fail at step {}, for type {:?}. Msg: {}", Self::TYPE_ID, step, Self::REFERENCE_PATH[step], msg);
    }
}

#[derive(Clone, Copy)]
#[derive(PartialEq, Eq)]
pub enum EntityRefFlagRefType {
    Immutable,
    Mutable,
}

pub trait InternalEntityRef {
    fn ref_type() -> EntityRefType;
}

pub trait GetEntityRef: InternalEntityRef {
    fn entity(&self) -> Entity;
}

enum EntityRefType {
    SelfEntity(SelfEntity),
    ParentEntity(ParentEntity),
    ChildEntity(ChildEntity),
    RootEntity(RootEntity)
}

struct SelfEntity;
impl InternalEntityRef for SelfEntity {
    fn ref_type() -> EntityRefType {
        return EntityRefType::SelfEntity(SelfEntity)
    }
}
struct ParentEntity;
impl InternalEntityRef for ParentEntity {
    fn ref_type() -> EntityRefType {
        return EntityRefType::ParentEntity(ParentEntity)
    }
}
struct ChildEntity;
impl InternalEntityRef for ChildEntity {
    fn ref_type() -> EntityRefType {
        return EntityRefType::ChildEntity(ChildEntity)
    }
}
struct RootEntity;
impl InternalEntityRef for RootEntity {
    fn ref_type() -> EntityRefType {
        return EntityRefType::RootEntity(RootEntity)
    }
}

#[macro_export]
macro_rules! entity_ref_impls { ($t:ty, $ref_type:ident) => {
    impl $t {
        pub const PLACEHOLDER: Self = Self(Entity::PLACEHOLDER);

        pub fn new(entity: Entity) -> Self {
            return Self(entity)
        }
    }

    impl TypeIdGet for $t { }

    impl Default for $t {
        fn default() -> Self {
            return Self::PLACEHOLDER
        }
    }

    impl InternalEntityRef for $t {
        fn ref_type() -> EntityRefType {
            return $ref_type::ref_type()
        }
    }

    impl GetEntityRef for $t {
        fn entity(&self) -> Entity {
            return self.0
        }
    }
};}
pub(crate) use entity_ref_impls;


#[derive(Clone, Copy)]
#[derive(Component)]
/// Attach to the root entity
/// An entity that is expected to be a the root entity of an RTS unit
pub struct RTSUnitRoot(Entity);
entity_ref_impls!(RTSUnitRoot, SelfEntity);

#[derive(Component)]
/// For any entity attached to the unit in the tree
pub struct ToRoot(Entity);
entity_ref_impls!(ToRoot, RootEntity);