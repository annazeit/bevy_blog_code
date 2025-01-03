use bevy::prelude::*;
use bevy::color::palettes::basic::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, draw_player)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn draw_player(mut gizmos: Gizmos,) {
    let size_radius = 20.0;
    gizmos.circle_2d(Vec2::new(0.0, 0.0), size_radius, RED);
}