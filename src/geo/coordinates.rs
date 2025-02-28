use crate::config::{EARTH_RADIUS_METERS, METERS_PER_BEVY_UNIT};
use bevy::prelude::*;
use gdal::spatial_ref::{CoordTransform, SpatialRef};
use std::f64::consts::PI;

#[derive(Debug, Clone, Copy)]
pub struct GeoPoint {
    pub longitude: f64, // WGS84 longitude in degrees
    pub latitude: f64,  // WGS84 latitude in degrees
    pub elevation: f64, // Elevation in meters
}

impl GeoPoint {
    pub fn new(longitude: f64, latitude: f64, elevation: f64) -> Self {
        Self {
            longitude,
            latitude,
            elevation,
        }
    }

    // Convert to Bevy Vec3 relative to a reference point (origin)
    pub fn to_bevy_position(&self, reference: Option<&GeoPoint>) -> Vec3 {
        let ref_point = reference.unwrap_or(&GeoPoint::new(0.0, 0.0, 0.0));

        // Convert to local ENU (East-North-Up) coordinates using GDAL
        let (east, north, up) = Self::wgs84_to_enu(
            self.longitude,
            self.latitude,
            self.elevation,
            ref_point.longitude,
            ref_point.latitude,
            ref_point.elevation,
        );

        // Scale according to our config and convert to Bevy coordinates
        // Bevy uses: +X = right (east), +Y = up, +Z = back (south)
        Vec3::new(
            (east / METERS_PER_BEVY_UNIT as f64) as f32,
            (up / METERS_PER_BEVY_UNIT as f64) as f32,
            (-north / METERS_PER_BEVY_UNIT as f64) as f32, // Note the negation for north -> south
        )
    }

    // Convert WGS84 coordinates to ENU (East-North-Up) relative to a reference point
    // Returns (east_meters, north_meters, up_meters)
    fn wgs84_to_enu(
        lon: f64,
        lat: f64,
        alt: f64,
        ref_lon: f64,
        ref_lat: f64,
        ref_alt: f64,
    ) -> (f64, f64, f64) {
        // Create WGS84 spatial reference
        let wgs84 = SpatialRef::from_epsg(4326).unwrap();

        // Create ECEF (Earth-Centered, Earth-Fixed) spatial reference
        let ecef = SpatialRef::from_proj4("+proj=geocent +datum=WGS84 +units=m +no_defs").unwrap();

        // Create ENU (East-North-Up) spatial reference centered on reference point
        let enu = SpatialRef::from_proj4(&format!(
            "+proj=tmerc +datum=WGS84 +units=m +k=1 +lon_0={} +lat_0={} +x_0=0 +y_0=0 +no_defs",
            ref_lon, ref_lat
        ))
        .unwrap();

        // Transform from WGS84 to ECEF
        let mut wgs84_to_ecef = CoordTransform::new(&wgs84, &ecef).unwrap();
        let mut x = lon;
        let mut y = lat;
        let mut z = alt;
        wgs84_to_ecef
            .transform_coords(&mut x, &mut y, &mut z)
            .unwrap();

        // Transform from ECEF to ENU
        let mut ecef_to_enu = CoordTransform::new(&ecef, &enu).unwrap();
        let mut east = x;
        let mut north = y;
        let mut up = z;
        ecef_to_enu
            .transform_coords(&mut east, &mut north, &mut up)
            .unwrap();

        (east, north, up)
    }

    // Alternative implementation using simple great-circle calculations
    // Less accurate but doesn't require GDAL if we encounter issues
    #[allow(dead_code)]
    fn wgs84_to_enu_simple(
        lon: f64,
        lat: f64,
        alt: f64,
        ref_lon: f64,
        ref_lat: f64,
        ref_alt: f64,
    ) -> (f64, f64, f64) {
        let lat_rad = lat * PI / 180.0;
        let ref_lat_rad = ref_lat * PI / 180.0;
        let delta_lon_rad = (lon - ref_lon) * PI / 180.0;

        // East-West distance
        let east = EARTH_RADIUS_METERS * delta_lon_rad * ref_lat_rad.cos();

        // North-South distance
        let north = EARTH_RADIUS_METERS * (lat_rad - ref_lat_rad);

        // Up distance is just elevation difference
        let up = alt - ref_alt;

        (east, north, up)
    }
}
