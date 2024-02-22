use bevy::prelude::*;

use crate::rts_unit::*;

use super::ToBehaviourTreeRoot;

#[derive(Component)]
/// Behaviour bang terminal
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

#[derive(PartialEq, Eq)]
#[derive(Clone, Copy)]
pub enum PropagationState {
    Send,
    Dormant,
}

#[derive(Component)]
pub struct RefBangPropagator(PropagationState);

/// Methods
impl RefBangPropagator {
    fn set(&mut self, state: PropagationState) {
        self.0 = state;
    }

    fn is_propagating(&self) -> bool {
        match self.0 {
            PropagationState::Send => {
                return true
            },
            PropagationState::Dormant => {
                return false
            },
        }
    }
}

/// Pre Update
fn ref_bang_propagation(
    mut node_q: Query<(&mut RefBangPropagator, &Children), Changed<RefBangPropagator>>,
    mut child_q: Query<(&mut RefBangPropagator, &TBehaviourBang)>,
) {
    for (mut propagator, children) in node_q.iter_mut() {
        if !propagator.is_propagating() {
            continue;
        }
        propagator.set(PropagationState::Dormant);
        
        for child in children.iter() {
            let result = child_q.get_mut(*child);
            let Ok((mut child_propagator, terminal)) = result else {
                println!("err"); // will rework in refactor
                continue;
            };

            if !terminal.is_active() {
                continue;
            }

            child_propagator.set(PropagationState::Send);
        }
    }
}

fn ref_bang_propagation_end(
    mut node_q: Query<&mut RefBangPropagator, (Changed<RefBangPropagator>, Without<Children>)>,
) {
    for mut propagator in node_q.iter_mut() {
        if !propagator.is_propagating() {
            continue;
        }

        propagator.set(PropagationState::Dormant);
    }
}

// Hmm... This many generics and trait bounds? Seems bad. I've done this here though, cause I plan to refactor some of the stuff, and make this better.
// Maybe procedural macros fix this?
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

    fn ref_bang_system(
        node_q: Query<(&RefBangPropagator, &ToBehaviourTreeRoot, &TBehaviourBang), (Changed<RefBangPropagator>, With<T>)>,
        mut root_q: Query<&mut RootBang>,
    ) {
        for (propagator, to_root, terminal) in node_q.iter() {
            if !propagator.is_propagating(){
                continue;
            }
            if !terminal.is_active() {
                continue;
            }

            let root = to_root.entity();
            let root_bang = root_q.get_mut(root);
            let Ok(mut root_bang) = root_bang else {
                Self::print_err_descript(2, "Failed at getting ref bang export from root.");
                return;
            };

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