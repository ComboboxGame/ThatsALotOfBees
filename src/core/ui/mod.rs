use self::{
    button::button_hover,
    counter::{setup_bee_counters, update_counter},
    menu::{menu_update, spawn_menu, Menu, order_button_system},
    moving_ui::move_ui,
};
use super::{get_building_position, Building, MouseState, UniversalMaterial};
use bevy::prelude::*;

mod button;
mod constants;
mod counter;
mod menu;
mod moving_ui;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(UiMaterialPlugin::<UniversalMaterial>::default());

        app.add_systems(Startup, setup_ui);
        app.add_systems(Update, update_counter);
        app.add_systems(Update, highlight_hive);
        app.add_systems(Update, menu_update);
        app.add_systems(Update, move_ui);
        app.add_systems(Update, button_hover);

        app.add_systems(Update, order_button_system);
    }
}

#[derive(Component)]
struct MainUiNode {}

fn setup_ui(
    mut commands: Commands,
    materials: ResMut<Assets<UniversalMaterial>>,
    mut asset_server: ResMut<AssetServer>,
) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    ..Default::default()
                },
                ..Default::default()
            },
            MainUiNode {},
            Interaction::None,
        ))
        .with_children(|builder| {
            setup_bee_counters(builder, materials, &mut asset_server);
            spawn_menu(builder, &mut asset_server);
        });
}

fn highlight_hive(
    mut interaction_query: Query<(&Interaction, Changed<Interaction>, &MainUiNode)>,
    mut building_menu_query: Query<&mut Menu>,
    buildings_query: Query<(Entity, &Building, &Handle<ColorMaterial>)>,
    mouse: Res<MouseState>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut mouse_pos_when_pressed: Local<Vec2>,
) {
    let (interaction, changed, _) = interaction_query.single_mut();
    let mut building_menu = building_menu_query.single_mut();
    
    if let Some(mouse_position) = mouse.position {
        if *interaction == Interaction::Pressed && changed {
            // Start waiting for button release
            *mouse_pos_when_pressed = mouse_position;
        }

        if *interaction == Interaction::Hovered && changed && mouse_position.distance(*mouse_pos_when_pressed) < 8.0 {
            // Button released and mouse did not move
            building_menu.focus_building = None;
            for (_, building, _) in buildings_query.iter() {
                let building_position = get_building_position(building.index);
                if mouse_position.distance(building_position) < 32.0 {
                    building_menu.focus_building = Some(building.index);
                    break;
                }
            }
        }
    }

    if let Some(mouse_position) = mouse.position {
        for (_, building, material) in buildings_query.iter() {
            let building_position = get_building_position(building.index);

            let color = if mouse_position.distance(building_position) < 32.0 {
                Color::rgb_linear(1.4, 1.4, 1.4)
            } else {
                Color::rgb_linear(1.0, 1.0, 1.0)
            };

            if let Some(material) = materials.get_mut(material) {
                if material.color != color {
                    material.color = color;
                }
            }
        }
    }
}
