use bevy::prelude::*; 
use bevy::color::palettes::basic::*;

#[derive(Component)]
struct Player {
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
}

fn draw_player(
    mut gizmos: Gizmos,
    mut player_query: Query<&mut Player>, 
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    let step = 5.0;

    for mut player in &mut player_query {
        gizmos.circle_2d(player.position, player.size_radius, player.color); // Draw player
        
        if keyboard_input.pressed(KeyCode::ArrowLeft) {
            player.position.x -= step; // Move left
        } 
        if keyboard_input.pressed(KeyCode::ArrowRight) {
            player.position.x += step; // Move right
        }
        if keyboard_input.pressed(KeyCode::ArrowUp) {
            player.position.y += step; // Move up
        }
        if keyboard_input.pressed(KeyCode::ArrowDown) {
            player.position.y -= step; // Move down
        }
    }    
}