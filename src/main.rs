use bevy::prelude::*;

pub fn dummy() {
    println!("Hello from Bevy!");
}

fn main() {
    App::new()
        .add_systems(Startup, setup)
        .add_systems(Update, dummy)
        .add_systems(Update, print_names)
        .run();
}

pub fn setup(mut commands: Commands) {
    commands.spawn(Person {
        name: "Alex".to_string(),
    });
    commands.spawn(Person {
        name: "Pedro".to_string(),
    });
    commands.spawn(Person {
        name: "Fabi".to_string(),
    });
    commands.spawn(Person {
        name: "Alex".to_string(),
    });
}

pub fn print_names(person_query: Query<&Person>) {
    for person in person_query.iter() {
        println!("Name: {}", person.name);
    }
}

#[derive(Component)]
pub struct Person {
    name: String,
}
