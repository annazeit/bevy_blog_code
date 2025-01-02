use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn draw_circle(mut gizmos: Gizmos,) {
    gizmos.circle_2d(Vec2::new(0.0, 0.0), 100.0, Color::RED);
}