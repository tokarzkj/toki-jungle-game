use bevy::prelude::*;
use simple_jungle_game::characters::*;

fn main() {
    App::build()
        .add_startup_system(add_people.system())
        .run();
}
