use crate::core::{
    ui::button::{spawn_button, EnableButtonWhenHaveMoney},
    Building, BuildingKind, CurrencyValues, HiveBuildings, RelativePixelSized, CURRENCY_NUM,
};
use bevy::prelude::*;

#[derive(Component)]
pub struct BuildButton {
    pub kind: BuildingKind,
    pub index: usize,
}

pub fn spawn_building_menu(
    builder: &mut ChildBuilder,
    asset_server: &AssetServer,
    hive: &HiveBuildings,
    index: usize,
) {
    let buildable = vec![
        BuildingKind::Workshop,
        BuildingKind::Armory,
        BuildingKind::Storage,
        BuildingKind::WaxReactor,
        BuildingKind::MagicWaxReactor,
    ];

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
            RelativePixelSized {
                width: 114,
                height: 141,
            },
            UiImage::new(asset_server.load("images/BuildMenu.png")),
        ))
        .with_children(|builder| {
            for kind in buildable {
                let child: Option<Transform> = None;
                let cost = hive.get_build_cost(kind);
                let mut a = if cost[0] != 0 {
                    Some(cost[0])
                } else {
                    Some(cost[1])
                };
                let mut b = if cost[0] != 0 {
                    Some(cost[1])
                } else {
                    Some(cost[2])
                };
                let mut cost = hive.get_build_cost(kind);

                if kind == BuildingKind::Storage && hive.storages >= hive.get_max_storages() {
                    // Can't build more storages
                    a = Some(0);
                    b = Some(0);
                    cost = [99999999; CURRENCY_NUM];
                }

                spawn_button(
                    builder,
                    &kind.to_string(),
                    (
                        BuildButton { kind, index },
                        EnableButtonWhenHaveMoney { target: cost },
                    ),
                    asset_server,
                    child,
                    a,
                    b,
                    50.0,
                    17.0,
                );
            }
        });
}
