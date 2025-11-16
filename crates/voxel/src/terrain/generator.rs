//! Terrain generation trait and implementations
//!
//! Provides a pluggable system for generating voxel terrain with different algorithms.

use crate::{Chunk, Voxel, VoxelType, CHUNK_SIZE};
use bevy::math::Vec3;

/// Trait for procedural terrain generation
///
/// Implementors define how to fill a chunk with voxels based on world position and voxel size.
/// This abstraction allows different terrain types (flat, hilly, mountainous, etc.) to be
/// swapped easily.
pub trait TerrainGenerator: Send + Sync {
    /// Generate voxel data for a chunk at the given world position
    ///
    /// # Arguments
    /// * `world_offset` - World-space position of the chunk's origin (bottom-left-front corner)
    /// * `voxel_size` - Size of each voxel in meters
    ///
    /// # Returns
    /// A filled Chunk with voxels set based on the generator's algorithm
    fn generate_chunk(&self, world_offset: Vec3, voxel_size: f32) -> Chunk;
}

/// Calculate terrain height at world coordinates (scale-independent)
///
/// Uses layered sine waves to create varied terrain with multiple frequencies:
/// - Large rolling hills (dominant features visible from distance)
/// - Medium frequency variation (adds character to landscapes)
/// - Small detail (only visible up close)
///
/// # Arguments
/// * `world_x` - X coordinate in world space (meters)
/// * `world_z` - Z coordinate in world space (meters)
///
/// # Returns
/// Height in meters at the given coordinates
///
/// # Note
/// This function is scale-independent - it works in world-space meters,
/// so the same coordinates will produce the same height regardless of voxel size.
/// This property is critical for LOD systems where the same terrain must be
/// recognizable at different detail levels.
pub fn terrain_height(world_x: f32, world_z: f32) -> f32 {
    // Base height
    let base = 2.0;

    // Large rolling hills (spans entire world)
    let hills = (world_x * 0.02).sin() * (world_z * 0.02).cos() * 8.0;

    // Medium frequency variation
    let medium = (world_x * 0.1 + world_z * 0.08).sin() * 3.0;

    // Small detail
    let detail = (world_x * 0.4).cos() * (world_z * 0.35).sin() * 1.0;

    // Combine and ensure positive
    (base + hills + medium + detail).max(0.5)
}

/// Default terrain generator using height-based generation
///
/// Uses the `terrain_height()` function to create rolling hills with
/// layered materials (stone base, dirt, grass on top).
#[derive(Debug, Clone, Default)]
pub struct DefaultTerrainGenerator {
    /// Maximum world size to generate (prevents generating outside intended bounds)
    pub max_world_size: Option<f32>,
}

impl DefaultTerrainGenerator {
    /// Create a new default terrain generator
    pub fn new() -> Self {
        Self::default()
    }

    /// Set a maximum world size boundary
    pub fn with_max_world_size(mut self, size: f32) -> Self {
        self.max_world_size = Some(size);
        self
    }
}

impl TerrainGenerator for DefaultTerrainGenerator {
    fn generate_chunk(&self, world_offset: Vec3, voxel_size: f32) -> Chunk {
        let mut chunk = Chunk::new();

        for local_x in 0..CHUNK_SIZE {
            for local_z in 0..CHUNK_SIZE {
                let world_x = world_offset.x + (local_x as f32 * voxel_size);
                let world_z = world_offset.z + (local_z as f32 * voxel_size);

                // Skip voxels outside max world size if set
                if let Some(max_size) = self.max_world_size {
                    if world_x >= world_offset.x + max_size || world_z >= world_offset.z + max_size {
                        continue;
                    }
                }

                // Get terrain height in world space (meters)
                let height_meters = terrain_height(world_x, world_z);

                // Convert to voxel coordinates
                let height_voxels = (height_meters / voxel_size) as usize;
                let max_height = height_voxels.min(CHUNK_SIZE - 1);

                // Fill voxels from bottom to height with layered materials
                for local_y in 0..=max_height {
                    let depth_from_surface = max_height.saturating_sub(local_y);

                    let voxel_type = if depth_from_surface == 0 {
                        VoxelType::Grass // Top layer
                    } else if depth_from_surface < 3 {
                        VoxelType::Dirt // Few layers of dirt
                    } else {
                        VoxelType::Stone // Stone foundation
                    };

                    chunk.set_voxel(
                        local_x,
                        local_y,
                        local_z,
                        Voxel {
                            voxel_type,
                            density: 255,
                        },
                    );
                }
            }
        }

        chunk
    }
}

