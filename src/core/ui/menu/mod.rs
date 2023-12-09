use crate::core::{
    get_building_image_name, Building, BuildingKind, HiveBuildings, UniversalMaterial,
};

use self::{
    building_menu::{spawn_building_menu, BuildButton},
    title::{spawn_title, TitleItem},
    upgrade_menu::{spawn_upgrage_menu, DestroyButton, OrderButton, UpgradeButton},
};

use super::{
    button::{MyButton, PrevInteraction},
    constants,
    moving_ui::{MovingUi, Target},
    UiSize,
};
use bevy::prelude::*;

mod building_menu;
mod title;
mod upgrade_menu;

#[derive(Component, Default)]
pub struct Menu {
    pub focus_building: Option<usize>,
}

#[derive(Component)]
pub struct MenuContent {}

pub fn spawn_menu(builder: &mut ChildBuilder, asset_server: &mut AssetServer) {
    builder
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    right: Val::Px(0.),
                    height: Val::Percent(100.),
                    padding: UiRect::right(Val::Percent(1.0)),
                    flex_direction: FlexDirection::Column,
                    ..Default::default()
                },
                //background_color: BackgroundColor(constants::background_color()),
                //border_color: BorderColor(constants::border_color()),
                ..Default::default()
            },
            MovingUi {
                target: Target {
                    right: -400.,
                    ..Default::default()
                },
            },
            Interaction::default(),
            Menu::default(),
        ))
        .with_children(|builder| {
            //spawn_title(builder, asset_server);
            builder.spawn((
                MenuContent {},
                NodeBundle {
                    style: Style {
                        width: Val::Percent(100.),
                        margin: UiRect {
                            left: Val::ZERO,
                            right: Val::ZERO,
                            top: Val::Auto,
                            bottom: Val::Auto,
                        },
                        ..Default::default()
                    },
                    ..Default::default()
                },
            ));
        });
}

pub fn menu_helper(
    mut building_menu_query: Query<&mut Menu>,
    mut hive_buildings: ResMut<HiveBuildings>,
) {
    if let Some(mut menu) = building_menu_query.iter_mut().next() {
        if hive_buildings.any_order_done {
            menu.focus_building = None;
            hive_buildings.any_order_done = false;
        }
        if hive_buildings.any_upgrade_done {
            menu.focus_building = menu.focus_building;
            hive_buildings.any_upgrade_done = false;
        }
    }
}

pub fn menu_update(
    mut commands: Commands,
    mut building_menu_query: Query<(&Menu, Changed<Menu>, Option<&mut MovingUi>)>,
    mut title: Query<&mut Text, With<TitleItem>>,
    mut image: Query<&mut UiImage, With<TitleItem>>,
    mut content: Query<Entity, With<MenuContent>>,
    ui_size: Res<UiSize>,
    buildings: Query<&Building>,
    asset_server: ResMut<AssetServer>,
    mut time_since_closing: Local<Option<f32>>,
    mut content_exists: Local<bool>,
    hive_buildings: Res<HiveBuildings>,
    time: Res<Time>,
    mut materials: ResMut<Assets<UniversalMaterial>>,
) {
    if content.iter().count() == 0 {
        return;
    }

    let content = content.single();

    if let Some(mut t) = time_since_closing.as_mut() {
        *t += time.delta_seconds();
    }

    if let Some((mut menu, menu_changed, mut maybe_moving)) = building_menu_query.iter_mut().next()
    {
        if menu_changed {
            // If menu has changed - hide it and wait until it hides.
            if let Some(mut moving) = maybe_moving.as_mut() {
                moving.target = Target {
                    right: -((ui_size.size * 120) as f32),
                    ..Default::default()
                };
            }
            if *content_exists {
                *time_since_closing = Some(0.0);
            }
        }

        if let Some(t) = time_since_closing.as_ref() {
            if *t > 0.3 {
                // Succesfully closed
                commands.entity(content).despawn_descendants();
                *time_since_closing = None;
                *content_exists = false;
            }
        }

        if time_since_closing.is_none() && !*content_exists {
            if let Some(building_idx) = menu.focus_building {
                if let Some(building) = buildings
                    .into_iter()
                    .filter(|bld| bld.index == building_idx && bld.kind != BuildingKind::Storage)
                    .nth(0)
                {
                    *content_exists = true;
                    if let Some(mut moving) = maybe_moving.as_mut() {
                        moving.target = Target::default();
                    }
                    for mut title in title.iter_mut() {
                        title.sections[0].value = building.kind.to_string();
                    }
                    for mut image in image.iter_mut() {
                        image.texture = asset_server.load(get_building_image_name(building.kind));
                    }

                    if building.kind == BuildingKind::None {
                        commands.entity(content).with_children(|e| {
                            spawn_building_menu(e, &asset_server, &hive_buildings, building_idx)
                        });
                    } else {
                        commands.entity(content).with_children(|e| {
                            spawn_upgrage_menu(
                                e,
                                building_idx,
                                &hive_buildings,
                                &asset_server,
                                &mut materials,
                            )
                        });
                    };
                }
            }
        }
    }
}

pub fn click_button_system(
    mut order_interactions: Query<
        (
            &Interaction,
            &mut PrevInteraction,
            &MyButton,
            Option<&OrderButton>,
            Option<&BuildButton>,
            Option<&UpgradeButton>,
            Option<&DestroyButton>,
        ),
        Changed<Interaction>,
    >,
    mut buildings: Query<&mut Building>,
    mut hive_buildings: ResMut<HiveBuildings>,
) {
    for (
        interaction,
        mut prev_interaction,
        my_button,
        maybe_order,
        maybe_build,
        maybe_upgrade,
        maybe_destroy,
    ) in order_interactions.iter_mut()
    {
        if *interaction == Interaction::Hovered
            && prev_interaction.0 == Interaction::Pressed
            && my_button.enabled
        {
            // Order button
            if let Some(order) = maybe_order {
                for mut b in buildings.iter_mut() {
                    if b.index != order.building_index {
                        continue;
                    }
                    b.order();
                }
            }

            // Build button
            if let Some(build) = maybe_build {
                hive_buildings.build_order = Some((build.kind, build.index));
            }
            // Upgrade button
            if let Some(upgrade) = maybe_upgrade {
                hive_buildings.upgrade_order = Some(upgrade.building_index);
            }
            // Destroy button
            if let Some(destroy) = maybe_destroy {
                hive_buildings.destroy_order = Some(destroy.building_index);
            }
        }

        prev_interaction.0 = *interaction;
    }
}
