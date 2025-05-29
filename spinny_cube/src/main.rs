use bevy::{prelude::*, color::palettes::css::*};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, hover_cube)
        .add_systems(Update, grid)
        .add_systems(Update,orbit_camera)
        .run();
}

#[derive(Component)]
struct OrbitCamera {
    angle: f32,
    radius: f32,
    speed: f32,
}

#[derive(Component)]
pub struct Grid {
    enabled: bool,
    size: i32,
    cell_size: f32,
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // camera with OrbitCamera component
        commands.spawn((
        Name::new("Camera"),
        Camera3d::default(),
        Transform::from_xyz(5.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
      
        OrbitCamera {
            angle: 0.0,
            radius: 5.0,
            speed: 0.5  ,
        } ));

    // cube
    commands.spawn((
        Name::new("Cube"),
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
        Transform::from_xyz(0.0, 0.0, 0.0) ));

    // light
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0) ));

    // grid
    commands.spawn(Grid {
        enabled: false,
        size: 10,
        cell_size:1.0 });
}

fn hover_cube(mut transform: Single<&mut Transform, With<Mesh3d>>, time: Res<Time>) {
    let hover_speed = 3.0;
    let hover_height = 0.3;
    let base_height = 1.0; // Offset to keep the cube above the grid

    transform.translation.y = base_height + hover_height * (hover_speed * time.elapsed_secs()).sin();
}


fn grid(mut gizmos: Gizmos, keyboard_input: Res<ButtonInput<KeyCode>>, mut grid_query: Query<&mut Grid>,) {
    for mut grid in &mut grid_query {

        if keyboard_input.just_pressed(KeyCode::Space) {
        grid.enabled = !grid.enabled; 
        }

        if grid.enabled {
            for i in -grid.size..=grid.size {
            let pos = i as f32  * grid.cell_size;

            gizmos.line(Vec3::new(pos, 0.0, -grid.size as f32),Vec3::new(pos, 0.0, grid.size as f32), GREY,);
            gizmos.line(Vec3::new(-grid.size as f32, 0.0, pos),Vec3::new(grid.size as f32, 0.0, pos), GREY,);
        }

        gizmos.line(Vec3::new(0.0, -100.0, 0.0), Vec3::new(0.0, 100.0, 0.0), RED);
        gizmos.line(Vec3::new(-100.0, 0.0, 0.0), Vec3::new(100.0, 0.0, 0.0), RED);
        gizmos.line(Vec3::new(0.0, 0.0, -100.0), Vec3::new(0.0, 0.0, 100.0), RED);
        }
    }
}

fn orbit_camera(mut query: Query<(&mut Transform, &mut OrbitCamera)>, time: Res<Time>) {
    for (mut transform, mut orbit) in &mut query {

        orbit.angle += orbit.speed * time.delta_secs(); // Update angle

        let x = orbit.radius * orbit.angle.cos(); 
        let z = orbit.radius * orbit.angle.sin();
        transform.translation = Vec3::new(x, 3.0, z);
        transform.look_at(Vec3::ZERO, Vec3::Y);
    }
}