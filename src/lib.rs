mod camera;
mod level;
mod loading_state;
mod player;

use bevy::{input::system::exit_on_esc_system, log::LogSettings, prelude::*};
use camera::CameraPlugin;
use level::LevelPlugin;
use loading_state::LoadingPlugin;
use player::PlayerPlugin;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum GameState {
    Loading,
    Gameplay,
}

pub struct BevManPlugin;

impl Plugin for BevManPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(WindowDescriptor {
            width: 800.0,
            height: 600.0,
            title: "BevMan".to_owned(),
            resizable: false,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(LogSettings {
            level: bevy::log::Level::INFO,
            ..Default::default()
        })
        .add_state(GameState::Loading)
        .add_system(exit_on_esc_system.system())
        .add_plugins(DefaultPlugins)
        .add_plugin(LoadingPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(LevelPlugin)
        .add_plugin(CameraPlugin)
        .add_startup_system(startup_system.system());
    }

    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }
}

fn startup_system(mut cmd: Commands) {
    cmd.spawn_bundle(OrthographicCameraBundle::new_2d());
}
