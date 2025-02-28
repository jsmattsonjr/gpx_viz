use crate::geo::coordinates::GeoPoint;
use crate::geo::terrain::TerrainBounds;

// Will be expanded in later milestones
// For now, just basic structure

pub fn get_srtm_file_name(longitude: f64, latitude: f64) -> String {
    // SRTM files are named by the southwest corner of the tile
    // e.g., N47E008.hgt for the tile covering 47-48°N and 8-9°E

    let lat_prefix = if latitude >= 0.0 { 'N' } else { 'S' };
    let lon_prefix = if longitude >= 0.0 { 'E' } else { 'W' };

    let lat_value = latitude.abs().floor() as i32;
    let lon_value = longitude.abs().floor() as i32;

    format!(
        "{}{:02}{}{:03}.hgt",
        lat_prefix, lat_value, lon_prefix, lon_value
    )
}

pub fn get_required_srtm_files(bounds: &TerrainBounds) -> Vec<String> {
    // This will be expanded in later milestones
    // For now, just a placeholder that returns a single file

    let min_lat = bounds.min_lat.floor() as i32;
    let min_lon = bounds.min_lon.floor() as i32;
    let max_lat = bounds.max_lat.ceil() as i32;
    let max_lon = bounds.max_lon.ceil() as i32;

    let mut files = Vec::new();
    for lat in min_lat..max_lat {
        for lon in min_lon..max_lon {
            let file_name = get_srtm_file_name(lon as f64, lat as f64);
            files.push(file_name);
        }
    }

    files
}

// Will be expanded in later milestones to handle downloading SRTM files
pub async fn download_srtm_file(_file_name: &str) -> anyhow::Result<()> {
    // Placeholder for future implementation
    Ok(())
}
