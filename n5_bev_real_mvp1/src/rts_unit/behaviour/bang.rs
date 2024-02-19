use bevy::prelude::*;

use crate::rts_unit::*;

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

pub trait RefBangToTreeRoot<RefBangExport>
where RefBangExport: RefBangExport<> {
    fn to_root_ref_bang(&self, mut root_ref_bang: Mut<RefBangExport>) {

    }
}

pub trait RefBangExport<Export>
where Export: BehaviourInterfacedComponent {

}

pub trait BehaviourInterfacedComponent: Component {

}