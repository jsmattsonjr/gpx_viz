use bevy::prelude::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.1, 0.1, 0.1))) // Dark background
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Create a red sphere at the origin
    commands.spawn(PbrBundle {
        mesh: meshes.add(Sphere { radius: 1.0 }), // Unit radius sphere
        material: materials.add(StandardMaterial {
            base_color: Color::srgb(1.0, 0.0, 0.0), // Red color
            ..default()
        }),
        transform: Transform::from_xyz(0.0, 0.0, 0.0), // At origin
        ..default()
    });

    // Add a light so we can see the sphere
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

    // Add a camera looking at the sphere
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 2.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y), // Look at origin, with up direction
        ..default()
    });
}
