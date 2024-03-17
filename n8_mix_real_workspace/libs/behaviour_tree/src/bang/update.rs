use bevy:: prelude::*;

use super::*;

#[derive(Component)]
/// Bang propogator.
/// When a bang is set active, the child nodes will have this component be set to true.
/// When this component is true, bang latches will check to see if their bang should now be activated.
/// The component sets itself to false, and the actiavted bangs repeat the cycle.
pub(crate) struct BangPropogator(bool);
impl Default for BangPropogator {
    fn default() -> Self {
        return Self::new()
    }
}
impl BangPropogator { //! Constructor
    pub fn new() -> Self {
        return Self(false)
    }
}

fn bang_propogation_sys(
    node_q: Query<(&TBang, &Children), Changed<TBang>>,
    mut child_q: Query<&mut BangPropogator>,
) {
    for (bang, children) in node_q.iter() {
        if bang.c
    }
}

// system for when they die