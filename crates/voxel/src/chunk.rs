//! Core voxel and chunk data structures
//!
//! Provides the fundamental types for representing voxel data in fixed-size chunks.

/// Types of voxel materials
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VoxelType {
    Air,
    Stone,
    Dirt,
    Grass,
}

impl Default for VoxelType {
    fn default() -> Self {
        Self::Air
    }
}

/// A single voxel with material type and density
#[derive(Debug, Clone, Copy)]
pub struct Voxel {
    pub voxel_type: VoxelType,
    pub density: u8, // 0 = air, 255 = solid
}

impl Default for Voxel {
    fn default() -> Self {
        Self {
            voxel_type: VoxelType::Air,
            density: 0,
        }
    }
}

/// Fixed-size chunk of voxels (32×32×32)
pub const CHUNK_SIZE: usize = 32;

#[derive(Debug)]
pub struct Chunk {
    voxels: [[[Voxel; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE],
}

impl Default for Chunk {
    fn default() -> Self {
        Self {
            voxels: [[[Voxel::default(); CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE],
        }
    }
}

impl Chunk {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_voxel(&self, x: usize, y: usize, z: usize) -> Option<&Voxel> {
        if x < CHUNK_SIZE && y < CHUNK_SIZE && z < CHUNK_SIZE {
            Some(&self.voxels[x][y][z])
        } else {
            None
        }
    }

    pub fn set_voxel(&mut self, x: usize, y: usize, z: usize, voxel: Voxel) -> bool {
        if x < CHUNK_SIZE && y < CHUNK_SIZE && z < CHUNK_SIZE {
            self.voxels[x][y][z] = voxel;
            true
        } else {
            false
        }
    }
}
