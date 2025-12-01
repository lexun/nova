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
//! - [`textures`]: Separate texture generation for voxel materials
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
//! Examples are organized by category:
//!
//! ### Demos (feature showcases)
//! ```bash
//! cargo run -p voxel --example demo_simple_terrain      # High-level API (start here)
//! cargo run -p voxel --example demo_large_terrain       # Production LOD (2048m world)
//! cargo run -p voxel --example demo_grass_terrain       # Minecraft-style grass
//! cargo run -p voxel --example demo_caves               # Cave generation (BROKEN)
//! cargo run -p voxel --example demo_octree              # 3D octree LOD (BROKEN)
//! ```
//!
//! ### Debug tools (visual debugging)
//! ```bash
//! cargo run -p voxel --example debug_lod_comparison     # Side-by-side LOD levels
//! cargo run -p voxel --example debug_meshing_patterns   # Greedy meshing test patterns
//! cargo run -p voxel --example debug_uv_complex         # UV tiling on various shapes
//! cargo run -p voxel --example debug_uv_single          # Single voxel UV orientation
//! cargo run -p voxel --example debug_uv_horizontal      # 2x1 horizontal UV tiling
//! cargo run -p voxel --example debug_uv_vertical        # 1x2 vertical UV tiling
//! cargo run -p voxel --example debug_grass_single       # Single grass voxel faces
//! cargo run -p voxel --example debug_grass_stack        # Dirt+Grass vertical stack
//! cargo run -p voxel --example debug_grass_horizontal   # 2x1 horizontal grass
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
pub mod textures;
pub mod meshing;

// System modules
pub mod lod;
pub mod terrain;
pub mod plugin;

// Re-export core types for backwards compatibility
pub use chunk::{Chunk, Voxel, VoxelType, CHUNK_SIZE};
pub use meshing::FaceDir;

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
