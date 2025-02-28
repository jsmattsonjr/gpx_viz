use bevy::prelude::*;
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};

pub fn setup_camera(mut commands: Commands) {
    // Position the camera to view the grid from a reasonable distance
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 400.0, 1000.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        PanOrbitCamera {
            focus: Vec3::ZERO,
            radius: 1000.0,                      // Start 1km away from origin
            alpha: -std::f32::consts::FRAC_PI_4, // Camera angle (pitch)
            beta: std::f32::consts::FRAC_PI_4,   // Camera angle (yaw)
            zoom_sensitivity: 0.1,
            ..default()
        },
    ));
}
