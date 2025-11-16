//! LOD level definitions and distance thresholds
//!
//! Provides a 5-level LOD system with 2× voxel size jumps between levels.

use super::config::LodSettings;

/// LOD distance thresholds (in meters) - BASE VALUES
///
/// These constants define the base LOD distances used when LodSettings.distance_scale = 1.0.
/// For configurable distances, use LodSettings resource instead.
pub const LOD_0_DISTANCE: f32 = 35.0;   // 0.25m voxels
pub const LOD_1_DISTANCE: f32 = 75.0;   // 0.5m voxels
pub const LOD_2_DISTANCE: f32 = 150.0;  // 1.0m voxels
pub const LOD_3_DISTANCE: f32 = 300.0;  // 2.0m voxels
pub const LOD_4_DISTANCE: f32 = 400.0;  // 4.0m voxels, culled beyond this

/// Hysteresis buffer to prevent LOD flickering - BASE VALUE
pub const HYSTERESIS_BUFFER: f32 = 5.0;

/// Level of Detail enumeration
///
/// Defines 5 LOD levels with progressively larger voxel sizes.
/// Each level uses 2× the voxel size of the previous level for smooth transitions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LodLevel {
    /// Highest detail: 0.25m voxels, 4×4 chunks per region
    Lod0,
    /// High detail: 0.5m voxels, 2×2 chunks per region
    Lod1,
    /// Medium detail: 1.0m voxels, 1 chunk per region
    Lod2,
    /// Low detail: 2.0m voxels, 1 chunk per region
    Lod3,
    /// Lowest detail: 4.0m voxels, 1 chunk per region
    Lod4,
    /// Not rendered (too far from camera)
    None,
}

impl LodLevel {
    /// Determine LOD level based on distance from camera (using base distances)
    ///
    /// For configurable distances, use `from_distance_with_settings()` instead.
    pub fn from_distance(distance: f32) -> Self {
        if distance < LOD_0_DISTANCE {
            LodLevel::Lod0
        } else if distance < LOD_1_DISTANCE {
            LodLevel::Lod1
        } else if distance < LOD_2_DISTANCE {
            LodLevel::Lod2
        } else if distance < LOD_3_DISTANCE {
            LodLevel::Lod3
        } else if distance < LOD_4_DISTANCE {
            LodLevel::Lod4
        } else {
            LodLevel::None
        }
    }

    /// Determine LOD level based on distance with custom settings
    pub fn from_distance_with_settings(distance: f32, settings: &LodSettings) -> Self {
        if distance < settings.lod_0_distance() {
            LodLevel::Lod0
        } else if distance < settings.lod_1_distance() {
            LodLevel::Lod1
        } else if distance < settings.lod_2_distance() {
            LodLevel::Lod2
        } else if distance < settings.lod_3_distance() {
            LodLevel::Lod3
        } else if distance < settings.lod_4_distance() {
            LodLevel::Lod4
        } else {
            LodLevel::None
        }
    }

    /// Get the target LOD with hysteresis to prevent flickering (using base settings)
    ///
    /// Uses HYSTERESIS_BUFFER to add a distance buffer when transitioning between levels.
    /// This prevents rapid switching when the camera is near a threshold.
    ///
    /// For configurable hysteresis, use `from_distance_with_hysteresis_settings()`.
    pub fn from_distance_with_hysteresis(distance: f32, current: LodLevel) -> Self {
        match current {
            LodLevel::None => {
                // Coming into view - use regular thresholds
                Self::from_distance(distance)
            }
            LodLevel::Lod4 => {
                if distance < LOD_3_DISTANCE - HYSTERESIS_BUFFER {
                    LodLevel::Lod3
                } else if distance > LOD_4_DISTANCE + HYSTERESIS_BUFFER {
                    LodLevel::None
                } else {
                    LodLevel::Lod4
                }
            }
            LodLevel::Lod3 => {
                if distance < LOD_2_DISTANCE - HYSTERESIS_BUFFER {
                    LodLevel::Lod2
                } else if distance > LOD_3_DISTANCE + HYSTERESIS_BUFFER {
                    LodLevel::Lod4
                } else {
                    LodLevel::Lod3
                }
            }
            LodLevel::Lod2 => {
                if distance < LOD_1_DISTANCE - HYSTERESIS_BUFFER {
                    LodLevel::Lod1
                } else if distance > LOD_2_DISTANCE + HYSTERESIS_BUFFER {
                    LodLevel::Lod3
                } else {
                    LodLevel::Lod2
                }
            }
            LodLevel::Lod1 => {
                if distance < LOD_0_DISTANCE - HYSTERESIS_BUFFER {
                    LodLevel::Lod0
                } else if distance > LOD_1_DISTANCE + HYSTERESIS_BUFFER {
                    LodLevel::Lod2
                } else {
                    LodLevel::Lod1
                }
            }
            LodLevel::Lod0 => {
                if distance > LOD_0_DISTANCE + HYSTERESIS_BUFFER {
                    LodLevel::Lod1
                } else {
                    LodLevel::Lod0
                }
            }
        }
    }

