//use bevy::prelude::*;
use std::marker::*;

// Reference composition
    // Data-destination, reference flag.
    // Data-source, reference flag.
    // Data-transformation, transmission flag.
    // Transmission system.
    // Literal waymark data (Entity) (to reference).
// Each part of the composition, signed by a type signature, denoting the reference.

pub trait RefSignature: Sync + Send + 'static { }

#[macro_export]
macro_rules! ref_signature {($ty:ty) => {
    impl RefSignature for $ty { }
};}