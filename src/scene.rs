use bevy::prelude::*;
use tiled::*;
use std::fs::File;
use std::path::Path;
use std::io::BufReader;

pub fn load_background(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands
        .spawn(SpriteComponents {
            transform: Transform::from_translation_rotation_scale(Vec3::new(1.0, 1.0, 0.5), Default::default(), 3.5),
            material: materials.add(asset_server.load("assets/parallax background/plx-5.png").unwrap().into()),
            ..Default::default()
        });
}

pub fn load_tile_map (
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>
) {
    let file = File::open(&Path::new("assets/levels/level1.tmx")).unwrap();
    let reader = BufReader::new(file);
    let map = parse(reader).unwrap();
    println!("Count {}", map.layers.len());

    let layer = &map.layers[0];
    let tiles: &LayerData = &layer.tiles;

    let layer_tiles = match tiles {
        tiled::LayerData::Finite(layer_tiles) => layer_tiles,
        tiled::LayerData::Infinite(_) => panic!("error"),
    };

    for row in layer_tiles.iter() {
        for tile in row.iter() {
            if tile.gid == 0 {
                continue;
            }

            let tileset = match map.get_tileset_by_gid(tile.gid) {
                Some(t) => t,
                None => panic!("No tileset") 
            };

            println!("this is the tileset {}", tileset.name);
        }
    }
}