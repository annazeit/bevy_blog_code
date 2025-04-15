use bevy::prelude::*; 

// Component to store the position of an entity
#[derive(Component)]
struct Position {
    x: f32,
    y: f32,
}

// Component to store the velocity of an entity
#[derive(Component)]
struct Velocity {
    dx: f32,
    dy: f32,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, spawn_player) // Run the spawn_player system once at startup
        .add_systems(Update, update_position) // Run the update_position system every frame
        .add_systems(Update, print_position) // Run the print_position system every frame
        .run(); // Start the application
}

// System that spawns a player entity with Position and Velocity components, using Commands
fn spawn_player(mut commands: Commands) {
    commands.spawn((
        Position { x: 0.0, y: 0.0 }, // Initial position
        Velocity { dx: 1.0, dy: 1.0 }, // Initial velocity
    ));
}

// System that updates the position of entities based on their velocity, using Query
fn update_position(mut query: Query<(&mut Position, &Velocity)>) {
    for (mut position, velocity) in query.iter_mut() {
        position.x += velocity.dx; // Update the x-coordinate
        position.y += velocity.dy; // Update the y-coordinate
    }
}

// System that prints the position of entities with a Position component, using Query
fn print_position(query: Query<(Entity, &Position)>) {
    for (entity, position) in query.iter() {
        println!("Player {:?} is at position: ({}, {})", entity, position.x, position.y);
        // Print the entity ID and its position
    }
}