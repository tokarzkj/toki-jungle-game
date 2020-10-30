use bevy::prelude::*;

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