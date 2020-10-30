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
            transform: Transform::from_translation_rotation_scale(Vec3::new(-500.0, -400.0, 1.5), Default::default(), 6.0),
            ..Default::default()
        })
        .with(Character { speed: 500.0, jump_velocity: 0.0, is_jumping: false })
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
    speed : f32,
    jump_velocity: f32,
    is_jumping : bool,
}


pub fn character_movement_system(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Character, &mut Transform)>,
) {
    for (mut character, mut transform) in &mut query.iter() {
        let mut x_direction = 0.0;
        if keyboard_input.pressed(KeyCode::Left) {
            x_direction -= 1.0;
        }

        if keyboard_input.pressed(KeyCode::Right) {
            x_direction += 1.0;
        }

        let translation = &mut transform.translation_mut();
        *translation.x_mut() += time.delta_seconds * x_direction * character.speed;
        *translation.x_mut() = translation.x().min(600.0).max(-600.0);

        if keyboard_input.pressed(KeyCode::Up) && !character.is_jumping {
            character.jump_velocity = 2.0;
            character.is_jumping = true;
        }

        if !keyboard_input.pressed(KeyCode::Up) && character.is_jumping {
            character.jump_velocity -= 0.1;
        }

        if translation.y() == -300.0 {
            character.is_jumping = false;
        }

        let translation = &mut transform.translation_mut();
        *translation.y_mut() += time.delta_seconds * character.jump_velocity * character.speed;
        *translation.y_mut() = translation.y().min(300.0).max(-300.0);
    }
}