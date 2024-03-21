// Traits, you could define traits, and maybe macros, for custom state terminals, similar to the main one
// So unlike aux, they use hashmaps and collect an overall output.
// So you could have like an adder or something, if you wanted.

// Trait for component
// Macro for system (or just generic system)

// Un-implemented because it'd require reworking of some of the propagation stuff.
// (All state components would have to have their own propagator, or trigger a shared one)
// The bang actuator, fizzler, latch, and release, would have to be updated to support it.

// It is fine, because this is not a required feature, but it would make the system more flexible.