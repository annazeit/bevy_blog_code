use bevy::{
    prelude::*,
    color::palettes::css::*, 
    math::UVec2,
    render::camera::Viewport, 
    window::{PrimaryWindow, Window}
};

#[derive(Component)]
pub struct Grid {
    enabled: bool,
    size: i32,
    cell_size: f32,
}

#[derive(Component)]
struct FullScreen {
    enabled: bool,
}

#[derive(Component)]
struct FlyCamera {
    yaw: f32,   // rotation around Y axis in radians
    pitch: f32, // rotation around X axis in radians
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, grid)
        .add_systems(Update, fly_camera)
        .add_systems(Update, setup_viewpoints)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // UI node for camera background
    commands
        .spawn(Node {
            position_type: PositionType::Absolute,
            right: Val::Px(0.0),
            top: Val::Px(0.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        })
        .insert(BackgroundColor(BLACK.into()));

    // game view camera
    commands.spawn((
        Name::new("GameViewCamera"),
        Camera3d::default(),
        Transform::from_xyz(5.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        FullScreen { enabled: false },
    ));

    // main camera
    commands.spawn((
        Name::new("MainCamera"),
        Camera3d::default(),
        Transform::from_xyz(5.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        FlyCamera { yaw: 0.0, pitch: 0.0 },
    ));

    // light source
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));

    // grid entity
    commands.spawn(Grid {
        enabled: false,
        size: 10,
        cell_size: 1.0
    });
}

// Draw grid and axes, toggle with Space
fn grid(
    mut gizmos: Gizmos,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut grid: Single<&mut Grid>,
) {
        // toggle grid visibility
    if keyboard_input.just_pressed(KeyCode::Space) {
        grid.enabled = !grid.enabled;
    }

    if grid.enabled {
        // grid lines
        for i in -grid.size..=grid.size {
            let pos = i as f32 * grid.cell_size;
            gizmos.line(
                Vec3::new(pos, 0.0, -grid.size as f32),
                Vec3::new(pos, 0.0, grid.size as f32),
                GREY,
            );
            gizmos.line(
                Vec3::new(-grid.size as f32, 0.0, pos),
                Vec3::new(grid.size as f32, 0.0, pos),
                GREY,
            );
        }
        // axes
        gizmos.line(Vec3::new(-100.0, 0.01, 0.0), Vec3::new(100.0, 0.0, 0.0), RED);
        gizmos.line(Vec3::new(0.0, -100.0, 0.0), Vec3::new(0.0, 100.0, 0.0), GREEN);
        gizmos.line(Vec3::new(0.0, 0.01, -100.0), Vec3::new(0.0, 0.0, 100.0), BLUE);
        }
}

// WASD + QE movement and arrow keys for camera rotation
fn fly_camera(
    mut query: Query<(&mut Transform, &mut FlyCamera)>,
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let speed = 5.0;
    let rot_speed = 1.5; // radians/sec

    for (mut transform, mut camera) in &mut query {
        // spin on Y axis
        if keys.pressed(KeyCode::ArrowLeft) {
            camera.yaw += rot_speed * time.delta_secs();
        }
        if keys.pressed(KeyCode::ArrowRight) {
            camera.yaw -= rot_speed * time.delta_secs();
        }
        // pitch up/down
        if keys.pressed(KeyCode::ArrowUp) {
            camera.pitch += rot_speed * time.delta_secs();
        }
        if keys.pressed(KeyCode::ArrowDown) {
            camera.pitch -= rot_speed * time.delta_secs();
        }
        camera.pitch = camera.pitch.clamp(-1.54, 1.54); // clamp pitch to avoid flipping

        // apply yaw and pitch rotation to the camera
        transform.rotation =
            Quat::from_axis_angle(Vec3::Y, camera.yaw) *
            Quat::from_axis_angle(Vec3::X, camera.pitch);

        // movement (WASD for horizontal, QE for vertical)
        let mut direction = Vec3::ZERO;
        if keys.pressed(KeyCode::KeyW) {
            direction += *transform.forward() * time.delta_secs();
        }
        if keys.pressed(KeyCode::KeyS) {
            direction -= *transform.forward() * time.delta_secs();
        }
        if keys.pressed(KeyCode::KeyA) {
            direction -= *transform.right() * time.delta_secs();
        }
        if keys.pressed(KeyCode::KeyD) {
            direction += *transform.right() * time.delta_secs();
        }
        if keys.pressed(KeyCode::KeyQ) {
            direction += Vec3::Y * time.delta_secs();
        }
        if keys.pressed(KeyCode::KeyE) {
            direction -= Vec3::Y * time.delta_secs();
        }
        if direction.length_squared() > 0.0 {
            transform.translation += direction.normalize() * speed * time.delta_secs();
        }
        println!("Camera Position: {:?}", transform.translation);
        println!("Camera Rotation: {:?}", transform.rotation);
    }
}

// Toggle full screen for the game view camera with F11
fn full_screen_toggle(
    mut full_screen: Single<&mut FullScreen>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::F11) {
        full_screen.enabled = !full_screen.enabled;
        println!("Full Screen Mode: {}", full_screen.enabled);
    }
}

// Set up camera viewpoints and UI node size/position
fn setup_viewpoints(
    mut cameras: Query<(&Name, &mut Camera)>,
    mut ui_node: Single<&mut Node>,
    windows: Query<&Window, With<PrimaryWindow>>,
    full_screen: Single<&mut FullScreen>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    let window = windows.single();
    let width = window.resolution.physical_width();
    let height = window.resolution.physical_height();

    // Size of the small camera (e.g., 1/3 of window width and height)
    let small_width = width / 3;
    let small_height = height / 3;

    for (name, mut camera) in &mut cameras {
        match name.as_str() {
            "MainCamera" => {
                // Main camera covers the whole window
                camera.viewport = Some(Viewport {
                    physical_position: UVec2::new(0, 0),
                    physical_size: UVec2::new(width, height),
                    ..default()
                });
            }
            "GameViewCamera" => {
                match full_screen.enabled {
                    true => {
                        camera.viewport = Some(Viewport {
                            physical_size: UVec2::new(width, height),
                            ..default()
                        });
                        ui_node.width = Val::Px(width as f32) / 2.0;
                        ui_node.height = Val::Px(height as f32) / 2.0;
                    }
                    false => {
                        camera.viewport = Some(Viewport {
                            physical_position: UVec2::new(width - small_width, 0),
                            physical_size: UVec2::new(small_width, small_height),
                            ..default()
                        });
                        ui_node.width = Val::Px(small_width as f32) / 2.0;
                        ui_node.height = Val::Px(small_height as f32) / 2.0;
                    }
                }
            }
            _ => {}
        }
    }
    full_screen_toggle(full_screen, keyboard_input);
}
