use std::fmt::Write;

use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
    utils::Duration,
};

const FORMAT: &str = "FPS: ";

pub struct FpsPlugin;

impl Plugin for FpsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(FrameTimeDiagnosticsPlugin::default())
            .add_systems(Startup, spawn_text)
            .add_systems(Update, update)
            .init_resource::<FpsTimer>();
    }
}

#[derive(Resource)]
pub struct FpsTimer {
    pub timer: Timer,
    pub average_fps: f32,
}

impl Default for FpsTimer {
    fn default() -> Self {
        Self {
            timer: Timer::new(Duration::from_millis(50), TimerMode::Repeating),
            average_fps: 60.0,
        }
    }
}

#[derive(Component)]
pub struct FpsText;

fn update(
    time: Res<Time>,
    diagnostics: Res<DiagnosticsStore>,
    mut state: ResMut<FpsTimer>,
    mut text_query: Query<&mut Text, With<FpsText>>,
) {
    if state.timer.tick(time.delta()).just_finished() {
        let fps_diags = diagnostics
            .get(FrameTimeDiagnosticsPlugin::FPS)
            .and_then(|fps| fps.average());
        for mut text in text_query.iter_mut() {
            let value = &mut text.sections[0].value;
            value.clear();

            if let Some(fps) = fps_diags {
                state.average_fps = state.average_fps * 0.95 + fps as f32 * 0.05;
                write!(value, "{}{:.0}", FORMAT, state.average_fps).unwrap();
            } else {
                write!(value, "{}", FORMAT).unwrap();
            }
        }
    }
}

fn spawn_text(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/roboto.ttf");
    commands
        .spawn(TextBundle {
            style: Style {
                position_type: PositionType::Absolute,
                left: Val::Px(10.0),
                top: Val::Px(10.0),
                ..default()
            },
            text: Text {
                sections: vec![TextSection {
                    value: FORMAT.to_string(),
                    style: TextStyle {
                        font,
                        font_size: 30.0,
                        color: Color::ORANGE_RED,
                    },
                }],
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(FpsText);
}
