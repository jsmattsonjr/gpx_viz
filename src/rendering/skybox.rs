use crate::config::SKY_COLOR;
use bevy::prelude::*;

// This is a placeholder for future implementation
// Will be expanded in later milestones

pub fn setup_skybox(mut commands: Commands) {
    // For now, just use a clear color for the sky
    // Will be replaced with proper skybox implementation
}

// Set sky color
pub fn set_sky_clear_color(mut clear_color: ResMut<ClearColor>) {
    *clear_color = ClearColor(Color::rgb(SKY_COLOR[0], SKY_COLOR[1], SKY_COLOR[2]));
}
