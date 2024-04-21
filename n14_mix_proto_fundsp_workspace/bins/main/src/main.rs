#![allow(clippy::precedence)]

use {bevy::prelude::*, bevy_fundsp_13::prelude::*};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(DspPlugin::default())
        .add_dsp_source(active, SourceType::Dynamic)
        .add_systems(PostStartup, play_noise)
        .run();
}

// https://github.com/SamiPerttu/fundsp?tab=readme-ov-file#more-on-multithreading-and-real-time-control
// Hmm... Atmoics.
// This don't make sense though. 
// Threads aswell? Wont that get messed up with bevy's multi-threading? 
// Settings then?
// Hmm... Don't get it either.
// I don't have the time to not get this stuff aswell.
// Well, I think that is that, this is not the project to explore this stuff during.
// Unless I somehow end up with spare-time after my essays.

// It seems like they're scope independant, but thread locked or something?
// I really don't know how I could utallise that in my game though.

fn active() -> impl AudioUnit32 {
    white() >> split::<U2>() * 0.2
}


// It seems that the synth cannot be manipulated during runtime beyond effects.
// Hmm... Ain't really a point to it if I can't do that.

fn proto_4_lfo() -> impl AudioUnit32 {
    let f = 220.0;
    let m = 2.0;

    let lfo1 = lfo(|t| {
        0.1
    });
    let m = m - lfo1;

    let lfo2 = lfo(|t| {
        1.0
    });
    let ff = f * lfo2;

    let lfo3 = lfo(|t| {
        10.0
    });
    let fff = f * lfo3;

    let synth = (sine_hz(f) * ff * m + fff) >> sine();

    //return synth >> split::<U2>() * 0.2
    return synth 
}

fn proto_3_resonantor() -> impl AudioUnit32 {
    let f = 440.0;
    let m = 5.0;
    let synth1 = (sine_hz(f) * f * m + f) >> sine();

    //let f = 120.0;
    //let m = 12.0;
    //let synth2 = (sine_hz(f) * f * m + f) >> sine();

    //let reverb = multipass() & 1.0 * reverb_stereo(20.0, 4.0, 1.0);
    //let synth = (synth1 | synth2) >> reverb;

    let synth = synth1 | dc((220.0, 440.0));

    let synth = synth >> resonator();

    //return synth >> split::<U2>() * 0.2
    return synth 
}

fn proto2_fm_synth() -> impl AudioUnit32 {
    let f = 440.0;
    let m = 5.0;
    let synth = (sine_hz(f) * f * m + f) >> sine();

    //return synth >> split::<U2>() * 0.2
    return synth 
}

// white() >> split::<U2>() * 0.2
fn proto1_reverb() -> impl AudioUnit32 {
    let synth1 = sine_hz(440.0) & sine_hz(440.0);
    let synth2 = sine_hz(440.0) & sine_hz(440.0);
    let reverb = multipass() & 1.0 * reverb_stereo(20.0, 4.0, 1.0);
    let synth = (synth1 | synth2) >> reverb;

    //return synth >> split::<U2>() * 0.2
    return synth 
}

fn play_noise(
    mut commands: Commands,
    mut assets: ResMut<Assets<DspSource>>,
    dsp_manager: Res<DspManager>,
) {
    let source = assets.add(
        dsp_manager
            .get_graph(active)
            .unwrap_or_else(|| panic!("DSP source not found!"))
            .clone(),
    );
    commands.spawn(AudioSourceBundle {
        source,
        ..default()
    });
}