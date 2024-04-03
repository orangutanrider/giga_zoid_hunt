// All edit mode stuff, has been relegated to theory.
// This is to limit the technical scope of the project.
// So that I can focus on creating the game itself.

// What you would have here, in terms of data
/*
    Data pertaining to the kind of waymark it is (Direct, Up, Down)
    To say where it is pointing to, how it is constrained.
    Direct has no constraints, up and down can point to anywhere above or below
    In the entity hierachy (so they can skip to root, but they cannot cross branches)

    Data pertaining to the reference path that a flag takes.
    Loosely, this is the caravan that it uses.
    However, a caravan can be adapted to account for multiple reference paths.
    So that comparison doesn't make complete sense.

    Data pertaining to what the flag is for (what the reference gets)
*/

// Theoretically, a mutable waymark could be like a turn-table.