use bevy::prelude::*;

pub struct Person;

pub struct Name(String);

pub fn add_people(mut commands : Commands) {
    commands
        .spawn((Person, Name("Kris".to_string())));
}