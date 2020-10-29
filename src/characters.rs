use bevy::prelude::*;

pub fn setup_character_sprite_sheet(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("assets/Character/sprites/character-idle-spritesheet.png").unwrap();
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(24.0, 24.0), 4, 3);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    
    commands
        .spawn(Camera2dComponents::default())
        .spawn(SpriteSheetComponents {
            texture_atlas: texture_atlas_handle,
            transform: Transform::from_scale(6.0),
            ..Default::default()
        })
        .with(Character { speed: 500.0 })
        .with(Timer::from_seconds(0.1, true));
}

pub fn animate_sprite_system(
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(&mut Timer, &mut TextureAtlasSprite, &Handle<TextureAtlas>)>,
) {
    for (timer, mut sprite, texture_atlas_handle) in &mut query.iter() {
        if timer.finished {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            sprite.index = ((sprite.index as usize + 1) % texture_atlas.textures.len()) as u32;
        }
    }
}

pub struct Character {
    speed : f32
}

pub fn character_movement_system(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Character, &mut Transform)>,
) {
    for (character, mut transform) in &mut query.iter() {
        let mut x_direction = 0.0;
        if keyboard_input.pressed(KeyCode::Left) {
            x_direction -= 1.0;
        }

        if keyboard_input.pressed(KeyCode::Right) {
            x_direction += 1.0;
        }

        let translation = &mut transform.translation_mut();
        *translation.x_mut() += time.delta_seconds * x_direction * character.speed;
        *translation.x_mut() = translation.x().min(450.0).max(-450.0);

        let mut y_direction = 0.0;
        if keyboard_input.pressed(KeyCode::Up) {
            y_direction += 1.0;
        }

        if keyboard_input.pressed(KeyCode::Down) {
            y_direction -= 1.0;
        }

        let translation = &mut transform.translation_mut();
        *translation.y_mut() += time.delta_seconds * y_direction * character.speed;
        *translation.y_mut() = translation.y().min(300.0).max(-300.0);
    }
}