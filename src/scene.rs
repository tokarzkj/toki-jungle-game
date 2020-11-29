use bevy::prelude::*;
use bevy_tiled_prototype::*;

pub fn load_tile_map (
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands
        .spawn(TiledMapComponents {
            map_asset: asset_server.load("levels/Level1.tmx"),
            center: TiledMapCenter(true),
            ..Default::default()
        })
        .with(TileMapName {
            name: "First Level".to_string()
        });
}

pub struct TileMapName {
    pub name: String,
}