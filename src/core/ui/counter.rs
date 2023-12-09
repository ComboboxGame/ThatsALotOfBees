use crate::core::{BeeType, HiveBuildings, LivingCreature, UniversalMaterial, FONT_HANDLE, BuildingKind, BUILDINGS_NUM, Building, MouseState, CurrencyGainPerMinute};
use bevy::{prelude::*, ui::FocusPolicy};
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

#[derive(Component)]
pub struct StatsMenu;
#[derive(Component)]
pub enum StatsMenuVal {
    Attack,
    Cooldown,
    Health,
    Honey,
    Wax,
}

#[derive(Component)]
pub struct Stub1;

#[derive(Component)]
pub struct StatsTarget {
    pub bee: Option<BeeType>,
    pub attack: u32,
    pub cooldown: f32,
    pub health: u32,
    pub honey: u32,
    pub wax: u32,
}

pub fn setup_bee_counters(
    builder: &mut ChildBuilder,
    mut materials: ResMut<Assets<UniversalMaterial>>,
    mut asset_server: &mut AssetServer,
) {
    builder.spawn(
        (NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                left: Val::Px(0.0),
                top: Val::Px(0.0),
                ..Default::default()
            },
            z_index: ZIndex::Global(100),
            background_color: BackgroundColor(Color::WHITE),
            ..default()
        },
        UiImage::new(asset_server.load("images/StatsMenu.png")),
        StatsMenu,
    RelativePixelSized {
        width: 72,
        height: 24,
    })
    ).with_children(|builder| {
        builder.spawn((
            TextBundle {
                text: Text::from_section("x", TextStyle {font: FONT_HANDLE, font_size: 16.0, color: constants::border_color()}),
                style: Style {
                    position_type: PositionType::Absolute,
                    right: Val::Percent(79.0),
                    top: Val::Percent(11.0),
                    ..default()
                },
                ..default()
            },
            StatsMenuVal::Attack,
            Stub1,
            RelativePixelFont {
                size: 12
            }
        ));        builder.spawn((
            TextBundle {
                text: Text::from_section("y", TextStyle {font: FONT_HANDLE, font_size: 16.0, color: constants::border_color()}),
                style: Style {
                    position_type: PositionType::Absolute,
                    right: Val::Percent(45.0),
                    top: Val::Percent(11.0),
                    ..default()
                },
                ..default()
            },
            StatsMenuVal::Cooldown,
            Stub1,
            RelativePixelFont {
                size: 12
            }
        ));        builder.spawn((
            TextBundle {
                text: Text::from_section("z", TextStyle {font: FONT_HANDLE, font_size: 16.0, color: constants::border_color()}),
                style: Style {
                    position_type: PositionType::Absolute,
                    right: Val::Percent(16.0),
                    top: Val::Percent(11.0),
                    ..default()
                },
                ..default()
            },
            StatsMenuVal::Health,
            Stub1,
            RelativePixelFont {
                size: 12
            }
        ));builder.spawn((
            TextBundle {
                text: Text::from_section("h", TextStyle {font: FONT_HANDLE, font_size: 16.0, color: constants::border_color()}),
                style: Style {
                    position_type: PositionType::Absolute,
                    right: Val::Percent(61.0),
                    top: Val::Percent(50.0),
                    ..default()
                },
                ..default()
            },
            StatsMenuVal::Honey,
            Stub1,
            RelativePixelFont {
                size: 12
            }
        ));builder.spawn((
            TextBundle {
                text: Text::from_section("w", TextStyle {font: FONT_HANDLE, font_size: 16.0, color: constants::border_color()}),
                style: Style {
                    position_type: PositionType::Absolute,
                    right: Val::Percent(30.0),
                    top: Val::Percent(50.0),
                    ..default()
                },
                ..default()
            },
            StatsMenuVal::Wax,
            Stub1,
            RelativePixelFont {
                size: 12
            }
        ));
    });

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
    let creature = LivingCreature::from(kind);
    let gain = CurrencyGainPerMinute::from(kind).gain;
    builder
        .spawn((
            NodeBundle { 
                focus_policy: FocusPolicy::Block,
                ..default()
            },
            RelativePixelSized {
                width: 64,
                height: 30,
            },
            StatsTarget {
                bee: Some(kind),
                attack: creature.attack_damage,
                cooldown: creature.attack_cooldown,
                health: creature.health as u32,
                honey: gain[0] as u32,
                wax: gain[1] as u32,
            },
            Interaction::default(),
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
                RelativePixelFont { size: 10 },
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
    mut counters_text: Query<(&mut BeeCounter, &mut Text), (Without<Handle<UniversalMaterial>>, Without<StatsMenuVal>)>,
    mut counters_images: Query<(&mut BeeCounter, &mut Handle<UniversalMaterial>), Without<Text>>,
    mut future_counters_text: Query<(&mut BeeFutureCounter, &mut Text), (Without<BeeCounter>, Without<StatsMenuVal>)>,
    mut stats_targets: Query<(&mut StatsTarget, &Interaction)>,
    mut stats: Query<&mut Style, With<StatsMenu>>,
    mut stats_val: Query<(&mut Text, &StatsMenuVal)>,
    buildings: Query<&Building>,
    hive_buildings: Res<HiveBuildings>,
    mut materials: ResMut<Assets<UniversalMaterial>>,
    mouse: Res<MouseState>,
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

    if stats.is_empty() {
        return;
    }

    let mut stats = stats.single_mut();

    let mut any = false;
    for (mut stats_target, interaction) in stats_targets.iter_mut() {
        if *interaction == Interaction::None {
            continue;
        }
        any = true;

        if let Some(pos) = mouse.screen_position {
            stats.left = Val::Px(pos.x);
            stats.top = Val::Px(pos.y);
        }

        if let Some(bee) = stats_target.bee {
            if bee != hive_buildings.get_current_level(bee) {
                stats_target.bee = Some(hive_buildings.get_current_level(bee));
                let creature = LivingCreature::from(stats_target.bee.unwrap());
                stats_target.attack = creature.attack_damage;
                stats_target.cooldown = creature.attack_cooldown;
                stats_target.health = creature.health as u32;
                let gain = CurrencyGainPerMinute::from(stats_target.bee.unwrap()).gain;
                stats_target.honey = gain[0] as u32;
                stats_target.wax = gain[1] as u32;
            }
        }

        for (mut text, val) in stats_val.iter_mut() {
            match val {
                StatsMenuVal::Attack => text.sections[0].value = stats_target.attack.to_string(),
                StatsMenuVal::Cooldown => text.sections[0].value = format!("{:.1}", stats_target.cooldown),
                StatsMenuVal::Health => text.sections[0].value = stats_target.health.to_string(),
                StatsMenuVal::Honey => text.sections[0].value = stats_target.honey.to_string(),
                StatsMenuVal::Wax => text.sections[0].value = stats_target.wax.to_string(),
            }
        }
        break;
    }

    if !any {
        stats.left = Val::Px(-1000.0);
        stats.top = Val::Px(-1000.0);
    }
}
