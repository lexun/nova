//! # Voxel System for Nova Engine
//!
//! A modular voxel system inspired by Enshrouded's approach to smooth, non-blocky voxel rendering
//! with support for destructible environments and creative building.
//!
//! ## Overview
//!
//! This crate provides the core data structures and algorithms for Nova Engine's voxel system.
//! The design focuses on:
//!
//! - Small, high-resolution voxels that combine for smooth surfaces
//! - Real-time modification and mesh regeneration
//! - Structural integrity for realistic destruction
//! - Performance optimization for large worlds
//!
//! ## Architecture
//!
//! The voxel system is built around several key components:
//!
//! - `VoxelType`: Enum defining different voxel materials
//! - `Chunk`: Fixed-size 3D grid of voxels (32³ resolution)
//! - `VoxelWorld`: World-level management of chunks and coordinates
//! - Meshing algorithms for converting voxel data to renderable geometry
//!
//! ## Module Organization
//!
//! - [`chunk`]: Core voxel and chunk data structures ([`Chunk`], [`Voxel`], [`VoxelType`])
//! - [`meshing`]: Greedy meshing algorithm for efficient voxel-to-mesh conversion
//! - [`atlas`]: Texture atlas generation for voxel materials
//! - [`lod`]: Level of Detail system ([`lod::LodLevel`], [`lod::ChunkManager`])
//! - [`terrain`]: Procedural terrain generation utilities ([`terrain::terrain_height`])
//!
//! ## Quick Start
//!
//! The easiest way to use the voxel system is via the high-level [`VoxelTerrain`] API:
//!
//! ```ignore
//! use bevy::prelude::*;
//! use voxel::{VoxelTerrain, VoxelTerrainPlugin};
//!
//! App::new()
//!     .add_plugins(DefaultPlugins)
//!     .add_plugins(VoxelTerrainPlugin)
//!     .add_systems(Startup, setup)
//!     .run();
//!
//! fn setup(mut commands: Commands) {
//!     commands.spawn(VoxelTerrain::planar(512.0));  // That's it!
//!     // Camera, lights, etc.
//! }
//! ```
//!
//! ## Examples
//!
//! The crate includes several examples demonstrating different aspects of the voxel system:
//!
//! ```bash
//! # Simple high-level API (recommended starting point)
//! cargo run -p voxel --example simple_terrain
//!
//! # Large-scale terrain with production LOD settings
//! cargo run -p voxel --example large_scale_terrain --release
//!
//! # Multi-chunk terrain with chunk boundaries
//! cargo run -p voxel --example multi_chunk_terrain
//!
//! # Side-by-side LOD comparison (1, 4, and 16 chunks)
//! cargo run -p voxel --example lod_comparison
//!
//! # Manual LOD transitions (low-level control)
//! cargo run -p voxel --example dynamic_lod
//!
//! # Greedy meshing validation with 8 test patterns
//! cargo run -p voxel --example meshing_validation
//! ```
//!
//! ## Level of Detail (LOD)
//!
//! The LOD system provides 5 levels with exponential voxel size scaling:
//!
//! - **LOD 0** (0-35m): 0.25m voxels, 16 chunks per 32m² region (highest detail)
//! - **LOD 1** (35-75m): 0.5m voxels, 4 chunks per region
//! - **LOD 2** (75-150m): 1.0m voxels, 1 chunk per region
//! - **LOD 3** (150-300m): 2.0m voxels, 1 chunk per region
//! - **LOD 4** (>300m): 4.0m voxels, 1 chunk per region (lowest detail)
//!
//! Distance-based transitions use hysteresis to prevent flickering. See [`lod::LodLevel`]
//! for details on distance thresholds and [`lod::ChunkManager`] for region management.

use bevy::prelude::*;

// Core modules
pub mod chunk;
pub mod atlas;
pub mod textures;
pub mod meshing;

// System modules
pub mod lod;
pub mod terrain;
pub mod plugin;

// Re-export core types for backwards compatibility
pub use chunk::{Chunk, Voxel, VoxelType, CHUNK_SIZE};

// Re-export high-level API
pub use plugin::{VoxelTerrain, VoxelTerrainPlugin};

// Re-export terrain generators
pub use terrain::{CaveTerrainGenerator, DefaultTerrainGenerator, TerrainGenerator};

/// Voxel system plugin for Bevy
pub struct VoxelSystemPlugin;

impl Plugin for VoxelSystemPlugin {
    fn build(&self, _app: &mut App) {
        // TODO: Add voxel systems once we have mesh generation
    }
}
