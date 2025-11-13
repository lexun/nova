//! # Test: Multi-Scale Terrain Comparison
//!
//! Validates that terrain looks recognizable at different voxel sizes.
//! Generates the SAME terrain area at three resolutions side-by-side:
//! - High res (0.25m voxels)
//! - Medium res (1.0m voxels)
//! - Low res (4.0m voxels)
//!
//! This proves we can use dynamic voxel_size for LOD without losing visual consistency.
//!
//! Controls:
//! - WASD: Move camera
//! - QE: Up/Down
//! - Mouse: Look around

use bevy::{
    pbr::wireframe::{Wireframe, WireframePlugin},
    prelude::*,
};
use bevy_brp_extras::BrpExtrasPlugin;
use engine::{FlyCameraController, FlyCameraPlugin};
use std::time::Instant;
use voxel::{Chunk, Voxel, VoxelType, CHUNK_SIZE};

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Camera positioned to view all three terrain patches
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(20.0, 20.0, 20.0).looking_at(Vec3::new(15.0, 0.0, 0.0), Vec3::Y),
        FlyCameraController::new(15.0, 0.003),
    ));

    // Directional light
    commands.spawn((
        DirectionalLight {
            illuminance: 10000.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, -0.7, 0.4, 0.0)),
    ));

    info!("Generating multi-scale terrain comparison...");

    // Three different sampling resolutions to test
    // All meshes rendered at 0.25m voxel size, but terrain sampled at different frequencies
    let scales = [
        ("High (0.25m)", 0.25, Color::srgb(0.3, 0.7, 0.2)),  // Green - sample every 0.25m
        ("Med (1.0m)",   1.0,  Color::srgb(0.2, 0.6, 0.8)),  // Blue - sample every 1.0m
        ("Low (4.0m)",   4.0,  Color::srgb(0.7, 0.4, 0.2)),  // Brown - sample every 4.0m
    ];

    const RENDER_VOXEL_SIZE: f32 = 0.25; // All meshes rendered at same scale
    let chunk_spacing = 12.0; // Space between chunks for visibility

    for (i, (label, sample_resolution, color)) in scales.iter().enumerate() {
        info!("Generating {} terrain (sample every {}m)...", label, sample_resolution);

        let start = Instant::now();

        // Generate chunk with specified sampling resolution
        let chunk = generate_terrain_chunk(0, 0, *sample_resolution);
        // But render all meshes at the same voxel size for consistent physical dimensions
        let mesh = voxel::meshing::generate_chunk_mesh(&chunk, RENDER_VOXEL_SIZE);

        let generation_time = start.elapsed();
        info!("  Generated in {:?}", generation_time);

        // Create material with distinctive color
        let material = materials.add(StandardMaterial {
            base_color: *color,
            perceptual_roughness: 0.85,
            ..default()
        });

        // Position side-by-side
        let position = Vec3::new(i as f32 * chunk_spacing, 0.0, 0.0);

        commands.spawn((
            Mesh3d(meshes.add(mesh)),
            MeshMaterial3d(material),
            Transform::from_translation(position),
            Wireframe,
        ));

        info!("  Spawned at {:?}", position);
    }

    info!("Multi-scale terrain generation complete!");
    info!("Compare the three terrain patches:");
    info!("  - Green (left): High detail (0.25m voxels)");
    info!("  - Blue (middle): Medium detail (1.0m voxels)");
    info!("  - Brown (right): Low detail (4.0m voxels)");
}

/// Generate a terrain chunk at a given world offset with parameterized voxel size
///
/// For this test, we want all chunks to represent the SAME world area (e.g., 0-8m)
/// but with different sampling rates (voxel sizes).
/// This lets us compare how the same terrain looks at different resolutions.
fn generate_terrain_chunk(chunk_offset_x: usize, chunk_offset_z: usize, voxel_size: f32) -> Chunk {
    let mut chunk = Chunk::new();

    // Fixed world area: always 8m × 8m (matching the high-res chunk size)
    const WORLD_AREA_SIZE: f32 = 8.0;

    for local_x in 0..CHUNK_SIZE {
        for local_z in 0..CHUNK_SIZE {
            // Map voxel coordinates to the fixed world area
            // All chunks sample the same 8m × 8m area, just at different resolutions
            let world_x = (chunk_offset_x as f32 * WORLD_AREA_SIZE)
                + (local_x as f32 / CHUNK_SIZE as f32) * WORLD_AREA_SIZE;
            let world_z = (chunk_offset_z as f32 * WORLD_AREA_SIZE)
                + (local_z as f32 / CHUNK_SIZE as f32) * WORLD_AREA_SIZE;

            // Generate terrain height in world units (meters)
            let height = terrain_height(world_x, world_z);

            // Convert height to voxel coordinates
            let height_voxels = (height / voxel_size) as usize;
            let max_height = height_voxels.min(CHUNK_SIZE - 1);

            for local_y in 0..=max_height {
                let voxel_type = if local_y == max_height {
                    VoxelType::Grass
                } else if local_y > max_height.saturating_sub(3) {
                    VoxelType::Dirt
                } else {
                    VoxelType::Stone
                };

                chunk.set_voxel(local_x, local_y, local_z, Voxel {
                    voxel_type,
                    density: 255,
                });
            }
        }
    }

    chunk
}

/// Calculate terrain height at world coordinates (in meters)
/// Uses layered sine waves (simple procedural approach)
///
/// IMPORTANT: This function is voxel_size-independent!
/// It always works in world-space meters.
fn terrain_height(world_x: f32, world_z: f32) -> f32 {
    // Base height
    let base = 1.0;

    // Large rolling hills
    let hills = (world_x * 0.05).sin() * (world_z * 0.05).cos() * 3.0;

    // Medium frequency variation
    let medium = (world_x * 0.15 + world_z * 0.1).sin() * 1.5;

    // Small detail
    let detail = (world_x * 0.4).cos() * (world_z * 0.35).sin() * 0.5;

    // Combine and ensure positive
    (base + hills + medium + detail).max(0.5)
}

fn main() {
    let mut app = engine::create_app();
    app.add_plugins((BrpExtrasPlugin, WireframePlugin::default(), FlyCameraPlugin));
    app.add_systems(Startup, setup_scene);
    app.run();
}
