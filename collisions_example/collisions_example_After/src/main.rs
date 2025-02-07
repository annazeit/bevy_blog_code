use bevy::prelude::*; 
use bevy::color::palettes::basic::*;

#[derive(Component)]
struct Player {
    position: Vec2,
    color: Srgba,
    size_radius: f32,
} 
#[derive(Component)]
struct Obstacle {
    position: Vec2,
    color: Srgba,
    size_radius: f32,
} 

fn main() {     
    App::new()     
        .add_plugins(DefaultPlugins) 
        .add_systems(Startup, setup) // Startup runs once at the beginning
        .add_systems(Update, draw_player)  // Update runs every frame
        .run();// Runs the application
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default()); //Spawn a 2D camera entity

    commands.spawn(Player { //Spawn a Player entity
        position: Vec2::new(0.0, 0.0),
        color: RED,
        size_radius: 20.0,
    });

    commands.spawn(Obstacle { //Spawn an Obstacle entity
        position: Vec2::new(100.0, 100.0),
        color: BLUE,
        size_radius: 50.0,
    });
}

fn draw_player(
    mut gizmos: Gizmos,
    mut player_query: Query<&mut Player>, 
    obstacle_query: Query<&Obstacle>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    let step = 5.0;

    for mut player in &mut player_query {
        for obstacle in &obstacle_query {
            gizmos.circle_2d(player.position, player.size_radius, player.color); // Draw player
            gizmos.circle_2d(obstacle.position, obstacle.size_radius, obstacle.color); // Draw obstacle
            
            let mut new_position = player.position;

            if keyboard_input.pressed(KeyCode::ArrowLeft) {
                new_position.x -= step; // Move left
            } 
            if keyboard_input.pressed(KeyCode::ArrowRight) {
                new_position.x += step; // Move right
            }
            if keyboard_input.pressed(KeyCode::ArrowUp) {
                new_position.y += step; // Move up
            }
            if keyboard_input.pressed(KeyCode::ArrowDown) {
                new_position.y -= step; // Move down
            }

            if !check_collisions(new_position, &mut player, &obstacle) {
                player.position = new_position; // Update if no collision
            }
        }    
    }
}

fn check_collisions(
    new_position: Vec2, 
    player: &mut Player,
    obstacle: &Obstacle,
) -> bool {
    let distance = new_position.distance(obstacle.position);
    let sum_radius = player.size_radius + obstacle.size_radius;

    if distance < sum_radius { // if distance smaller than sum of radii
        player.color = GREEN;
        return true;
    }
    else {
        return false;
    }
}