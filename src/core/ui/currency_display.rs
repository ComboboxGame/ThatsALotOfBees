use super::constants;
use crate::core::{CurrencyStorage, CurrencyType, CURRENCY_NUM};
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
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|builder| {
            builder
                .spawn(NodeBundle {
                    style: Style {
                        position_type: PositionType::Absolute,
                        bottom: Val::Px(0.),
                        padding: UiRect::vertical(Val::Px(5.)),
                        justify_content: JustifyContent::Center,
                        border: UiRect::all(Val::Px(2.)),
                        ..Default::default()
                    },
                    background_color: BackgroundColor(constants::background_color()),
                    border_color: BorderColor(constants::border_color()),
                    ..Default::default()
                })
                .with_children(|builder| {
                    for currency in 0..CURRENCY_NUM {
                        spawn_currency_counter(builder, CurrencyType(currency), asset_server);
                    }
                });
        });
}

pub fn spawn_currency_counter(
    builder: &mut ChildBuilder,
    currency_type: CurrencyType,
    asset_server: &mut AssetServer,
) {
    builder
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
        });
}

pub fn refresh_display(
    storage: Res<CurrencyStorage>,
    mut display_q: Query<(&CurrencyType, &DisplayType, &mut Text)>,
) {
    for (currency, display, mut text) in display_q.iter_mut() {
        text.sections[0].value = match display {
            //DisplayType::Inflow => currency.inflow.to_string(),
            DisplayType::Value => storage.stored[currency.0].to_string(),
            DisplayType::Limit => storage.max_stored[currency.0].to_string(),
            DisplayType::Inflow => storage.estimated_inflow[currency.0].to_string(),
        }
    }
}
