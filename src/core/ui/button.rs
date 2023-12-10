use crate::core::{CurrencyValue, CurrencyValues, FONT_HANDLE};

use super::{constants, RelativePixelFont, RelativePixelPositioned, RelativePixelSized};
use bevy::{prelude::*, ui::FocusPolicy, audio::PlaybackMode};

#[derive(Component, Default)]
pub struct MyButton {
    pub enabled: bool,
}

#[derive(Component, Default)]
pub struct PrevInteraction(pub Interaction);

#[derive(Component, Default)]
pub struct EnableButtonWhenHaveMoney {
    pub target: CurrencyValues,
}

pub fn spawn_button<T: Bundle, C: Bundle>(
    builder: &mut ChildBuilder,
    title: &str,
    components: T,
    asset_server: &AssetServer,
    child: Option<C>,
    price_a: Option<u64>,
    price_b: Option<u64>,
    r1: f32,
    r2: f32,
) {
    builder
        .spawn((
            components,
            MyButton::default(),
            Interaction::default(),
            PrevInteraction::default(),
            NodeBundle {
                style: Style {
                    justify_content: JustifyContent::Center,
                    ..Default::default()
                },
                background_color: BackgroundColor(Color::WHITE),
                focus_policy: FocusPolicy::Block,
                z_index: ZIndex::Global(25),
                ..Default::default()
            },
            RelativePixelSized {
                width: 114,
                height: 28,
            },
            UiImage::default(),
        ))
        .with_children(|builder| {
            builder.spawn((TextBundle::from_section(
                title,
                TextStyle {
                    font: FONT_HANDLE,
                    font_size: 24.,
                    color: constants::border_color(),
                    ..Default::default()
                },
            ),RelativePixelFont {size: 10}));
            if let Some(child) = child {
                builder.spawn(child);
            }
            let style = TextStyle {
                font: FONT_HANDLE,
                font_size: 24.,
                color: constants::border_color(),
                ..Default::default()
            };
            if let Some(a) = price_a {
                builder.spawn((
                    TextBundle {
                        text: Text::from_section(a.to_string(), style.clone())
                            .with_alignment(TextAlignment::Right),
                        style: Style {
                            position_type: PositionType::Absolute,
                            right: Val::Percent(r1),
                            top: Val::Percent(25.0),
                            ..default()
                        },
                        ..default()
                    },
                    RelativePixelFont { size: 16 },
                ));
            }
            if let Some(b) = price_b {
                builder.spawn((
                    TextBundle {
                        text: Text::from_section(b.to_string(), style.clone())
                            .with_alignment(TextAlignment::Right),
                        style: Style {
                            position_type: PositionType::Absolute,
                            right: Val::Percent(r2),
                            top: Val::Percent(25.0),
                            ..default()
                        },
                        ..default()
                    },
                    RelativePixelFont { size: 16 },
                ));
            }
        });
}

pub fn button_hover(
    mut buttons: Query<
        (&Interaction, &mut BackgroundColor, &MyButton, &mut UiImage),
        Or<(Changed<Interaction>, Changed<MyButton>)>,
    >,
    asset_server: Res<AssetServer>,
) {
    for (interaction, mut background, button, mut image) in buttons.iter_mut() {
        if button.enabled {
            if *interaction == Interaction::Hovered {
                *image = UiImage::new(asset_server.load("images/HoveredButton.png"));
            } else {
                *image = UiImage::new(asset_server.load("images/EnabledButton.png"));
            }
        } else {
            if *interaction == Interaction::None {
                *image = UiImage::new(asset_server.load("images/DisabledButton.png"));
            } else {
                *image = UiImage::new(asset_server.load("images/DisabledHoveredButton.png"));
            }
        }
    }
}

pub fn button_click_sound(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    buttons: Query<(&MyButton, &Interaction), Changed<Interaction>>,
) {
    for (button, interaction) in buttons.iter() {
        if button.enabled {
            if *interaction == Interaction::Pressed {
                commands.spawn(AudioBundle {
                    source: asset_server.load("audio/button.ogg"),
                    settings: PlaybackSettings {
                        mode: PlaybackMode::Despawn,
                        ..default()
                    }
                });
            }
        }
    }
}