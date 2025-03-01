use crate::config::TRACK_RADIUS_METERS;
use crate::geo::coordinates::GeoPoint;
use bevy::prelude::*;
use bevy::render::mesh::shape;

// This will be expanded in later milestones to create cylindrical track segments
// For now, we'll just create placeholder functions

pub fn create_track_mesh(points: &[GeoPoint], reference: &GeoPoint, radius: f32) -> Mesh {
    // In a future milestone, we'll implement proper cylindrical segments
    // For now, return a simple line mesh
    let bevy_positions: Vec<Vec3> = points
        .iter()
        .map(|p| p.to_bevy_position(Some(reference)))
        .collect();

    create_line_mesh(&bevy_positions, radius)
}

// Temporary function to create a basic line mesh
fn create_line_mesh(points: &[Vec3], radius: f32) -> Mesh {
    // For now, just return a simple cylindrical mesh along the y-axis
    // This is a placeholder - will be replaced with proper track visualization
    Mesh::from(shape::Capsule {
        radius,
        depth: 10.0,
        ..default()
    })
}
