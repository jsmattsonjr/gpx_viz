# GPX Terrain Visualizer

A Rust application that visualizes GPX tracks in 3D with terrain data from SRTM.

## Features (Milestone 1)

- Load GPX track files and visualize them in 3D
- Proper distance scaling (1 Bevy unit = 1 meter)
- Camera controls to explore the visualization
- Converts WGS84 coordinates to a local 3D coordinate system

## Building and Running

### Prerequisites

- Rust and Cargo (latest stable version)
- GDAL library installed on your system

#### Installing GDAL

**Ubuntu/Debian:**
```bash
sudo apt-get update
sudo apt-get install libgdal-dev
```

**macOS:**
```bash
brew install gdal
```

**Windows:**
Download and install from [GISInternals](https://www.gisinternals.com/release.php)

### Running the Application

1. Clone the repository
2. Place your GPX file in the project root directory as `track.gpx` or use the sample provided
3. Run the application:

```bash
cargo run
```

### Camera Controls

- **Left Mouse Button + Drag**: Rotate the camera
- **Right Mouse Button + Drag**: Pan the camera
- **Mouse Wheel**: Zoom in and out

## Project Structure

```
src/
├── main.rs                # Entry point, app setup
├── config.rs              # Configuration constants and settings
├── camera.rs              # Camera setup and controls
├── ui.rs                  # UI elements (future milestones)
├── geo/                   # Geospatial modules
│   ├── coordinates.rs     # WGS84 to Bevy coordinate conversion
│   ├── gpx.rs             # GPX file parsing
│   └── terrain.rs         # Terrain handling (future milestones)
├── srtm/                  # SRTM data modules (future milestones)
│   ├── cache.rs           # Local SRTM file caching
│   ├── downloader.rs      # NASA EarthData API client
│   └── parser.rs          # SRTM data extraction
└── rendering/             # Visualization modules
    ├── track.rs           # Track visualization
    ├── terrain_mesh.rs    # Terrain mesh creation (future milestones)
    └── skybox.rs          # Skybox implementation (future milestones)
```

## Future Milestones

- Enhanced track visualization with cylindrical meshes
- SRTM terrain data integration
- Automatic downloading of missing SRTM files
- Skybox and improved visuals
- Interactive UI elements

## License

MIT
