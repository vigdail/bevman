use bevy::{input::system::exit_on_esc_system, prelude::*};

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
        .add_system(exit_on_esc_system.system())
        .add_plugins(DefaultPlugins)
        .add_startup_system((|| info!("BevMan")).system());
    }
}
