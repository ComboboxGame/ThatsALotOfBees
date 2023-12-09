use crate::core::{BeeType, HiveBuildings, LivingCreature, UniversalMaterial, FONT_HANDLE, BuildingKind, BUILDINGS_NUM, Building};
use bevy::prelude::*;
use strum::IntoEnumIterator;

use super::{constants, RelativePixelFont, RelativePixelPositioned, RelativePixelSized};

#[derive(Component)]
pub struct BeeCounter {
    kind: BeeType,
}

#[derive(Component)]
pub struct BeeFutureCounter {
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
                left: Val::Percent(1.0),
                height: Val::Percent(100.),
                align_items: AlignItems::Start,
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
                            margin: UiRect::new(Val::ZERO, Val::ZERO, Val::Auto, Val::Auto),
                            flex_direction: FlexDirection::Column,
                            ..Default::default()
                        },
                        background_color: BackgroundColor(Color::WHITE),
                        ..Default::default()
                    },
                    UiImage::new(asset_server.load("images/CounterMenu.png")),
                    RelativePixelSized {
                        width: 64,
                        height: 154,
                    },
                ))
                .with_children(|builder| {
                    for kind in [
                        BeeType::Baby,
                        BeeType::Regular,
                        BeeType::Worker(0),
                        BeeType::Defender(0),
                    ] {
                        spawn_bee_counter(builder, kind, &mut asset_server, &mut materials, 12.0)
                    }
                    // Padding
                    builder.spawn((
                        NodeBundle { ..default() },
                        RelativePixelSized {
                            width: 30,
                            height: 4,
                        },
                    ));
                    spawn_bee_counter(
                        builder,
                        BeeType::Queen,
                        &mut asset_server,
                        &mut materials,
                        12.0,
                    )
                });
        });
}

fn spawn_bee_counter(
    builder: &mut ChildBuilder,
    kind: BeeType,
    _asset_server: &mut AssetServer,
    materials: &mut Assets<UniversalMaterial>,
    right: f32,
) {
    builder
        .spawn((
            NodeBundle { ..default() },
            RelativePixelSized {
                width: 64,
                height: 30,
            },
        ))
        .with_children(|builder| {
            builder.spawn((
                MaterialNodeBundle::<UniversalMaterial> {
                    style: Style {
                        position_type: PositionType::Absolute,
                        left: Val::Percent(4.0),
                        top: Val::Percent(-4.0),
                        ..default()
                    },
                    material: materials.add(kind.into()),
                    ..MaterialNodeBundle::default()
                },
                BeeCounter { kind },
                RelativePixelSized {
                    width: 32,
                    height: 32,
                },
            ));

            builder.spawn((
                BeeCounter { kind },
                TextBundle::from_section(
                    "0",
                    TextStyle {
                        font: FONT_HANDLE,
                        font_size: 24.,
                        color: constants::border_color(),
                        ..Default::default()
                    },
                )
                .with_style(Style {
                    position_type: PositionType::Absolute,
                    right: Val::Percent(right),
                    top: Val::Percent(18.0),
                    ..default()
                }),
                RelativePixelFont { size: 16 },
            ));

            builder.spawn((
                BeeFutureCounter { kind },
                TextBundle::from_section(
                    "",
                    TextStyle {
                        font: FONT_HANDLE,
                        font_size: 24.,
                        color: constants::border_color(),
                        ..Default::default()
                    },
                )
                .with_style(Style {
                    position_type: PositionType::Absolute,
                    right: Val::Percent(right + 2.0),
                    bottom: Val::Percent(12.0),
                    ..default()
                }),
                RelativePixelFont { size: 8 },
            ));
        });

    /*builder
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
    });*/
}

pub fn update_counter(
    bees: Query<(&BeeType, &LivingCreature)>,
    mut counters_text: Query<(&mut BeeCounter, &mut Text), Without<Handle<UniversalMaterial>>>,
    mut counters_images: Query<(&mut BeeCounter, &mut Handle<UniversalMaterial>), Without<Text>>,
    mut future_counters_text: Query<(&mut BeeFutureCounter, &mut Text), Without<BeeCounter>>,
    buildings: Query<&Building>,
    hive_buildings: Res<HiveBuildings>,
    mut materials: ResMut<Assets<UniversalMaterial>>,
) {
    for (mut counter, mut text) in counters_text.iter_mut() {
        if counter.kind == BeeType::Queen {
            let health = bees
                .iter()
                .filter(|(bee, _)| **bee == counter.kind)
                .next()
                .map_or(0, |(_, c)| c.health);
            text.sections[0].value = health.to_string();
        } else {
            counter.kind = hive_buildings.get_current_level(counter.kind);
            let bees_of_kind = bees.iter().filter(|(bee, _)| **bee == counter.kind).count();
            text.sections[0].value = bees_of_kind.to_string();
        }
    }
    for (mut counter, mut material) in counters_images.iter_mut() {
        if counter.kind != hive_buildings.get_current_level(counter.kind) {
            counter.kind = hive_buildings.get_current_level(counter.kind);
            *material = materials.add(counter.kind.into());
        }
    }
    for (mut counter, mut text) in future_counters_text.iter_mut() {
        counter.kind = hive_buildings.get_current_level(counter.kind);
        let target_building = match counter.kind {
            BeeType::Baby => BuildingKind::Nexus,
            BeeType::Regular => BuildingKind::None,
            BeeType::Worker(_) => BuildingKind::Workshop,
            BeeType::Defender(_) => BuildingKind::Armory,
            BeeType::Queen => BuildingKind::None,
        };
        let mut total = 0;
        for b in buildings.iter() {
            if b.kind == target_building {
                total += b.orders_count;
            }
        }
        if total == 0 {
            text.sections[0].value = "".to_string();
        } else {
            text.sections[0].value = format!("+{}", total);
        }
    }
}
