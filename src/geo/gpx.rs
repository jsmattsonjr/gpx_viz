use bevy::prelude::*;
use gpx::{read, Gpx};
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use crate::config::{DEFAULT_GPX_FILE, TRACK_COLOR, TRACK_RADIUS_METERS};
use crate::geo::coordinates::GeoPoint;

// Track data resource
#[derive(Resource)]
pub struct GpxTrackData {
    pub points: Vec<GeoPoint>,
    pub origin: GeoPoint,
    pub name: String,
    pub loaded: bool,
}

impl Default for GpxTrackData {
    fn default() -> Self {
        Self {
            points: Vec::new(),
            origin: GeoPoint::new(0.0, 0.0, 0.0),
            name: String::new(),
            loaded: false,
        }
    }
}

// Command to load a GPX file
#[derive(Component)]
pub struct LoadGpx(pub String);

// System to load GPX file
pub fn load_gpx_file(
    mut commands: Commands,
    mut track_data: ResMut<GpxTrackData>,
    file_path: String,
) -> anyhow::Result<()> {
    info!("Loading GPX file: {}", file_path);

    // Open and parse the GPX file
    let file = File::open(&file_path)?;
    let reader = BufReader::new(file);
    let gpx: Gpx = read(reader)?;

    // Extract track points
    let mut points = Vec::new();

    for track in gpx.tracks {
        for segment in track.segments {
            for point in segment.points {
                let elevation = point.elevation.unwrap_or(0.0);
                points.push(GeoPoint::new(
                    point.point().x(),
                    point.point().y(),
                    elevation,
                ));
            }
        }
    }

    // If we have points, update the track data
    if !points.is_empty() {
        // Use the first point as the origin for local coordinates
        let origin = points[0];

        track_data.points = points;
        track_data.origin = origin;
        track_data.name = gpx.metadata.and_then(|m| m.name).unwrap_or_else(|| {
            Path::new(&file_path)
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("Unknown Track")
                .to_string()
        });
        track_data.loaded = true;

        info!("Loaded GPX track with {} points", track_data.points.len());
    } else {
        warn!("No track points found in GPX file");
    }

    Ok(())
}

// System to visualize loaded GPX tracks
pub fn visualize_loaded_gpx(
    mut commands: Commands,
    track_data: Res<GpxTrackData>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Only run once, when track data is loaded but not yet visualized
    if !track_data.loaded || track_data.points.is_empty() {
        return;
    }

    // For now, just use simple point visualization
    for (i, point) in track_data.points.iter().enumerate() {
        let position = point.to_bevy_position(Some(&track_data.origin));

        // Create a small sphere for each point
        commands.spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::UVSphere {
                radius: TRACK_RADIUS_METERS,
                sectors: 8,
                stacks: 8,
            })),
            material: materials
                .add(Color::srgb(TRACK_COLOR[0], TRACK_COLOR[1], TRACK_COLOR[2]).into()),
            transform: Transform::from_translation(position),
            ..default()
        });

        // For first and last points, add labels
        if i == 0 || i == track_data.points.len() - 1 {
            let label = if i == 0 { "Start" } else { "End" };

            // In a real implementation, we'd add a text element here
            // For now, just log the positions
            info!("{} point at: {:?}", label, position);
        }
    }

    info!(
        "Visualized GPX track with {} points",
        track_data.points.len()
    );
}

// Startup system to load the default GPX file
pub fn load_default_gpx(mut commands: Commands) {
    commands.spawn(LoadGpx(DEFAULT_GPX_FILE.to_string()));
}
