use super::{constants, RelativePixelFont, RelativePixelSized};
use crate::core::{
    CurrencyStorage, CurrencyType, CURRENCY_HONEY, CURRENCY_MAGIC_WAX, CURRENCY_NUM, CURRENCY_WAX,
    FONT_HANDLE,
};
use bevy::prelude::*;
use strum::IntoEnumIterator;

#[derive(Component)]
pub enum DisplayType {
    Value,
    Inflow,
    Limit,
}

pub fn spawn_currency_display(builder: &mut ChildBuilder, asset_server: &mut AssetServer) {
    builder
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.),
                position_type: PositionType::Absolute,
                bottom: Val::Percent(2.0),
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|builder| {
            builder
                .spawn((
                    NodeBundle {
                        style: Style {
                            margin: UiRect {
                                left: Val::Auto,
                                right: Val::Auto,
                                top: Val::ZERO,
                                bottom: Val::ZERO,
                            },
                            ..Default::default()
                        },
                        background_color: BackgroundColor(Color::WHITE),
                        ..Default::default()
                    },
                    RelativePixelSized {
                        width: 150,
                        height: 32,
                    },
                    UiImage::new(asset_server.load("images/ResourcesMenu.png")),
                ))
                .with_children(|builder| {
                    spawn_currency_counter(builder, CURRENCY_HONEY, asset_server, 66.0);
                    spawn_currency_counter(builder, CURRENCY_WAX, asset_server, 39.0);
                    spawn_currency_counter(builder, CURRENCY_MAGIC_WAX, asset_server, 13.0);
                });
        });
}

pub fn spawn_currency_counter(
    builder: &mut ChildBuilder,
    currency_type: CurrencyType,
    asset_server: &mut AssetServer,
    right: f32,
) {
    let text_style = TextStyle {
        font: FONT_HANDLE,
        font_size: 16.,
        color: constants::border_color(),
        ..Default::default()
    };
    let font_size = RelativePixelFont { size: 16 };

    builder
        .spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                top: Val::Percent(25.0),
                right: Val::Percent(right),
                width: Val::Percent(30.0),
                height: Val::Percent(75.0),
                ..default()
            },
            ..default()
        })
        .with_children(|builder| {
            builder.spawn((
                currency_type.clone(),
                DisplayType::Value,
                TextBundle {
                    text: Text::from_section("", text_style.clone())
                        .with_alignment(TextAlignment::Right),
                    style: Style {
                        position_type: PositionType::Absolute,
                        right: Val::Percent(5.0),
                        ..default()
                    },
                    ..default()
                },
                font_size,
            ));

            builder.spawn((
                currency_type,
                DisplayType::Inflow,
                TextBundle {
                    text: Text::from_section("", text_style.clone())
                        .with_alignment(TextAlignment::Right),
                    style: Style {
                        position_type: PositionType::Absolute,
                        right: Val::Percent(5.0),
                        bottom: Val::Percent(5.0),
                        ..default()
                    },
                    ..default()
                },
                RelativePixelFont { size: 10 },
            ));
        });

    /*builder
    .spawn(NodeBundle {
        style: Style {
            flex_direction: FlexDirection::Column,
            margin: UiRect::horizontal(Val::Px(10.)),
            ..Default::default()
        },
        ..Default::default()
    })
    .with_children(|builder| {
        let text_style = TextStyle {
            font: FONT_HANDLE,
            font_size: 24.,
            color: constants::border_color(),
            ..Default::default()
        };
        builder
            .spawn(NodeBundle {
                style: Style {
                    justify_content: JustifyContent::End,
                    ..Default::default()
                },
                ..Default::default()
            })
            .with_children(|builder| {
                builder.spawn((
                    NodeBundle {
                        style: Style {
                            width: Val::Px(32.),
                            height: Val::Px(32.),
                            ..Default::default()
                        },
                        background_color: BackgroundColor(Color::WHITE),
                        ..Default::default()
                    },
                    UiImage::new(asset_server.load(currency_type.get_image_name())),
                ));
                builder.spawn((
                    currency_type.clone(),
                    DisplayType::Value,
                    TextBundle::from_section("", text_style.clone()),
                ));
                builder.spawn(TextBundle::from_section("/", text_style.clone()));
                builder.spawn((
                    currency_type.clone(),
                    DisplayType::Limit,
                    TextBundle::from_section("", text_style.clone()),
                ));
            });
        builder
            .spawn(NodeBundle {
                style: Style {
                    justify_content: JustifyContent::End,
                    ..Default::default()
                },
                ..Default::default()
            })
            .with_children(|builder| {
                builder.spawn(TextBundle::from_section("+", text_style.clone()));
                builder.spawn((
                    currency_type,
                    DisplayType::Inflow,
                    TextBundle::from_section("", text_style.clone()),
                ));
                builder.spawn(TextBundle::from_section("/min", text_style.clone()));
            });
    });*/
}

pub fn refresh_display(
    storage: Res<CurrencyStorage>,
    mut display_q: Query<(&CurrencyType, &DisplayType, &mut Text)>,
) {
    for (currency, display, mut text) in display_q.iter_mut() {
        text.sections[0].value = match display {
            //DisplayType::Inflow => currency.inflow.to_string(),
            DisplayType::Value => {
                if *currency == CURRENCY_HONEY {
                    format!(
                        "{}/{}",
                        storage.stored[currency.0], storage.max_stored[currency.0]
                    )
                } else {
                    format!("{}", storage.stored[currency.0])
                }
            }
            DisplayType::Limit => storage.max_stored[currency.0].to_string(),
            DisplayType::Inflow => format!("+{}/min", storage.estimated_inflow[currency.0]),
        }
    }
}
