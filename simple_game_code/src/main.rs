use bevy::prelude::*; // includes commonly used types, traits, and functions from the Bevy game engine.
use bevy::color::palettes::basic::*;
//use bevy::input::ButtonInput;

#[derive(Component)] // Marks the Player struct as a component that can be attached to entities in Bevy's Entity-Component-System.
struct Player {
    position: Vec2,
    direction_angle: f32,
    speed: f32,
    color: Srgba,
} 
fn main() {     
    App::new() // Creates a new Bevy application.
    
        .add_plugins(DefaultPlugins) 

        // Adds the setup system to the Startup stage, which runs once at the beginning.
        .add_systems(Startup, setup) 

        //Adds the player_update system to the Update stage, which runs every frame.
        .add_systems(Update, draw_player) 

         // Runs the application.
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default()); //Spawns a 2D camera entity.

    commands.spawn(Player { //Spawns a Player entity with these parameters.
        position: Vec2::new(0.0, 0.0),
        direction_angle: 0.0,
        speed: 3.0,
        color: RED,
    });
}

fn draw_player(
    mut gizmos: Gizmos,
    mut player_query: Query<&mut Player>, 
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    let size_radius = 20.0;
    for mut player in &mut player_query {
        gizmos.circle_2d(player.position, size_radius, player.color); // Draws a circle at the player's position.
    
        if keyboard_input.pressed(KeyCode::ArrowLeft) {
            player.direction_angle -= 0.1; // Rotates the player to the left.
        } 
        if keyboard_input.pressed(KeyCode::ArrowRight) {
            player.direction_angle += 0.1; // Rotates the player to the right.
        }

        // Calculate the movement vector based on the player's direction and speed.
        let x = f32::sin(player.direction_angle);
        let y = f32::cos(player.direction_angle);
        let movement_vector = Vec2::new(x, y) * player.speed;

        if keyboard_input.pressed(KeyCode::ArrowUp) {
            player.position += movement_vector; // Moves the player forward.
        }
        if keyboard_input.pressed(KeyCode::ArrowDown) {
            player.position -= movement_vector; // Moves the player backward.
        }
    }    
}