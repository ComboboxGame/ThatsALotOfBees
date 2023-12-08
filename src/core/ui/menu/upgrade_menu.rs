use bevy::prelude::*;

use crate::core::ui::{constants, button::spawn_button};

pub fn spawn_upgrage_menu(builder: &mut ChildBuilder) {
    builder
        .spawn((NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(10.)),
                ..Default::default()
            },
            ..Default::default()
        },))
        .with_children(|builder| {
            builder
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.),
                        margin: UiRect::bottom(Val::Px(10.)),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .with_children(|builder| {
                    builder.spawn(TextBundle::from_section(
                        "Some description of the building",
                        TextStyle {
                            font_size: 24.,
                            color: constants::border_color(),
                            ..Default::default()
                        },
                    ));
                });
            spawn_button(builder, "Upgrade", ());
            spawn_button(builder, "Destroy", ());
        });
}