use crate::core::{BeeType, UniversalMaterial};
use bevy::prelude::*;
use strum::IntoEnumIterator;

use super::constants;

#[derive(Component)]
pub struct BeeCounter {
    kind: BeeType,
}

pub fn setup_bee_counters(
    builder: &mut ChildBuilder,
    mut materials: ResMut<Assets<UniversalMaterial>>,
    mut asset_server: &mut AssetServer,
) {
    builder
        .spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                flex_direction: FlexDirection::Column,
                height: Val::Percent(100.),
                align_items: AlignItems::Start,
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
                        flex_direction: FlexDirection::Column,
                        height: Val::Percent(100.),
                        align_items: AlignItems::Start,
                        justify_content: JustifyContent::Center,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .with_children(|builder| {
                    for kind in BeeType::iter() {
                        spawn_bee_counter(builder, kind, &mut asset_server, &mut materials)
                    }
                });
        });
}

fn spawn_bee_counter(
    builder: &mut ChildBuilder,
    kind: BeeType,
    asset_server: &mut AssetServer,
    materials: &mut Assets<UniversalMaterial>,
) {
    builder
        .spawn(NodeBundle {
            style: Style {
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                margin: UiRect {
                    top: Val::Px(10.),
                    ..Default::default()
                },
                padding: UiRect::all(Val::Px(5.)),
                border: UiRect::all(Val::Px(2.)),
                ..default()
            },
            background_color: BackgroundColor(constants::background_color()),
            border_color: BorderColor(constants::border_color()),
            ..default()
        })
        .with_children(|builder| {
            builder.spawn((MaterialNodeBundle::<UniversalMaterial> {
                style: Style {
                    width: Val::Px(60.0),
                    height: Val::Px(60.0),
                    ..default()
                },
                material: materials.add(kind.into()),
                ..MaterialNodeBundle::default()
            },));
            builder
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Px(40.),
                        justify_content: JustifyContent::Center,
                        align_content: AlignContent::Center,
                        margin: UiRect {
                            right: Val::Px(5.),
                            ..Default::default()
                        },
                        border: UiRect::all(Val::Px(2.)),
                        ..Default::default()
                    },
                    background_color: BackgroundColor(Color::WHITE),
                    border_color: BorderColor(constants::border_color()),
                    ..Default::default()
                })
                .with_children(|builder| {
                    builder.spawn((
                        BeeCounter { kind },
                        TextBundle::from_section(
                            "0",
                            TextStyle {
                                font_size: 24.,
                                color: constants::border_color(),
                                ..Default::default()
                            },
                        ),
                    ));
                });
        });
}

pub fn update_counter(bees: Query<&BeeType>, mut counters: Query<(&BeeCounter, &mut Text)>) {
    for (counter, mut text) in counters.iter_mut() {
        let bees_of_kind = bees.iter().filter(|bee| **bee == counter.kind).count();
        text.sections[0].value = bees_of_kind.to_string();
    }
}