/// Flat terrain generator - generates a flat plane at a specific height
///
/// Useful for testing, debugging, or creating building platforms.
#[derive(Debug, Clone)]
pub struct FlatTerrainGenerator {
    /// Height of the flat plane in meters
    pub height: f32,
    /// Voxel type to use
    pub voxel_type: VoxelType,
}

impl FlatTerrainGenerator {
    /// Create a new flat terrain generator
    pub fn new(height: f32) -> Self {
        Self {
            height,
            voxel_type: VoxelType::Grass,
        }
    }

    /// Set the voxel type
    pub fn with_voxel_type(mut self, voxel_type: VoxelType) -> Self {
        self.voxel_type = voxel_type;
        self
    }
}

impl TerrainGenerator for FlatTerrainGenerator {
    fn generate_chunk(&self, _world_offset: Vec3, voxel_size: f32) -> Chunk {
        let mut chunk = Chunk::new();

        // Calculate height in voxels
        let height_voxels = (self.height / voxel_size) as usize;
        let max_height = height_voxels.min(CHUNK_SIZE - 1);

        // Fill up to height
        for local_x in 0..CHUNK_SIZE {
            for local_z in 0..CHUNK_SIZE {
                for local_y in 0..=max_height {
                    chunk.set_voxel(
                        local_x,
                        local_y,
                        local_z,
                        Voxel {
                            voxel_type: self.voxel_type,
                            density: 255,
                        },
                    );
                }
            }
        }

        chunk
    }
}

/// Debug pattern generator - creates visible grid patterns
///
/// Generates alternating checkerboard or striped patterns to visualize
/// chunk boundaries and LOD transitions.
#[derive(Debug, Clone)]
pub struct DebugPatternGenerator {
    /// Pattern type
    pub pattern: DebugPattern,
    /// Height of the pattern plane
    pub height: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DebugPattern {
    /// Checkerboard pattern (alternating blocks)
    Checkerboard,
    /// Stripe pattern (parallel lines)
    Stripes,
}

impl DebugPatternGenerator {
    /// Create a new debug pattern generator
    pub fn new(pattern: DebugPattern, height: f32) -> Self {
        Self { pattern, height }
    }

    /// Checkerboard pattern at given height
    pub fn checkerboard(height: f32) -> Self {
        Self::new(DebugPattern::Checkerboard, height)
    }

    /// Stripe pattern at given height
    pub fn stripes(height: f32) -> Self {
        Self::new(DebugPattern::Stripes, height)
    }
}

impl TerrainGenerator for DebugPatternGenerator {
    fn generate_chunk(&self, world_offset: Vec3, voxel_size: f32) -> Chunk {
        let mut chunk = Chunk::new();

        let height_voxels = (self.height / voxel_size) as usize;
        let max_height = height_voxels.min(CHUNK_SIZE - 1);

        for local_x in 0..CHUNK_SIZE {
            for local_z in 0..CHUNK_SIZE {
                let world_x = world_offset.x + (local_x as f32 * voxel_size);
                let world_z = world_offset.z + (local_z as f32 * voxel_size);

                // Determine voxel type based on pattern
                let voxel_type = match self.pattern {
                    DebugPattern::Checkerboard => {
                        // Checkerboard based on world position
                        let grid_x = (world_x / 8.0).floor() as i32;
                        let grid_z = (world_z / 8.0).floor() as i32;
                        if (grid_x + grid_z) % 2 == 0 {
                            VoxelType::Stone
                        } else {
                            VoxelType::Grass
                        }
                    }
                    DebugPattern::Stripes => {
                        // Stripes along X axis
                        let grid_x = (world_x / 8.0).floor() as i32;
                        if grid_x % 2 == 0 {
                            VoxelType::Stone
                        } else {
                            VoxelType::Dirt
                        }
                    }
                };

                for local_y in 0..=max_height {
                    chunk.set_voxel(
                        local_x,
                        local_y,
                        local_z,
                        Voxel {
                            voxel_type,
                            density: 255,
                        },
                    );
                }
            }
        }

        chunk
    }
}
