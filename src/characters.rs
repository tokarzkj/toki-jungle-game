use bevy_tiled_prototype::Map;
use bevy::prelude::*;

pub fn setup_character_sprite_sheet(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window: Res<WindowDescriptor>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("Character/sprites/character-idle-spritesheet.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(24.0, 24.0), 4, 3);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    
    let width = get_character_x_placement(window.width);
    let height = get_character_y_placement(window.height);

    let camera_transform = Transform::from_translation(Vec3::new(-8.0, 8.0, 1.0));
    let spritesheet_transform = Transform::from_translation(Vec3::new(-width, -height, 1.0));
    commands
        .spawn(Camera2dComponents {
            transform: camera_transform,
            ..Default::default()
        })
        .spawn(SpriteSheetComponents {
            texture_atlas: texture_atlas_handle,
            transform: spritesheet_transform,
            ..Default::default()
        })
        .with(Character { speed: 250.0, jump_velocity: 0.0, is_jumping: false })
        .with(Timer::from_seconds(0.1, true));
}

pub fn animate_sprite_system(
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(&mut Timer, &mut TextureAtlasSprite, &Handle<TextureAtlas>)>,
) {
    for (timer, mut sprite, texture_atlas_handle) in &mut query.iter_mut() {
        if timer.finished {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            sprite.index = ((sprite.index as usize + 1) % texture_atlas.textures.len()) as u32;
        }
    }
}

pub struct Character {
    pub speed : f32,
    jump_velocity: f32,
    is_jumping : bool,
}


pub fn character_movement_system(
    window: Res<WindowDescriptor>,
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Character, &mut Transform)>,
) {
    let width = get_character_x_placement(window.width);
    let height = get_character_y_placement(window.height);

    for (mut character, mut transform) in &mut query.iter_mut() {
        let mut x_direction = 0.0;
        if keyboard_input.pressed(KeyCode::Left) {
            x_direction -= 1.0;
        }

        if keyboard_input.pressed(KeyCode::Right) {
            x_direction += 1.0;
        }

        let translation = &mut transform.translation;
        *translation.x_mut() += time.delta_seconds * x_direction * character.speed;
        *translation.x_mut() = translation.x().min(width).max(-width);

        if keyboard_input.pressed(KeyCode::Up) && !character.is_jumping {
            character.jump_velocity = 4.0;
            character.is_jumping = true;
        }

        if !keyboard_input.pressed(KeyCode::Up) && character.is_jumping {
            character.jump_velocity -= 0.1;
        }

        if translation.y() == -height {
            character.is_jumping = false;
        }
        
        let translation = &mut transform.translation;
        *translation.y_mut() += time.delta_seconds * character.jump_velocity * character.speed;
        *translation.y_mut() = translation.y().min(height).max(-height);
    }
}

fn get_character_x_placement(window_width : u32) -> f32 {
    window_width as f32 / 2.0 - 15.0
}

fn get_character_y_placement(window_height: u32) -> f32 {
    window_height as f32 / 2.0 - 50.0
}

pub fn map_collision_detection(
    asset_server: Res<Assets<Map>>,
    mut character_query: Query<(&mut Character, &mut Transform)>,
    mut chunk_component: Query<&Handle<Map>>
) {
     for (mut character, transform) in &mut character_query.iter_mut() {
            // for map_handle in &mut chunk_component.iter() {
            //     let map = asset_server.get(map_handle);
            //     let chunks = &map.layers[0].tileset_layers[0].chunks;
            //     println!("Count of chunks {}", chunks.len());
            //     for chunk_row in chunks {
            //         for chunk in chunk_row {
            //             let x = chunk.position[0] as usize;
            //             let y = chunk.position[1] as usize;
            //             let tile = &chunk.tiles[x][y];
            //             println!("this chunk is located at {},{}", tile.uv.x(), tile.uv.y());
            //             let translation = transform.translation;
            //             if (translation.x() <= chunk.position[0] + 8.0 && translation.x() >= chunk.position[0] - 8.0) &&
            //                 translation.y() == chunk.position[1] + 8.0  {
            //                 character.is_jumping = false;
            //                 println!("We aren't jumping anymore");
            //             }
            //         }
            //     }
            //     println!("The id value is");//, tileMapName.layer_id);
                
            // }   
     }
}