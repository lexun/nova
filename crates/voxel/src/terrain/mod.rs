//! Procedural terrain generation utilities
//!
//! Provides terrain generation functions using noise-based algorithms.

pub mod generator;

// Re-export terrain generation function
pub use generator::terrain_height;
