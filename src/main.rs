use bevy::prelude::*;
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};
use geo::gpx::{load_gpx_file, LoadGpx};

mod camera;
mod config;
mod geo;
mod rendering;
mod srtm;
mod ui;

fn main() {
    env_logger::init();

    App::new()
        // Add default Bevy plugins
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "GPX Terrain Visualizer".to_string(),
                resolution: (1280.0, 720.0).into(),
                ..default()
            }),
            ..default()
        }))
        // Add camera control plugin
        .add_plugins(PanOrbitCameraPlugin)
        // Add resources
        .init_resource::<geo::gpx::GpxTrackData>()
        // Add our custom plugins/systems
        .add_systems(Startup, setup)
        .add_systems(Startup, camera::setup_camera)
        .add_systems(Startup, rendering::skybox::set_sky_clear_color)
        .add_systems(Startup, geo::gpx::load_default_gpx)
        // GPX file handling systems
        .add_systems(Update, handle_load_gpx)
        .add_systems(Update, geo::gpx::visualize_loaded_gpx)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Add a directional light for better visibility
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 10000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 10.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // Add a reference grid to help with scale (each grid cell = 10 meters)
    let grid_size = 1000.0; // 1km grid
    let grid_divisions = 100; // 10m divisions

    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Plane::from_size(grid_size).into()),
        material: materials.add(Color::srgb(0.3, 0.3, 0.3).into()),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    });

    // Add grid lines
    for i in 0..=grid_divisions {
        let position = -grid_size / 2.0 + i as f32 * (grid_size / grid_divisions as f32);

        // X-axis line
        commands.spawn(PbrBundle {
            mesh: meshes.add(shape::Box::new(grid_size, 0.1, 0.1).into()),
            material: materials.add(Color::srgb(0.5, 0.5, 0.5).into()),
            transform: Transform::from_xyz(0.0, 0.1, position),
            ..default()
        });

        // Z-axis line
        commands.spawn(PbrBundle {
            mesh: meshes.add(shape::Box::new(0.1, 0.1, grid_size).into()),
            material: materials.add(Color::srgb(0.5, 0.5, 0.5).into()),
            transform: Transform::from_xyz(position, 0.1, 0.0),
            ..default()
        });
    }

    info!("GPX Terrain Visualizer started");
}

// System to handle loading GPX files
fn handle_load_gpx(
    mut commands: Commands,
    mut track_data: ResMut<geo::gpx::GpxTrackData>,
    query: Query<(Entity, &LoadGpx)>,
) {
    for (entity, load_gpx) in query.iter() {
        if let Err(e) = load_gpx_file(commands.reborrow(), track_data.as_mut(), load_gpx.0.clone())
        {
            error!("Failed to load GPX file: {}", e);
        }

        // Remove the component after processing
        commands.entity(entity).despawn();
    }
}
