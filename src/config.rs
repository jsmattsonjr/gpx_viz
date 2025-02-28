// Physical constants
pub const EARTH_RADIUS_METERS: f64 = 6_371_000.0; // Average Earth radius in meters
pub const SRTM_RESOLUTION_ARCSECONDS: f64 = 1.0; // 1 arc-second resolution (~30m at equator)

// Scaling constants
pub const METERS_PER_BEVY_UNIT: f32 = 1.0; // 1 Bevy unit = 1 meter (1:1 scale)

// File paths
pub const SRTM_CACHE_DIR: &str = "srtm_cache";
pub const DEFAULT_GPX_FILE: &str = "assets/track.gpx";

// Visualization settings
pub const TRACK_COLOR: [f32; 3] = [1.0, 0.2, 0.2]; // Red
pub const TRACK_RADIUS_METERS: f32 = 1.0; // Thickness of track visualization
pub const TERRAIN_BASE_COLOR: [f32; 3] = [0.2, 0.5, 0.2]; // Green-ish
pub const SKY_COLOR: [f32; 3] = [0.5, 0.7, 1.0]; // Light blue
