use bevy::asset::load_internal_asset;

mod builder;
mod camera;
mod input;
mod model;
mod navigation;
mod ui;

pub use builder::*;
pub use camera::*;
pub use input::*;
pub use model::*;
pub use navigation::*;
pub use ui::*;

pub struct CorePlugin;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy, Default, States)]
pub enum AppState {
    #[default]
    MainMenu,
    InGame,
}

pub const COMMON_SHADER_HANDLE: Handle<Shader> = Handle::weak_from_u128(1312296983110122547);
pub const UVDXDY_COMMON_SHADER_HANDLE: Handle<Shader> = Handle::weak_from_u128(1212291923110122247);
pub const BEE_COMMON_SHADER_HANDLE: Handle<Shader> = Handle::weak_from_u128(1412291983110122547);

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<AppState>();

        load_internal_asset!(
            app,
            UVDXDY_COMMON_SHADER_HANDLE,
            "../../assets/shaders/common/uvdxdy.wgsl",
            Shader::from_wgsl
        );
        load_internal_asset!(
            app,
            COMMON_SHADER_HANDLE,
            "../../assets/shaders/common/common.wgsl",
            Shader::from_wgsl
        );
        load_internal_asset!(
            app,
            BEE_COMMON_SHADER_HANDLE,
            "../../assets/shaders/common/bee.wgsl",
            Shader::from_wgsl
        );

        app.add_plugins(InputHelperPlugin);
        app.add_plugins(NavigationPlugin);
        app.add_plugins(UiPlugin);
        app.add_plugins(ModelPlugin);

        app.add_systems(
            Update,
            in_game_camera_system.run_if(in_state(AppState::InGame)),
        );
    }
}
