use bevy::prelude::*;

use crate::core::{
    ui::{
        button::{spawn_button, EnableButtonWhenHaveMoney},
        constants,
    },
    Building, BuildingKind, HiveBuildings, RelativePixelSized, UniversalMaterial, CURRENCY_NUM, MAX_WORKER_LEVEL, MAX_DEFENDER_LEVEL,
};

#[derive(Component)]
pub struct OrderButton {
    pub building_index: usize,
}

#[derive(Component)]
pub struct UpgradeButton {
    pub building_index: usize,
}

#[derive(Component)]
pub struct DestroyButton {
    pub building_index: usize,
}

pub fn spawn_upgrage_menu(
    builder: &mut ChildBuilder,
    building_index: usize,
    hive_buildings: &HiveBuildings,
    asset_server: &AssetServer,
    materials: &mut Assets<UniversalMaterial>,
) {
    let kind = hive_buildings.buildings[building_index];
    let (width, height) = kind.get_menu_size();
    builder
        .spawn((
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    ..Default::default()
                },
                background_color: BackgroundColor(Color::WHITE),
                ..Default::default()
            },
            RelativePixelSized { width, height },
            UiImage::new(asset_server.load(kind.get_menu_image())),
        ))
        .with_children(|builder| {
            let cur_child = if kind == BuildingKind::Armory {
                Some(hive_buildings.get_current_defender())
            } else if kind == BuildingKind::Workshop {
                Some(hive_buildings.get_current_worker())
            } else {
                None
            };
            let next_child = if kind == BuildingKind::Armory {
                Some(hive_buildings.get_next_defender())
            } else if kind == BuildingKind::Workshop {
                Some(hive_buildings.get_next_worker())
            } else {
                None
            };

            let cur_child = cur_child.map(|bee| {
                (
                    MaterialNodeBundle::<UniversalMaterial> {
                        style: Style {
                            position_type: PositionType::Absolute,
                            left: Val::ZERO,
                            top: Val::ZERO,
                            ..default()
                        },
                        z_index: ZIndex::Global(20),
                        material: materials.add(bee.into()),
                        ..MaterialNodeBundle::default()
                    },
                    RelativePixelSized {
                        width: 32,
                        height: 32,
                    },
                )
            });
            let next_child = next_child.map(|bee| {
                (
                    MaterialNodeBundle::<UniversalMaterial> {
                        style: Style {
                            position_type: PositionType::Absolute,
                            left: Val::ZERO,
                            top: Val::ZERO,
                            ..default()
                        },
                        z_index: ZIndex::Global(20),
                        material: materials.add(bee.into()),
                        ..MaterialNodeBundle::default()
                    },
                    RelativePixelSized {
                        width: 32,
                        height: 32,
                    },
                )
            });

            let cost = hive_buildings.get_order_cost(kind);
            let mut a = if cost[0] != 0 {
                Some(cost[0])
            } else if cost[1] != 0 {
                Some(cost[1])
            } else {
                Some(cost[2])
            };
            let mut b = if cost[1] != 0 && (cost[0] != 0) {
                Some(cost[1])
            } else if cost[2] != 0 && (cost[1] != 0 || cost[0] != 0) {
                Some(cost[2])
            } else {
                None
            };

            spawn_button(
                builder,
                hive_buildings.get_order_name(kind),
                (
                    OrderButton { building_index },
                    EnableButtonWhenHaveMoney { target: cost },
                ),
                asset_server,
                cur_child,
                a,
                b,
                if kind != BuildingKind::WaxReactor && kind != BuildingKind::MagicWaxReactor {
                    50.0
                } else {
                    73.0
                },
                if kind != BuildingKind::WaxReactor && kind != BuildingKind::MagicWaxReactor {
                    17.0
                } else {
                    18.0
                },
            );

            if kind != BuildingKind::WaxReactor && kind != BuildingKind::MagicWaxReactor && kind != BuildingKind::Nexus {
                let mut cost = hive_buildings.get_upgrade_cost(kind);
                let mut a = if cost[0] != 0 {
                    Some(cost[0])
                } else if cost[1] != 0 {
                    Some(cost[1])
                } else {
                    Some(cost[2])
                };
                let mut b = if cost[1] != 0 && (cost[0] != 0) {
                    Some(cost[1])
                } else if cost[2] != 0 && (cost[1] != 0 || cost[0] != 0) {
                    Some(cost[2])
                } else {
                    None
                };

                if kind == BuildingKind::Workshop && hive_buildings.worker_lvl >= MAX_WORKER_LEVEL - 1 ||
                    kind == BuildingKind::Armory && hive_buildings.defender_lvl >= MAX_DEFENDER_LEVEL - 1 {
                    a = Some(0);
                    b = Some(0);
                    cost = [99999999; CURRENCY_NUM];
                }

                spawn_button(
                    builder,
                    hive_buildings.get_upgrade_name(kind),
                    (
                        UpgradeButton { building_index },
                        EnableButtonWhenHaveMoney { target: cost },
                    ),
                    asset_server,
                    next_child,
                    a,
                    b,
                    50.0,
                    17.0,
                );
            }

            if kind != BuildingKind::Nexus {
                let child: Option<Transform> = None;
                spawn_button(
                    builder,
                    "Destroy",
                    (
                        DestroyButton { building_index },
                        EnableButtonWhenHaveMoney::default(),
                    ),
                    &asset_server,
                    child,
                    None,
                    None,
                    50.0,
                    17.0,
                );
            }
        });
}
