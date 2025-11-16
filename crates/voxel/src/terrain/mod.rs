//! Procedural terrain generation utilities
//!
//! This module provides terrain generation functions that operate in world-space
//! coordinates, ensuring consistent terrain across all LOD levels.
//!
//! ## World-Space Coordinates
//!
//! All terrain generation functions use **world-space meters** as input, not voxel
//! coordinates. This ensures that the terrain height at world position (16.0, 16.0)
//! is identical whether using 4m voxels or 0.25m voxels - only the vertical
//! resolution differs.
//!
//! ## Usage
//!
//! ```ignore
//! use voxel::terrain::terrain_height;
//!
//! // Generate terrain at world position
//! let world_x = 128.0;  // meters
//! let world_z = 64.0;   // meters
//! let height_meters = terrain_height(world_x, world_z);
//!
//! // Convert to voxel coordinates for given voxel size
//! let voxel_size = 0.25;
//! let height_voxels = (height_meters / voxel_size) as usize;
//! ```
//!
//! ## LOD Consistency
//!
//! Because terrain generation uses world-space coordinates:
//! - Same noise values at all LOD levels
//! - No height quantization artifacts
//! - Seamless transitions between LOD levels
//! - Perfect terrain alignment across chunk boundaries
//!
//! See `examples/lod_comparison.rs` for visual demonstration of terrain consistency
//! across multiple LOD levels.

pub mod generator;

// Re-export terrain generation types
pub use generator::{
    terrain_height, CaveTerrainGenerator, DebugPattern, DebugPatternGenerator,
    DefaultTerrainGenerator, FlatTerrainGenerator, TerrainGenerator,
};
