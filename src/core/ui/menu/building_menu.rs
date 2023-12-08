use bevy::prelude::*;
use crate::core::{BuildingKind, ui::button::spawn_button};

pub fn spawn_building_menu(builder: &mut ChildBuilder) {
    let buildable = vec![
        BuildingKind::Nexus,
        BuildingKind::Storage,
        BuildingKind::WaxReactor,
        BuildingKind::Armory,
        BuildingKind::Workshop,
        BuildingKind::BuilderAcademy,
    ];

    builder
        .spawn((NodeBundle {
            style: Style {
                width: Val::Percent(100.),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(10.)),
                ..Default::default()
            },
            ..Default::default()
        },))
        .with_children(|builder| {
            for b in buildable {
                spawn_button(builder, &b.to_string(), ());
            }
        });
}
