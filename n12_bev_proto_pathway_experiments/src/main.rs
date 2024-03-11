use bevy::prelude::*;

#[derive(Component)]
/// OriginToMid
/// MidToDestination
/// Destination
struct Test1Flag(Entity);

#[derive(Component)]
struct OriginToMid(Entity);

#[derive(Component)]
struct MidToDestination(Entity);

#[derive(Component)]
struct Destination(bool);


fn main() {
    println!("Hello, bevy.");

    let mut app = App::new();

    app.run();
}

// Test to see if the error can be expected in the first let statement, from the query get
fn test1(
    q_origin: Query<&OriginToMid, With<Test1Flag>>,
    q_mid: Query<&MidToDestination>,
    mut q_destination: Query<&mut Destination>,
) {
    for origin in q_origin.iter() {
        let Ok(mid) = q_mid.get(origin.0) else {
            return;
        };
        let Ok(mut destination) = q_destination.get_mut(mid.0) else {
            return;
        };

        destination.0 = true;
    }
} // Yeah, this seems to work