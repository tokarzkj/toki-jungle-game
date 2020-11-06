use bevy::prelude::*;
use simple_jungle_game::characters::*;
use simple_jungle_game::scene::*;

fn main() {
    App::build()
        .add_resource(WindowDescriptor {
            title: "Toki Jungle Game".to_string(),
            resizable: false,
            ..Default::default()
        })
        .add_default_plugins()
        .add_plugin(bevy_tiled::TiledMapPlugin)
        //.add_startup_system(load_background.system())
        .add_startup_system(setup_character_sprite_sheet.system())
        .add_startup_system(load_tile_map.system())
        .add_system(animate_sprite_system.system())
        .add_system(character_movement_system.system())
        .run();
}
