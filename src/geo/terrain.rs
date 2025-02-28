use crate::geo::coordinates::GeoPoint;
use bevy::prelude::*;
use gdal::Dataset;

// This is a placeholder that will be expanded in later milestones
// when we implement SRTM terrain data handling

#[derive(Resource)]
pub struct TerrainData {
    pub loaded: bool,
    pub bounds: TerrainBounds,
}

pub struct TerrainBounds {
    pub min_lon: f64,
    pub max_lon: f64,
    pub min_lat: f64,
    pub max_lat: f64,
    pub min_elevation: f64,
    pub max_elevation: f64,
}

impl Default for TerrainData {
    fn default() -> Self {
        Self {
            loaded: false,
            bounds: TerrainBounds {
                min_lon: 0.0,
                max_lon: 0.0,
                min_lat: 0.0,
                max_lat: 0.0,
                min_elevation: 0.0,
                max_elevation: 0.0,
            },
        }
    }
}

// Will be expanded in future milestones
pub fn calculate_terrain_bounds_for_track(track_points: &[GeoPoint]) -> TerrainBounds {
    let mut bounds = TerrainBounds {
        min_lon: f64::MAX,
        max_lon: f64::MIN,
        min_lat: f64::MAX,
        max_lat: f64::MIN,
        min_elevation: f64::MAX,
        max_elevation: f64::MIN,
    };

    for point in track_points {
        bounds.min_lon = bounds.min_lon.min(point.longitude);
        bounds.max_lon = bounds.max_lon.max(point.longitude);
        bounds.min_lat = bounds.min_lat.min(point.latitude);
        bounds.max_lat = bounds.max_lat.max(point.latitude);
        bounds.min_elevation = bounds.min_elevation.min(point.elevation);
        bounds.max_elevation = bounds.max_elevation.max(point.elevation);
    }

    // Add a buffer around the track (about 1km at equator)
    let buffer_degrees = 0.01;
    bounds.min_lon -= buffer_degrees;
    bounds.max_lon += buffer_degrees;
    bounds.min_lat -= buffer_degrees;
    bounds.max_lat += buffer_degrees;

    bounds
}
