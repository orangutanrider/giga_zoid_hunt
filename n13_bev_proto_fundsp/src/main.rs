// THIS PLUGIN IS ABANDONED
// Or something, point is, it doesn't support the latest version of bevy.
// Yet there're pull requests for it to do so https://github.com/harudagondi/bevy_fundsp/pull/50
// Hmm...
// Could maybe just take that code and use it as a lib
// I will test that in a new proto workspace

#![allow(clippy::precedence)]

use {bevy::prelude::*, bevy_fundsp::prelude::*};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(DspPlugin::default())
        .add_dsp_source(white_noise, SourceType::Dynamic)
        .add_systems(PostStartup, play_noise)
        .run();
}

fn white_noise() -> impl AudioUnit32 {
    white() >> split::<U2>() * 0.2
}

fn play_noise(
    mut commands: Commands,
    mut assets: ResMut<Assets<DspSource>>,
    dsp_manager: Res<DspManager>,
) {
    let source = assets.add(
        dsp_manager
            .get_graph(white_noise)
            .unwrap_or_else(|| panic!("DSP source not found!"))
            .clone(),
    );

    commands.spawn(AudioSourceBundle {
        source,
        ..default()
    });
}