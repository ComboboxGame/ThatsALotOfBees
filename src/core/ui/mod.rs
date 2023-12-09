use self::{
    button::{button_hover, EnableButtonWhenHaveMoney, MyButton},
    counter::{setup_bee_counters, update_counter},
    currency_display::{refresh_display, spawn_currency_display},
    menu::{click_button_system, menu_helper, menu_update, spawn_menu, Menu},
    moving_ui::move_ui,
};
use super::{
    get_building_position, AppState, Building, BuildingKind, BuildingMaterial, CurrencyStorage,
    HiveBuildings, MouseState, UniversalMaterial,
};
use bevy::prelude::*;

mod button;
mod constants;
mod counter;
mod currency_display;
mod menu;
mod moving_ui;

#[derive(Resource)]
pub struct UiSize {
    pub size: u32,
}

#[derive(Component)]
pub struct RelativePixelSized {
    pub width: u32,
    pub height: u32,
}

#[derive(Component, Clone)]
pub struct RelativePixelFont {
    pub size: u32,
}

#[derive(Component)]
pub struct RelativePixelPositioned {
    pub left: u32,
    pub top: u32,
}

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(UiMaterialPlugin::<UniversalMaterial>::default());
        app.insert_resource(UiSize { size: 3 });

        app.add_systems(
            PreUpdate,
            (
                pixel_sized_system,
                pixel_positioned_system,
                pixel_font_system,
            ),
        );

        app.add_systems(
            PreUpdate,
            (enable_button_when_have_money_system, menu_helper).run_if(in_state(AppState::InGame)),
        );

        app.add_systems(
            Update,
            (
                update_counter,
                highlight_hive,
                menu_update,
                move_ui,
                button_hover,
                refresh_display,
                click_button_system,
            )
                .run_if(in_state(AppState::InGame)),
        );

        app.add_systems(
            Last,
            setup_game_ui.run_if(state_changed::<AppState>().and_then(in_state(AppState::InGame))),
        );
    }
}

pub fn pixel_positioned_system(
    mut ui: ResMut<UiSize>,
    mut nodes_pos: Query<(
        &mut Style,
        &RelativePixelPositioned,
        Changed<RelativePixelPositioned>,
    )>,
    mut window: Query<&Window>,
) {
    if let Some(window) = window.iter().next() {
        let w = window.width();
        let h = window.height();

        let new_size = ((h / 300.0 + 0.5) as u32).max(2);
        if new_size != ui.size {
            ui.size = new_size;
        }

        for (mut style, pixel_pos, changed) in nodes_pos.iter_mut() {
            if changed || ui.is_changed() {
                style.position_type = PositionType::Absolute;
                style.left = Val::Px((ui.size * pixel_pos.left) as f32);
                style.top = Val::Px((ui.size * pixel_pos.top) as f32);
            }
        }
    }
}

pub fn pixel_font_system(
    mut ui: ResMut<UiSize>,
    mut nodes_pos: Query<(&mut Text, &RelativePixelFont, Changed<RelativePixelFont>)>,
    mut window: Query<&Window>,
) {
    if let Some(window) = window.iter().next() {
        let w = window.width();
        let h = window.height();

        let new_size = ((h / 300.0 + 0.5) as u32).max(2);
        if new_size != ui.size {
            ui.size = new_size;
        }

        for (mut text, pixel_pos, changed) in nodes_pos.iter_mut() {
            if changed || ui.is_changed() {
                for section in text.sections.iter_mut() {
                    section.style.font_size = (ui.size * pixel_pos.size) as f32;
                }
            }
        }
    }
}

pub fn pixel_sized_system(
    mut ui: ResMut<UiSize>,
    mut nodes: Query<(&mut Style, &RelativePixelSized, Changed<RelativePixelSized>)>,
    mut window: Query<&Window>,
) {
    if let Some(window) = window.iter().next() {
        let w = window.width();
        let h = window.height();

        let new_size = ((h / 300.0 + 0.5) as u32).max(2);
        if new_size != ui.size {
            ui.size = new_size;
        }

        for (mut style, pixel_size, changed) in nodes.iter_mut() {
            if changed || ui.is_changed() {
                style.width = Val::Px((ui.size * pixel_size.width) as f32);
                style.height = Val::Px((ui.size * pixel_size.height) as f32);
            }
        }
    }
}

pub fn enable_button_when_have_money_system(
    mut buttons: Query<(&mut MyButton, &EnableButtonWhenHaveMoney)>,
    storage: Res<CurrencyStorage>,
) {
    for (mut button, enable) in buttons.iter_mut() {
        let enabled = storage.check_can_spend(&enable.target);
        if enabled != button.enabled {
            button.enabled = enabled;
        }
    }
}

#[derive(Component)]
struct MainUiNode {}

fn setup_game_ui(
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
            spawn_currency_display(builder, &mut asset_server);
            spawn_menu(builder, &mut asset_server);
        });
}

fn highlight_hive(
    mut interaction_query: Query<(&Interaction, Changed<Interaction>, &MainUiNode)>,
    mut building_menu_query: Query<&mut Menu>,
    buildings_query: Query<(Entity, &Building, &Handle<BuildingMaterial>)>,
    mouse: Res<MouseState>,
    mut materials: ResMut<Assets<BuildingMaterial>>,
    mut mouse_pos_when_pressed: Local<Vec2>,
) {
    if interaction_query.is_empty() || building_menu_query.is_empty() {
        return;
    }

    let (interaction, changed, _) = interaction_query.single_mut();
    let mut building_menu = building_menu_query.single_mut();

    if let Some(mouse_position) = mouse.position {
        if *interaction == Interaction::Pressed && changed {
            // Start waiting for button release
            *mouse_pos_when_pressed = mouse_position;
        }

        if *interaction == Interaction::Hovered
            && changed
            && mouse_position.distance(*mouse_pos_when_pressed) < 8.0
        {
            // Button released and mouse did not move
            building_menu.focus_building = None;
            for (_, building, _) in buildings_query.iter() {
                let building_position = get_building_position(building.index);
                if mouse_position.distance(building_position) < 32.0
                    && building.kind != BuildingKind::Storage
                {
                    building_menu.focus_building = Some(building.index);
                    break;
                }
            }

            for (_, building, material) in buildings_query.iter() {
                let selected = if let Some(index) = building_menu.focus_building {
                    index == building.index
                } else {
                    false
                };

                if let Some(material) = materials.get_mut(material) {
                    if selected != ((material.state.x & 2) == 2)
                        && building.kind != BuildingKind::Storage
                    {
                        if selected {
                            material.state.x |= 2;
                        } else {
                            material.state.x &= !2;
                        }
                    }
                }
            }
        }
    }

    if let Some(mouse_position) = mouse.position {
        for (_, building, material) in buildings_query.iter() {
            let building_position = get_building_position(building.index);

            if let Some(material) = materials.get_mut(material) {
                if mouse_position.distance(building_position) < 32.0 {
                    material.state.x |= 1;
                } else {
                    material.state.x &= !1;
                }
            }
        }
    }
}
