//! Terrain generation functions
//!
//! Provides procedural terrain height generation using layered sine waves.
//! This is a simple approach suitable for testing; production systems should use
//! proper noise functions (Perlin, Simplex, etc.).

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
