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
fn a_to_b_lift() {
    let mut app = App::new();

    // Add system
    app.add_systems(Update, a_to_b_lift_sys);

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
}

fn a_to_b_lift_sys(
    a_q: Query<&ToMid, With<A>>,
    mid_q: Query<&MidToB>,
    b_q: Query<&mut B>,
) {
    for to_mid in a_q.iter() {
        ref_caravan!(
            ^to_mid::mid_q(mid_to_b) -> mid_to_b::b_q(b);
        );

        assert_eq!(b.0, false, "ref_caravan test: ^mid::mid_q(mid_to_b) -> mid_to_b::b_q(b);");
    }
}