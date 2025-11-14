//! Level of Detail (LOD) system for dynamic terrain resolution
//!
//! This module provides a 5-level LOD system that dynamically adjusts terrain resolution
//! based on camera distance. The system uses exponential voxel size scaling (0.25m → 4.0m)
//! with hysteresis to prevent flickering during transitions.
//!
//! ## Core Concepts
//!
//! - **[`LodLevel`]**: Enum representing the 5 LOD levels (Lod0-Lod4)
//! - **[`RegionCoord`]**: Fixed-size regions (e.g., 32m × 32m) that divide the world
//! - **[`ChunkManager`]**: Resource tracking which regions are loaded at which LOD level
//!
//! ## LOD Strategy
//!
//! Each LOD level doubles the voxel size while adjusting chunk count to maintain
//! world coverage:
//!
//! - **LOD 0** (0-35m): 0.25m voxels, 16 chunks per region (4×4 grid)
//! - **LOD 1** (35-75m): 0.5m voxels, 4 chunks per region (2×2 grid)
//! - **LOD 2** (75-150m): 1.0m voxels, 1 chunk per region
//! - **LOD 3** (150-300m): 2.0m voxels, 1 chunk per region
//! - **LOD 4** (>300m): 4.0m voxels, 1 chunk per region
//!
//! ## Hysteresis
//!
//! To prevent LOD flickering as the camera moves near distance thresholds,
//! transitions include a 10m hysteresis buffer. A region only changes LOD level
//! when the distance crosses the threshold by more than the buffer amount.
//!
//! ## Usage
//!
//! ```ignore
//! use voxel::lod::{ChunkManager, RegionCoord, LodLevel};
//!
//! fn update_terrain(
//!     mut chunk_manager: ResMut<ChunkManager>,
//!     camera: Query<&Transform, With<Camera>>,
//! ) {
//!     let camera_pos = camera.single().translation;
//!     let region = RegionCoord::from_world_pos(camera_pos, 32.0);
//!
//!     if let Some((lod, entities)) = chunk_manager.get_region(region) {
//!         // Region exists, check if LOD needs updating
//!     }
//! }
//! ```
//!
//! See `examples/dynamic_lod.rs` for a complete working implementation.

pub mod level;
pub mod manager;

// Re-export commonly used types
pub use level::{LodLevel, LOD_0_DISTANCE, LOD_1_DISTANCE, LOD_2_DISTANCE,
                LOD_3_DISTANCE, LOD_4_DISTANCE, HYSTERESIS_BUFFER};
pub use manager::{ChunkManager, RegionCoord};
