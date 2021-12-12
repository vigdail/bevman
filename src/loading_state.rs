use crate::GameState;
use bevy::prelude::*;
use bevy_asset_loader::{AssetCollection, AssetLoader};

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut AppBuilder) {
        AssetLoader::new(GameState::Loading, GameState::Gameplay)
            .with_collection::<TextureAssets>()
            .build(app);
    }
}

#[derive(AssetCollection)]
pub struct TextureAssets {
    #[asset(path = "textures/player.png")]
    pub player: Handle<Texture>,
    #[asset(path = "textures/wall.png")]
    pub wall: Handle<Texture>,
}
