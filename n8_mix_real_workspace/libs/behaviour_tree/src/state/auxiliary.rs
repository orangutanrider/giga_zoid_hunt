// Basic implementation, of terminal to terminal output
// No collection, they simply set eachothers held value

// Just TState, but State can be any type, and there is no hashmap.

// Un-implemented because it'd require reworking of some of the propogation stuff.
// (All state components would have to have their own propagator, or trigger a shared one)
// The bang actuator, fizzler, latch, and release, would have to be updated to support it.

// It is fine, because this is not a required feature, but it would make the system more flexible.