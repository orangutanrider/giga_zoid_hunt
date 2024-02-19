use bevy::prelude::*;

use crate::rts_unit::{
    *,
    behaviour::*,
};

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

pub trait RefBangToTreeRoot<TreeRoot, Export>
where TreeRoot: RefBangExport<Export>, Export: BehaviourInterfacedComponent {
    fn ref_bang_to_root(&self, mut root: Mut<TreeRoot>) {
        root.ref_bang()
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