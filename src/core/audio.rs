use bevy::{
    audio::{PlaybackMode, Volume, VolumeLevel},
    prelude::*,
};

pub fn play_background_audio(asset_server: Res<AssetServer>, mut commands: Commands) {
    commands.spawn(AudioBundle {
        source: asset_server.load("audio/background.ogg"),
        settings: PlaybackSettings {
            mode: PlaybackMode::Loop,
            volume: Volume::Absolute(VolumeLevel::new(0.12)),
            ..Default::default()
        },
    });
}
