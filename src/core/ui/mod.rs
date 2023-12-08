use self::{
    counter::{setup_bee_counters, update_counter},
    menu::{menu_update, spawn_menu, Menu}, moving_ui::move_ui, button::button_hover,
};
use bevy::prelude::*;
use super::{get_building_position, Building, MouseState, UniversalMaterial};

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
) {
    let (interaction, changed, _) = interaction_query.single_mut();
    let mut building_menu = building_menu_query.single_mut();
    if *interaction == Interaction::Pressed && changed {
        building_menu.focus_building = None;
        if let Some(mouse_position) = mouse.position {
            for (_, building, _) in buildings_query.iter() {
                let building_position = get_building_position(building.index);
                if mouse_position.distance(building_position) < 32.0 {
                    building_menu.focus_building = Some(building.index);
                    break;
                }
            }
        }
    } else {
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
}
