use bevy::prelude::*;

use crate::validate_active_terminal_c;

use super::*;

// PSEUDOCODE for targeting system
/* 
    Target enemy.
    Store enemy as entity.
    Insert self into targeters hashmap.

    When a attack order becomes current.
    Move it into the current target component.
    However, first validate the target.

    To validate, use the held entity and query it for that targeters component.
    Check if you are still contained in the hashmap.
    If not, the order becomes invalid.

    When one of these orders are processed.
    If it isn't already known that the enemy is dead.
    (If it has been cancelled.)
    Then, query and remove self from the hashmap, on the targeted entity.
    Then set current order to none.
    And clear current in the types terminal.
*/

// PSEUDOCODE systems
// AttackTarget, to current target.
// Current target validation.
// TargetedBy death, to HashSet references.
// Cancel signal, to current target, to targeted by.
// Cancel signal bang reset.

#[derive(Component)]
pub struct AbortCurrentTargetBang(bool);

#[derive(Component)]
pub struct CurrentTarget(Option<Entity>);

#[derive(Component)]
pub struct TargetedBy(HashSet<Entity>);
