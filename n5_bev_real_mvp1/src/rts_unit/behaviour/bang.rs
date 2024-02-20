use bevy::prelude::*;

#[derive(Component)]
pub struct TBehaviourBang(bool);
impl Default for TBehaviourBang {
    fn default() -> Self {
        return Self(false)
    }
}
impl TBehaviourBang {
    pub fn new() -> Self {
        return Self(false)
    }
}

/// Set Methods
impl TBehaviourBang {
    pub fn set_active(&mut self, v: bool) {
        self.0 = v;
    }
}

/// Get Methods
impl TBehaviourBang {
    pub fn is_active(&self) -> bool {
        return self.0
    }
}

// Hmm... This many generics and trait bounds? Seems bad. I've done this here though, cause I plan to refactor some of the stuff, and make this better.
pub trait BangToRootRefBang<T, RootBang, Export, const N: usize, PathwayOutput> where 
PathwayOutput: InternalEntityRef,
Self: EntityReferenceFlag<N, PathwayOutput>,
T: Component, 
RootBang: RefBangExport<Export>, 
RootBang: Component,
Export: BehaviourInterfacedComponent {
    fn ref_bang(&self, mut root: Mut<RootBang>) {
        root.ref_bang()
    }

    fn bang_to_root_system( // Hmm... Is it better to do this, or to do a system that has generic parameters?
        node_q: Query<(&TBehaviourBang, &ToBehaviourTreeRoot), (With<T>, Changed<TBehaviourBang>)>,
        mut root_q: Query<&mut RootBang>,
    ) {
        for (terminal, to_root) in node_q.iter() {
            let root = to_root.entity();
            let root_bang = root_q.get_mut(root);
            let Ok(mut root_bang) = root_bang else {
                Self::print_err_descript(2, "Failed at getting ref bang export from root.");
                return;
            };

            if !terminal.is_active() {
                return;
            }
            root_bang.ref_bang();
        }
    }
}

pub trait RefBangExport<Export>
where Export: BehaviourInterfacedComponent {
    fn ref_bang(&mut self);
    fn reset(&mut self);
    fn bang_val(&self) -> bool;

    fn export(&self, mut export: Mut<Export>){
        export.set_active(self.bang_val());
    }
}

pub trait BehaviourInterfacedComponent: Component {
    fn set_active(&mut self, v: bool);
}

#[macro_export]
macro_rules! ref_bang_export_impls { ($t:ty, $export:ty) => {
    impl RefBangExport<$export> for $t {
        fn ref_bang(&mut self) {
            self.bang = true;
        }

        fn reset(&mut self) {
            self.bang = true;
        }

        fn bang_val(&self) -> bool {
            return self.bang
        }
    }
};}
pub (crate) use ref_bang_export_impls;

#[macro_export]
macro_rules! bang_to_root_ref_bang_impls { ($t:ty) => {
    impl $t {
        fn bang_to_root_system() {
            
        }
    }
};}
pub (crate) use bang_to_root_ref_bang_impls;

use super::{EntityReferenceFlag, GetEntityRef, InternalEntityRef, ToBehaviourTreeRoot};