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
//! - `chunk`: Core voxel and chunk data structures
//! - `meshing`: Greedy meshing algorithm for efficient voxel-to-mesh conversion
//! - `atlas`: Texture atlas generation for voxel materials
//! - `lod`: Level of Detail system for dynamic terrain resolution
//! - `terrain`: Procedural terrain generation utilities

use bevy::prelude::*;

// Core modules
pub mod chunk;
pub mod atlas;
pub mod meshing;

// System modules (to be populated in Phase 3)
pub mod lod;
pub mod terrain;

// Re-export core types for backwards compatibility
pub use chunk::{Chunk, Voxel, VoxelType, CHUNK_SIZE};

/// Voxel system plugin for Bevy
pub struct VoxelSystemPlugin;

impl Plugin for VoxelSystemPlugin {
    fn build(&self, _app: &mut App) {
        // TODO: Add voxel systems once we have mesh generation
    }
}
