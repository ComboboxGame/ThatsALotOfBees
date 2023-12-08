use bevy::prelude::*;

use crate::core::ui::{button::spawn_button, constants};

#[derive(Component)]
struct OrderButton {
    building_index: usize
}

pub fn spawn_upgrage_menu(builder: &mut ChildBuilder, building_index: usize) {
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
                spawn_button(
                    builder,
                    "Order 1",
                    OrderButton {
                        building_index
                    },
                );
            spawn_button(builder, "Upgrade", ());
            spawn_button(builder, "Destroy", ());
        });
}

fn interaction_system(
    order_interactions: Query<(&Interaction, &OrderButton), Changed<Interaction>>,
) {
    for (interaction, _button) in order_interactions.iter() {
        match interaction {
            Interaction::Pressed => {
                
            }
            _ => {}
        }
    }
}
