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
struct MidToC(Entity);
waymark!(MidToC);

#[derive(Component)]
struct A;

#[derive(Component)]
struct B(bool);

#[derive(Component)]
struct C(bool);

#[derive(Bundle)]
struct ABundle {
    a: A,
    flag: AToBRefFlag,
    to_mid: ToMid,
}

#[derive(Bundle)]
struct MidBundle {
    to_b: MidToB,
    to_c: MidToC,
}

#[derive(Bundle)]
struct BBundle {
    b: B,
}

#[derive(Bundle)]
struct CBundle {
    c: C,
}

#[test]
fn a_to_bc_nested() {
    let mut app = App::new();

    // Add system
    app.add_systems(Update, a_to_bc_nested_sys);

    // Set-up world state
    let world = &mut app.world;
    let b = world.spawn(BBundle{
        b: B(false)
    }).id();
    let c = world.spawn(CBundle{
        c: C(false)
    }).id();
    let mid = world.spawn(MidBundle{
        to_b: MidToB::new(b),
        to_c: MidToC::new(c)
    }).id();
    world.spawn(ABundle{
        a: A,
        flag: AToBRefFlag,
        to_mid: ToMid::new(mid),
    });

    // run system
    app.update();
}

fn a_to_bc_nested_sys(
    a_q: Query<&ToMid, With<A>>,
    mid_q: Query<(&MidToB, &MidToC)>,
    b_q: Query<&B>,
    c_q: Query<&C>,
) {
    for to_mid in a_q.iter() {
        ref_caravan!(
            to_mid::mid_q((mid_to_b, mid_to_c)) -> {
                mid_to_b::b_q(b),
                mid_to_c::c_q(c)
            }
        );

        assert_eq!(b.0, false, "ref_caravan test:
        to_mid::mid_q((mid_to_b, mid_to_c)) -> {{ 
            mid_to_b::b_q(b), 
            mid_to_c::c_q(c), 
        }}");

        assert_eq!(c.0, false, "ref_caravan test:
        to_mid::mid_q((mid_to_b, mid_to_c)) -> {{ 
            mid_to_b::b_q(b), 
            mid_to_c::c_q(c), 
        }}");
    }
}