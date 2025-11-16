//! LOD configuration and preset definitions
//!
//! Provides configurable LOD settings with sensible presets for different use cases.

use bevy::prelude::*;

/// Configuration for Level of Detail system
///
/// Controls how voxel terrain detail changes with distance from camera.
/// All settings are multipliers/scales applied to base values.
#[derive(Debug, Clone, Resource)]
pub struct LodSettings {
    /// Multiplier for all LOD distance thresholds
    ///
    /// Default is 1.0. Values > 1.0 push transitions farther away,
    /// values < 1.0 bring them closer.
    ///
    /// Example: 2.0 means LOD transitions happen at 2× the base distances
    pub distance_scale: f32,

    /// Multiplier for all voxel sizes
    ///
    /// Default is 1.0. Values < 1.0 use smaller voxels (higher detail),
    /// values > 1.0 use larger voxels (lower detail).
    ///
    /// Example: 0.5 means all voxels are half the base size
    pub voxel_scale: f32,

    /// Maximum view distance in meters
    ///
    /// Chunks beyond this distance are not rendered.
    /// Default is 400m (matches LOD_4_DISTANCE).
    pub max_view_distance: f32,

    /// Hysteresis buffer in meters
    ///
    /// Distance buffer added when transitioning between LOD levels
    /// to prevent flickering when camera is near a threshold.
    /// Default is 5.0m.
    pub hysteresis: f32,
}

impl Default for LodSettings {
    fn default() -> Self {
        Self::demo()
    }
}

impl LodSettings {
    /// Demo preset - matches current hardcoded defaults
    ///
    /// Good for examples and testing. Transitions happen close together
    /// so all LOD levels are visible within a small area.
    ///
    /// - View distance: 400m
    /// - LOD transitions: 35m, 75m, 150m, 300m
    /// - Voxel sizes: 0.25m → 4.0m
    pub fn demo() -> Self {
        Self {
            distance_scale: 1.0,
            voxel_scale: 1.0,
            max_view_distance: 400.0,
            hysteresis: 5.0,
        }
    }

    /// Low-end preset - reduced view distance for performance
    ///
    /// Good for lower-end hardware or mobile devices.
    ///
    /// - View distance: 200m (50% of demo)
    /// - LOD transitions: 17.5m, 37.5m, 75m, 150m
    /// - Voxel sizes: 0.25m → 4.0m
    pub fn low_end() -> Self {
        Self {
            distance_scale: 0.5,
            voxel_scale: 1.0,
            max_view_distance: 200.0,
            hysteresis: 5.0,
        }
    }

    /// High-end preset - extended view distance
    ///
    /// Good for high-end hardware with distant horizons.
    ///
    /// - View distance: 800m (2× demo)
    /// - LOD transitions: 70m, 150m, 300m, 600m
    /// - Voxel sizes: 0.25m → 4.0m
    pub fn high_end() -> Self {
        Self {
            distance_scale: 2.0,
            voxel_scale: 1.0,
            max_view_distance: 800.0,
            hysteresis: 10.0,
        }
    }

    /// Production preset - balanced for realistic game scenarios
    ///
    /// Pushes high detail farther out, uses low detail only at extreme distances.
    /// Creates illusion of high-detail world.
    ///
    /// - View distance: 1000m (2.5× demo)
    /// - LOD transitions: 70m, 150m, 300m, 600m
    /// - Voxel sizes: 0.125m → 2.0m (2× higher detail)
    pub fn production() -> Self {
        Self {
            distance_scale: 2.0,
            voxel_scale: 0.5,
            max_view_distance: 1000.0,
            hysteresis: 10.0,
        }
    }

    /// Infinite preset - very large view distance
    ///
    /// For planetary-scale or open-world scenarios.
    ///
    /// - View distance: 2000m (5× demo)
    /// - LOD transitions: 140m, 300m, 600m, 1200m
    /// - Voxel sizes: 0.25m → 4.0m
    pub fn infinite() -> Self {
        Self {
            distance_scale: 4.0,
            voxel_scale: 1.0,
            max_view_distance: 2000.0,
            hysteresis: 20.0,
        }
    }

    /// Get the scaled LOD 0 distance threshold
    pub fn lod_0_distance(&self) -> f32 {
        35.0 * self.distance_scale
    }

    /// Get the scaled LOD 1 distance threshold
    pub fn lod_1_distance(&self) -> f32 {
        75.0 * self.distance_scale
    }

    /// Get the scaled LOD 2 distance threshold
    pub fn lod_2_distance(&self) -> f32 {
        150.0 * self.distance_scale
    }

    /// Get the scaled LOD 3 distance threshold
    pub fn lod_3_distance(&self) -> f32 {
        300.0 * self.distance_scale
    }

    /// Get the scaled LOD 4 distance threshold
    pub fn lod_4_distance(&self) -> f32 {
        self.max_view_distance.min(400.0 * self.distance_scale)
    }

    /// Get the scaled voxel size for LOD 0
    pub fn lod_0_voxel_size(&self) -> f32 {
        0.25 * self.voxel_scale
    }

    /// Get the scaled voxel size for LOD 1
    pub fn lod_1_voxel_size(&self) -> f32 {
        0.5 * self.voxel_scale
    }

    /// Get the scaled voxel size for LOD 2
    pub fn lod_2_voxel_size(&self) -> f32 {
        1.0 * self.voxel_scale
    }

    /// Get the scaled voxel size for LOD 3
    pub fn lod_3_voxel_size(&self) -> f32 {
        2.0 * self.voxel_scale
    }

    /// Get the scaled voxel size for LOD 4
    pub fn lod_4_voxel_size(&self) -> f32 {
        4.0 * self.voxel_scale
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_demo_preset() {
        let settings = LodSettings::demo();
        assert_eq!(settings.lod_0_distance(), 35.0);
        assert_eq!(settings.lod_4_distance(), 400.0);
        assert_eq!(settings.lod_0_voxel_size(), 0.25);
    }

    #[test]
    fn test_high_end_preset() {
        let settings = LodSettings::high_end();
        assert_eq!(settings.lod_0_distance(), 70.0);
        assert_eq!(settings.lod_4_distance(), 800.0);
    }

    #[test]
    fn test_production_preset() {
        let settings = LodSettings::production();
        // 2× distance scale
        assert_eq!(settings.lod_0_distance(), 70.0);
        // 0.5× voxel scale = higher detail
        assert_eq!(settings.lod_0_voxel_size(), 0.125);
        assert_eq!(settings.max_view_distance, 1000.0);
    }

    #[test]
    fn test_custom_settings() {
        let settings = LodSettings {
            distance_scale: 3.0,
            voxel_scale: 0.25,
            max_view_distance: 1500.0,
            hysteresis: 15.0,
        };
        assert_eq!(settings.lod_0_distance(), 105.0); // 35 * 3
        assert_eq!(settings.lod_0_voxel_size(), 0.0625); // 0.25 * 0.25
    }
}
