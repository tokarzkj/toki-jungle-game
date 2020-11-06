use bevy_render::camera::Camera;
use bevy::{prelude::*};
use bevy_tiled::*;

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
) {
    commands
        .spawn(TiledMapComponents {
            map_asset: asset_server.load("assets/levels/level1-small.tmx").unwrap(),
            center: TiledMapCenter(true),
            origin: Transform::from_non_uniform_scale(Vec3::new(10.0, 10.0, 10.0)),
            ..Default::default()
        });
}

pub fn camera_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Camera, &mut Transform)>,
) {
    for (_, mut transform) in &mut query.iter() {
        let scale = transform.value().x_axis().x();

        if keyboard_input.pressed(KeyCode::Z) {
            transform.set_scale(scale + 0.1);
        }

        if keyboard_input.pressed(KeyCode::X)  && scale > 1.1 {
            transform.set_scale(scale - 0.1);
        }

        let translation = transform.translation();

        transform.set_translation(translation);
    }
}