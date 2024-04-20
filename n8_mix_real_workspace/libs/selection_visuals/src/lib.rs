use bevy::prelude::*;

use ref_paths::*;

use rts_unit_control::prelude::*;

pub struct SelectionMotifPlugin;
impl Plugin for SelectionMotifPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            show_selection_visuals_sys,
            hide_selection_visuals_sys,
        ));
    }
}

#[derive(Component)]
pub struct ToSelectionMotif(Entity);
waymark!(ToSelectionMotif);

pub fn show_selection_visuals_sys(
    q: Query<&ToSelectionMotif, With<Selected>>, // Entrance query
    mut visuals_q: Query<&mut Visibility>,
) {
    for to_visuals in q.iter() {
        let Ok(mut visibility) = visuals_q.get_mut(to_visuals.go()) else { continue; };
        *visibility = Visibility::Visible; 
    }
}

pub fn hide_selection_visuals_sys(
    q: Query<&ToSelectionMotif, Without<Selected>>, // Entrance query
    mut visuals_q: Query<&mut Visibility>,
) {
    for to_visuals in q.iter() {
        let Ok(mut visibility) = visuals_q.get_mut(to_visuals.go()) else { continue; };
        *visibility = Visibility::Hidden; 
    }
}