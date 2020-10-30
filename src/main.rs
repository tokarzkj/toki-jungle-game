use bevy::prelude::*;
use simple_jungle_game::characters::*;
use simple_jungle_game::scene::*;

fn main() {
    App::build()
        .add_default_plugins()
        .add_startup_system(load_background.system())
        .add_startup_system(setup_character_sprite_sheet.system())
        .add_system(animate_sprite_system.system())
        .add_system(character_movement_system.system())
        .run();
}
