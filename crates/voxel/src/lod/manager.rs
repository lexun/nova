//! Chunk management for LOD-based terrain systems
//!
//! Provides region-based chunk tracking and management for dynamic LOD terrain.
//! Regions divide the world into fixed-size areas that can be loaded/unloaded
//! at different LOD levels based on camera distance.

use bevy::prelude::*;
use std::collections::HashMap;
use super::LodLevel;

/// Coordinate for a terrain region in the world grid
///
/// Regions divide the world into fixed-size areas (e.g., 32m × 32m).
/// Each region can contain multiple chunks depending on the LOD level.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RegionCoord {
    /// X coordinate in region space
    pub x: i32,
    /// Z coordinate in region space
    pub z: i32,
}

impl RegionCoord {
    /// Create a new region coordinate
    pub fn new(x: i32, z: i32) -> Self {
        Self { x, z }
    }

    /// Convert world position to region coordinate
    ///
    /// # Arguments
    /// * `world_pos` - Position in world space (meters)
    /// * `region_size` - Size of each region (meters)
    pub fn from_world_pos(world_pos: Vec3, region_size: f32) -> Self {
        Self {
            x: (world_pos.x / region_size).floor() as i32,
            z: (world_pos.z / region_size).floor() as i32,
        }
    }

    /// Get the world-space position of this region's origin
    ///
    /// # Arguments
    /// * `region_size` - Size of each region (meters)
    pub fn to_world_pos(&self, region_size: f32) -> Vec3 {
        Vec3::new(
            self.x as f32 * region_size,
            0.0,
            self.z as f32 * region_size,
        )
    }
}

/// Manages active terrain regions and their LOD levels
///
/// Tracks which regions are currently loaded, at what LOD level,
/// and which chunk entities belong to each region. This enables
/// dynamic loading/unloading of terrain based on camera distance.
///
/// # Example Usage
///
/// ```ignore
/// #[derive(Resource, Default)]
/// struct MyChunkManager {
///     manager: ChunkManager,
/// }
///
/// fn update_terrain(
///     camera: Query<&Transform, With<Camera>>,
///     mut manager: ResMut<MyChunkManager>,
/// ) {
///     let camera_pos = camera.single().translation;
///     let region = RegionCoord::from_world_pos(camera_pos, 32.0);
///
///     // Check if region is loaded
///     if let Some((lod, entities)) = manager.manager.get_region(region) {
///         // Region exists, maybe update LOD
///     } else {
///         // Load new region
///     }
/// }
/// ```
#[derive(Resource, Default)]
pub struct ChunkManager {
    /// Maps region coordinates to their current LOD level and chunk entities
    active_regions: HashMap<RegionCoord, (LodLevel, Vec<Entity>)>,
}

impl ChunkManager {
    /// Create a new chunk manager
    pub fn new() -> Self {
        Self::default()
    }

    /// Get the LOD level and entities for a region, if it exists
    pub fn get_region(&self, coord: RegionCoord) -> Option<&(LodLevel, Vec<Entity>)> {
        self.active_regions.get(&coord)
    }

    /// Get mutable access to a region's data
    pub fn get_region_mut(&mut self, coord: RegionCoord) -> Option<&mut (LodLevel, Vec<Entity>)> {
        self.active_regions.get_mut(&coord)
    }

    /// Add or update a region with its LOD level and chunk entities
    pub fn insert_region(&mut self, coord: RegionCoord, lod: LodLevel, entities: Vec<Entity>) {
        self.active_regions.insert(coord, (lod, entities));
    }

    /// Remove a region and return its data
    pub fn remove_region(&mut self, coord: RegionCoord) -> Option<(LodLevel, Vec<Entity>)> {
        self.active_regions.remove(&coord)
    }

    /// Check if a region is currently loaded
    pub fn has_region(&self, coord: RegionCoord) -> bool {
        self.active_regions.contains_key(&coord)
    }

    /// Get the number of active regions
    pub fn region_count(&self) -> usize {
        self.active_regions.len()
    }

    /// Get the total number of chunk entities across all regions
    pub fn total_chunks(&self) -> usize {
        self.active_regions.values()
            .map(|(_, entities)| entities.len())
            .sum()
    }

    /// Get an iterator over all active region coordinates
    pub fn regions(&self) -> impl Iterator<Item = &RegionCoord> {
        self.active_regions.keys()
    }

    /// Get an iterator over all regions with their LOD levels and entities
    pub fn regions_with_data(&self) -> impl Iterator<Item = (&RegionCoord, &LodLevel, &Vec<Entity>)> {
        self.active_regions.iter()
            .map(|(coord, (lod, entities))| (coord, lod, entities))
    }

    /// Clear all regions
    pub fn clear(&mut self) {
        self.active_regions.clear();
    }
}
