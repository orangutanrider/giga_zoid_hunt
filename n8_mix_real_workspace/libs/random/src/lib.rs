use bevy_rand::prelude::*;
use rand_core::*;
use bevy::prelude::*;

pub fn random_0_to_1_f32 (
    rng: &mut ResMut<GlobalEntropy<WyRand>>,
) -> f32 {
    let raw_random = rng.next_u32();
    let random = raw_random as f32;
    let random = random / (u32::MAX as f32);
    return random
}