    /// Get the target LOD with hysteresis using custom settings
    pub fn from_distance_with_hysteresis_settings(
        distance: f32,
        current: LodLevel,
        settings: &LodSettings,
    ) -> Self {
        let hyst = settings.hysteresis;
        match current {
            LodLevel::None => {
                // Coming into view - use regular thresholds
                Self::from_distance_with_settings(distance, settings)
            }
            LodLevel::Lod4 => {
                if distance < settings.lod_3_distance() - hyst {
                    LodLevel::Lod3
                } else if distance > settings.lod_4_distance() + hyst {
                    LodLevel::None
                } else {
                    LodLevel::Lod4
                }
            }
            LodLevel::Lod3 => {
                if distance < settings.lod_2_distance() - hyst {
                    LodLevel::Lod2
                } else if distance > settings.lod_3_distance() + hyst {
                    LodLevel::Lod4
                } else {
                    LodLevel::Lod3
                }
            }
            LodLevel::Lod2 => {
                if distance < settings.lod_1_distance() - hyst {
                    LodLevel::Lod1
                } else if distance > settings.lod_2_distance() + hyst {
                    LodLevel::Lod3
                } else {
                    LodLevel::Lod2
                }
            }
            LodLevel::Lod1 => {
                if distance < settings.lod_0_distance() - hyst {
                    LodLevel::Lod0
                } else if distance > settings.lod_1_distance() + hyst {
                    LodLevel::Lod2
                } else {
                    LodLevel::Lod1
                }
            }
            LodLevel::Lod0 => {
                if distance > settings.lod_0_distance() + hyst {
                    LodLevel::Lod1
                } else {
                    LodLevel::Lod0
                }
            }
        }
    }

    /// Get the voxel size in meters for this LOD level (using base sizes)
    ///
    /// For configurable voxel sizes, use `voxel_size_with_settings()`.
    pub fn voxel_size(&self) -> f32 {
        match self {
            LodLevel::Lod0 => 0.25,
            LodLevel::Lod1 => 0.5,
            LodLevel::Lod2 => 1.0,
            LodLevel::Lod3 => 2.0,
            LodLevel::Lod4 => 4.0,
            LodLevel::None => 0.0,
        }
    }

    /// Get the voxel size with custom settings
    pub fn voxel_size_with_settings(&self, settings: &LodSettings) -> f32 {
        match self {
            LodLevel::Lod0 => settings.lod_0_voxel_size(),
            LodLevel::Lod1 => settings.lod_1_voxel_size(),
            LodLevel::Lod2 => settings.lod_2_voxel_size(),
            LodLevel::Lod3 => settings.lod_3_voxel_size(),
            LodLevel::Lod4 => settings.lod_4_voxel_size(),
            LodLevel::None => 0.0,
        }
    }

    /// Get the number of chunks per edge for this LOD level
    ///
    /// Higher detail levels subdivide regions into more chunks.
    pub fn chunks_per_edge(&self) -> usize {
        match self {
            LodLevel::Lod0 => 4,  // 4×4 = 16 chunks
            LodLevel::Lod1 => 2,  // 2×2 = 4 chunks
            LodLevel::Lod2 => 1,  // 1 chunk
            LodLevel::Lod3 => 1,  // 1 chunk
            LodLevel::Lod4 => 1,  // 1 chunk
            LodLevel::None => 0,
        }
    }
}
