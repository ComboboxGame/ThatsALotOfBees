use super::constants;
use bevy::prelude::*;

#[derive(Component)]
pub struct Button {}

pub fn spawn_button<T: Bundle>(builder: &mut ChildBuilder, title: &str, components: T) {
    builder
        .spawn((
            components,
            Button {},
            ButtonBundle {
                style: Style {
                    width: Val::Percent(100.),
                    border: UiRect::all(Val::Px(2.)),
                    padding: UiRect::all(Val::Px(5.)),
                    margin: UiRect::bottom(Val::Px(10.)),
                    justify_content: JustifyContent::Center,
                    ..Default::default()
                },
                border_color: BorderColor(constants::border_color()),
                background_color: BackgroundColor(constants::button_color()),
                ..Default::default()
            },
        ))
        .with_children(|builder| {
            builder.spawn(TextBundle::from_section(
                title,
                TextStyle {
                    font_size: 24.,
                    color: constants::border_color(),
                    ..Default::default()
                },
            ));
        });
}

pub fn button_hover(
    mut buttons: Query<
        (&Interaction, &mut BackgroundColor),
        (With<Button>, Changed<Interaction>),
    >,
) {
    for (interaction, mut background) in buttons.iter_mut() {
        let mut color = constants::button_color();
        if *interaction != Interaction::None {
            color = constants::hover_color();
        }
        *background = BackgroundColor(color);
    }
}