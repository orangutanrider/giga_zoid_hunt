use bevy::prelude::*;
use ref_paths::*;
use ref_caravan::ref_caravan;

#[derive(Component)]
struct AToBRefFlag;

#[derive(Component)]
struct ToMid(Entity);
waymark!(ToMid);

#[derive(Component)]
struct MidToB(Entity);
waymark!(MidToB);

#[derive(Component)]
struct A;

#[derive(Component)]
struct B(bool);

#[derive(Bundle)]
struct ABundle {
    a: A,
    flag: AToBRefFlag,
    to_mid: ToMid,
}

#[derive(Bundle)]
struct MidBundle {
    to_b: MidToB
}

#[derive(Bundle)]
struct BBundle {
    b: B,
}

#[test]
fn a_to_b_mut() {
    let mut app = App::new();

    // Add system
    app.add_systems(Update, a_to_b_mut_sys);

    // Set-up world state
    let world = &mut app.world;
    let b = world.spawn(BBundle{
        b: B(false)
    }).id();
    let mid = world.spawn(MidBundle{
        to_b: MidToB::new(b),
    }).id();
    world.spawn(ABundle{
        a: A,
        flag: AToBRefFlag,
        to_mid: ToMid::new(mid),
    });

    // run system
    app.update();

    let b = app.world.get::<B>(b);
    assert!(b.is_some());
    assert_eq!(b.unwrap().0, true, "ref_caravan test: to_mid::mid_q(mid_to_b) -> mid_to_b::b_q(mut b)")
}

fn a_to_b_mut_sys(
    a_q: Query<&ToMid, With<A>>,
    mid_q: Query<&MidToB>,
    mut b_q: Query<&mut B>,
) {
    for to_mid in a_q.iter() {
        ref_caravan!(
            to_mid::mid_q(mid_to_b) -> mid_to_b::b_q(mut b);
        );

        b.0 = true;
    }
}