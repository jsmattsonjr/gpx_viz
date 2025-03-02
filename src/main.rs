mod camera;

use bevy::prelude::*;
use camera::{OrbitCamera, OrbitCameraPlugin};

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.1, 0.1, 0.1))) // Dark background
        .add_plugins(DefaultPlugins)
        .add_plugins(OrbitCameraPlugin)
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
            base_color: Color::srgb(0.95, 0.04, 0.04), // More saturated true red
            perceptual_roughness: 0.5,                 // Slightly higher roughness
            metallic: 0.0,                             // Non-metallic
            reflectance: 0.2, // Lower reflectance for more color saturation
            ..default()
        }),
        transform: Transform::from_xyz(0.0, 0.0, 0.0), // At origin
        ..default()
    });

    // Simulate outdoor sunlight with a bright directional light
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 50000.0, // Bright sunlight
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 10.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y), // Sun coming from above
        ..default()
    });

    // Add a softer fill light to simulate sky light
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 10000.0,
            shadows_enabled: false,
            color: Color::srgb(0.8, 0.8, 1.0), // Slightly blue for sky
            ..default()
        },
        transform: Transform::from_xyz(-5.0, 3.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // Ambient light for outdoors
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.5, // Brighter ambient for outdoor setting
    });

    // Add a camera looking at the sphere with orbit controls
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 2.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y), // Look at origin, with up direction
            ..default()
        },
        OrbitCamera {
            focus: Vec3::ZERO,
            radius: 5.0,
            sensitivity: 0.01,
            zoom_sensitivity: 0.25,
        },
    ));
}
