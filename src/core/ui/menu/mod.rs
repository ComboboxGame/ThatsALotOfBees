use crate::core::{get_building_image_name, Building, BuildingKind};

use self::{
    building_menu::spawn_building_menu,
    title::{spawn_title, TitleItem},
    upgrade_menu::spawn_upgrage_menu,
};

use super::{
    constants,
    moving_ui::{MovingUi, Target},
};
use bevy::prelude::*;

mod building_menu;
mod title;
mod upgrade_menu;

pub use upgrade_menu::order_button_system;

#[derive(Component, Default)]
pub struct Menu {
    pub focus_building: Option<usize>,
}

#[derive(Component)]
pub struct MenuContent {}

pub fn spawn_menu(builder: &mut ChildBuilder, asset_server: &mut AssetServer) {
    builder
        .spawn((
            ButtonBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    right: Val::Px(-400.),
                    height: Val::Percent(100.),
                    width: Val::Px(400.),
                    border: UiRect::all(Val::Px(2.)),
                    flex_direction: FlexDirection::Column,
                    ..Default::default()
                },
                background_color: BackgroundColor(constants::background_color()),
                border_color: BorderColor(constants::border_color()),
                ..Default::default()
            },
            MovingUi {
                target: Target {
                    right: -400.,
                    ..Default::default()
                },
            },
            Menu::default(),
        ))
        .with_children(|builder| {
            spawn_title(builder, asset_server);
            builder.spawn((
                MenuContent {},
                NodeBundle {
                    style: Style {
                        width: Val::Percent(100.),
                        ..Default::default()
                    },
                    ..Default::default()
                },
            ));
        });
}

pub fn menu_update(
    mut commands: Commands,
    mut building_menu_query: Query<(&Menu, &mut MovingUi), Changed<Menu>>,
    mut title: Query<&mut Text, With<TitleItem>>,
    mut image: Query<&mut UiImage, With<TitleItem>>,
    mut content: Query<Entity, With<MenuContent>>,
    buildings: Query<&Building>,
    asset_server: ResMut<AssetServer>,
) {
    if building_menu_query.iter().count() > 0 {
        let (menu, mut moving) = building_menu_query.single_mut();
        let content = content.single_mut();
        moving.target = Target {
            right: -400.,
            ..Default::default()
        };
        commands.entity(content).despawn_descendants();
        if let Some(building_idx) = menu.focus_building {
            if let Some(building) = buildings
                .into_iter()
                .filter(|bld| bld.index == building_idx)
                .nth(0)
            {
                moving.target = Target::default();
                title.single_mut().sections[0].value = building.kind.to_string();
                image.single_mut().texture =
                    asset_server.load(get_building_image_name(building.kind));

                if building.kind == BuildingKind::None {
                    commands.entity(content).with_children(spawn_building_menu);
                } else {
                    commands
                        .entity(content)
                        .with_children(|e| spawn_upgrage_menu(e, building_idx));
                };
            }
        }
    }
}
