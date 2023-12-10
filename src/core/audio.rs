use bevy::{
    audio::{PlaybackMode, Volume, VolumeLevel},
    prelude::*,
    utils::Instant,
};

pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, play_background_audio);
        app.insert_resource(SoundCounter::default());
    }
}

pub fn play_background_audio(asset_server: Res<AssetServer>, mut commands: Commands) {
    commands.spawn(AudioBundle {
        source: asset_server.load("audio/background.ogg"),
        settings: PlaybackSettings {
            mode: PlaybackMode::Loop,
            volume: Volume::Absolute(VolumeLevel::new(0.005)),
            ..Default::default()
        },
    });
}

#[derive(Resource)]
pub struct SoundCounter {
    pub last_played: Instant,
}

impl Default for SoundCounter {
    fn default() -> Self {
        Self {
            last_played: Instant::now(),
        }
    }
}
