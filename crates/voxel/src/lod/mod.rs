//! Level of Detail (LOD) system for dynamic terrain resolution
//!
//! Provides types and utilities for managing multiple levels of detail based on camera distance.

pub mod level;
pub mod manager;

// Re-export commonly used types
pub use level::{LodLevel, LOD_0_DISTANCE, LOD_1_DISTANCE, LOD_2_DISTANCE,
                LOD_3_DISTANCE, LOD_4_DISTANCE, HYSTERESIS_BUFFER};
pub use manager::{ChunkManager, RegionCoord};
