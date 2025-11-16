//! Integration tests for LOD configuration system

use voxel::lod::{LodLevel, LodSettings};

#[test]
fn test_settings_affect_distance_thresholds() {
    let demo = LodSettings::demo();
    let high_end = LodSettings::high_end();

    // Same distance should give different LOD levels with different settings
    let distance = 100.0;

    let demo_lod = LodLevel::from_distance_with_settings(distance, &demo);
    let high_end_lod = LodLevel::from_distance_with_settings(distance, &high_end);

    // Demo: 100m is LOD 2 (75-150m range)
    assert_eq!(demo_lod, LodLevel::Lod2);

    // High-end: 100m is LOD 1 (75-150m becomes 150-300m with 2× scale, so 100m is in LOD 1 which is 70-150m)
    assert_eq!(high_end_lod, LodLevel::Lod1);
}

#[test]
fn test_settings_affect_voxel_sizes() {
    let demo = LodSettings::demo();
    let production = LodSettings::production();

    let lod = LodLevel::Lod0;

    let demo_size = lod.voxel_size_with_settings(&demo);
    let prod_size = lod.voxel_size_with_settings(&production);

    // Demo uses base 0.25m voxels
    assert_eq!(demo_size, 0.25);

    // Production uses 0.5× scale = 0.125m voxels (higher detail)
    assert_eq!(prod_size, 0.125);
}

#[test]
fn test_hysteresis_with_settings() {
    let settings = LodSettings::high_end();

    // Start at LOD 2
    let current = LodLevel::Lod2;

    // Just below threshold shouldn't change (within hysteresis)
    let distance = settings.lod_2_distance() - settings.hysteresis + 1.0;
    let new_lod = LodLevel::from_distance_with_hysteresis_settings(distance, current, &settings);
    assert_eq!(new_lod, LodLevel::Lod2);

    // Well below threshold should upgrade
    let distance = settings.lod_1_distance() - settings.hysteresis - 10.0;
    let new_lod = LodLevel::from_distance_with_hysteresis_settings(distance, current, &settings);
    assert_eq!(new_lod, LodLevel::Lod1);
}

#[test]
fn test_backward_compatibility() {
    // Old methods still work without settings
    let distance = 50.0;
    let lod = LodLevel::from_distance(distance);
    assert_eq!(lod, LodLevel::Lod1); // 35-75m range

    let voxel_size = lod.voxel_size();
    assert_eq!(voxel_size, 0.5);
}

#[test]
fn test_preset_ranges() {
    let demo = LodSettings::demo();
    let low_end = LodSettings::low_end();
    let high_end = LodSettings::high_end();
    let production = LodSettings::production();
    let infinite = LodSettings::infinite();

    // Check max view distances scale as expected
    assert_eq!(demo.max_view_distance, 400.0);
    assert_eq!(low_end.max_view_distance, 200.0);
    assert_eq!(high_end.max_view_distance, 800.0);
    assert_eq!(production.max_view_distance, 1000.0);
    assert_eq!(infinite.max_view_distance, 2000.0);

    // Check distance scaling
    assert_eq!(demo.distance_scale, 1.0);
    assert_eq!(low_end.distance_scale, 0.5);
    assert_eq!(high_end.distance_scale, 2.0);
    assert_eq!(production.distance_scale, 2.0);
    assert_eq!(infinite.distance_scale, 4.0);

    // Check voxel scaling
    assert_eq!(demo.voxel_scale, 1.0);
    assert_eq!(production.voxel_scale, 0.5); // Higher detail
}

#[test]
fn test_all_lod_levels_with_custom_settings() {
    let settings = LodSettings {
        distance_scale: 2.5,
        voxel_scale: 0.75,
        max_view_distance: 1200.0,
        hysteresis: 8.0,
    };

    // Verify all LOD levels can be calculated
    assert_eq!(settings.lod_0_distance(), 35.0 * 2.5);
    assert_eq!(settings.lod_1_distance(), 75.0 * 2.5);
    assert_eq!(settings.lod_2_distance(), 150.0 * 2.5);
    assert_eq!(settings.lod_3_distance(), 300.0 * 2.5);
    assert_eq!(settings.lod_4_distance(), 400.0 * 2.5);

    // Verify all voxel sizes scale
    assert_eq!(settings.lod_0_voxel_size(), 0.25 * 0.75);
    assert_eq!(settings.lod_1_voxel_size(), 0.5 * 0.75);
    assert_eq!(settings.lod_2_voxel_size(), 1.0 * 0.75);
    assert_eq!(settings.lod_3_voxel_size(), 2.0 * 0.75);
    assert_eq!(settings.lod_4_voxel_size(), 4.0 * 0.75);
}
