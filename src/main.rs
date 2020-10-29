use bevy::prelude::*;
use simple_jungle_game::characters::*;

fn main() {
    App::build()
        .add_default_plugins()
        .add_startup_system(setup_character_sprite_sheet.system())
        .add_system(animate_sprite_system.system())
        .add_system(character_movement_system.system())
        .run();
}
