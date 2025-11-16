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

                // Fill voxels from bottom to height with layered materials
                // Clamp to chunk bounds, but calculate materials based on actual height
                let max_y = height_voxels.min(CHUNK_SIZE - 1);
                for local_y in 0..=max_y {
                    // Calculate depth from ACTUAL surface, not clamped height
                    let depth_from_surface = height_voxels.saturating_sub(local_y);

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

/// 3D terrain generator with caves, tunnels, and overhangs
///
/// Uses density-based voxel generation with 3D noise for cave carving.
/// Unlike heightmap generators, this samples density at every 3D position,
/// allowing for true 3D features like caves, tunnels, and floating islands.
#[derive(Debug, Clone)]
pub struct CaveTerrainGenerator {
    /// Maximum world size to generate (prevents generating outside intended bounds)
    pub max_world_size: Option<f32>,
    /// Cave density threshold (higher = more caves)
    pub cave_threshold: f32,
    /// Cave noise frequency (higher = smaller caves)
    pub cave_frequency: f32,
}

impl CaveTerrainGenerator {
    /// Create a new cave terrain generator with default settings
    pub fn new() -> Self {
        Self {
            max_world_size: None,
            cave_threshold: 0.3,
            cave_frequency: 0.05,
        }
    }

    /// Set a maximum world size boundary
    pub fn with_max_world_size(mut self, size: f32) -> Self {
        self.max_world_size = Some(size);
        self
    }

    /// Set cave threshold (0.0-1.0, higher = more caves)
    pub fn with_cave_threshold(mut self, threshold: f32) -> Self {
        self.cave_threshold = threshold.clamp(0.0, 1.0);
        self
    }

    /// Set cave frequency (higher = smaller, more detailed caves)
    pub fn with_cave_frequency(mut self, frequency: f32) -> Self {
        self.cave_frequency = frequency;
        self
    }

    /// Calculate density at a 3D world position
    ///
    /// Returns a value where:
    /// - density > 0 = solid terrain
    /// - density <= 0 = air/cave
    fn density_at(&self, world_pos: Vec3) -> f32 {
        // Get surface height using existing terrain function
        let surface_height = terrain_height(world_pos.x, world_pos.z);

        // Base density: positive below surface, negative above
        let distance_to_surface = surface_height - world_pos.y;
        let mut density = distance_to_surface;

        // Add cave carving using 3D noise
        // Use simple 3D sine-based noise for now (could use proper Perlin/Simplex later)
        let cave_noise = Self::noise_3d(
            world_pos.x * self.cave_frequency,
            world_pos.y * self.cave_frequency,
            world_pos.z * self.cave_frequency,
        );

        // Carve caves where noise exceeds threshold
        // Only carve underground (not above surface)
        if world_pos.y < surface_height - 2.0 {
            if cave_noise > self.cave_threshold {
                density -= 10.0; // Strong negative = definite air
            }
        }

        density
    }

    /// Simple 3D noise function using layered sine waves
    ///
    /// Returns value in range [-1, 1]
    fn noise_3d(x: f32, y: f32, z: f32) -> f32 {
        // Layer multiple sine waves for more organic patterns
        let n1 = (x * 1.0).sin() * (y * 1.1).cos() * (z * 0.9).sin();
        let n2 = (x * 2.3 + y * 1.7).sin() * (z * 2.1).cos() * 0.5;
        let n3 = (x * 4.7 - z * 3.9).cos() * (y * 5.1).sin() * 0.25;

        (n1 + n2 + n3) / 1.75
    }

    /// Determine voxel type based on depth from surface and material layers
    fn voxel_type_at(&self, world_pos: Vec3, surface_height: f32) -> VoxelType {
        let depth_from_surface = surface_height - world_pos.y;

        if depth_from_surface < 0.5 {
            VoxelType::Grass // Top layer
        } else if depth_from_surface < 3.0 {
            VoxelType::Dirt // Few layers of dirt
        } else {
            VoxelType::Stone // Deep stone
        }
    }
}

impl Default for CaveTerrainGenerator {
    fn default() -> Self {
        Self::new()
    }
}

impl TerrainGenerator for CaveTerrainGenerator {
    fn generate_chunk(&self, world_offset: Vec3, voxel_size: f32) -> Chunk {
        let mut chunk = Chunk::new();

        for local_x in 0..CHUNK_SIZE {
            for local_y in 0..CHUNK_SIZE {
                for local_z in 0..CHUNK_SIZE {
                    let world_x = world_offset.x + (local_x as f32 * voxel_size);
                    let world_y = world_offset.y + (local_y as f32 * voxel_size);
                    let world_z = world_offset.z + (local_z as f32 * voxel_size);

                    let world_pos = Vec3::new(world_x, world_y, world_z);

                    // Skip voxels outside max world size if set
                    if let Some(max_size) = self.max_world_size {
                        if world_x >= max_size || world_z >= max_size {
                            continue;
                        }
                    }

                    // Calculate density at this position
                    let density = self.density_at(world_pos);

                    // Only place voxel if density is positive (solid)
                    if density > 0.0 {
                        let surface_height = terrain_height(world_x, world_z);
                        let voxel_type = self.voxel_type_at(world_pos, surface_height);

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
        }

        chunk
    }
}
