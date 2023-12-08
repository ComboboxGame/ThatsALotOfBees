use crate::core::{get_building_image_name, ui::constants, BuildingKind};
use bevy::prelude::*;

#[derive(Component)]
pub struct TitleItem {}

pub fn spawn_title(builder: &mut ChildBuilder, asset_server: &mut AssetServer) {
    builder
        .spawn((NodeBundle {
            style: Style {
                width: Val::Percent(100.),
                height: Val::Percent(10.),
                padding: UiRect::all(Val::Px(3.)),
                ..Default::default()
            },
            background_color: BackgroundColor(constants::border_color()),
            ..Default::default()
        },))
        .with_children(|builder| {
            builder.spawn((
                TitleItem {},
                NodeBundle {
                    style: Style {
                        width: Val::Px(80.),
                        height: Val::Percent(100.),
                        ..Default::default()
                    },
                    background_color: BackgroundColor(Color::WHITE),
                    ..Default::default()
                },
                UiImage::new(asset_server.load(get_building_image_name(BuildingKind::default()))),
            ));
            builder
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Center,
                        flex_grow: 1.,
                        padding: UiRect {
                            left: Val::Px(5.),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .with_children(|builder| {
                    builder.spawn((
                        TitleItem {},
                        TextBundle::from_section(
                            "",
                            TextStyle {
                                font_size: 36.,
                                color: constants::background_color(),
                                ..Default::default()
                            },
                        ),
                    ));
                });
        });
}

