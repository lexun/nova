//! # Test: Multi-Scale LOD Terrain Comparison
//!
//! Demonstrates realistic LOD behavior by rendering the SAME 32m × 32m world area
//! at three different detail levels with appropriate chunk counts:
//!
//! - Low Detail (far): 1 chunk with 4m voxels (partial coverage of 128m chunk)
//! - Medium Detail: 2×2 = 4 chunks with 1m voxels
//! - High Detail (close): 4×4 = 16 chunks with 0.25m voxels
//!
//! Key insight: Higher detail requires exponentially more chunks to cover same area!
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

const WORLD_AREA_SIZE: f32 = 32.0; // All LOD levels cover same 32m × 32m area

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Camera positioned to view all three LOD patches
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(50.0, 40.0, 50.0).looking_at(Vec3::new(50.0, 0.0, 0.0), Vec3::Y),
        FlyCameraController::new(20.0, 0.003),
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

    info!("Generating multi-LOD terrain comparison...");
    info!("All patches cover {}m × {}m world area", WORLD_AREA_SIZE, WORLD_AREA_SIZE);

    // Three LOD levels with different voxel sizes
    let lod_configs = [
        ("Low (4m voxels)",    4.0,  Color::srgb(0.7, 0.4, 0.2)),  // Brown - far distance
        ("Medium (1m voxels)", 1.0,  Color::srgb(0.2, 0.6, 0.8)),  // Blue - medium distance
        ("High (0.25m voxels)", 0.25, Color::srgb(0.3, 0.7, 0.2)),  // Green - close up
    ];

    let lod_spacing = 50.0; // Space between LOD patches

    for (lod_index, (label, voxel_size, color)) in lod_configs.iter().enumerate() {
        info!("\n=== Generating {} ===", label);

        let start = Instant::now();

        // Calculate chunk world size and how many chunks needed
        let chunk_world_size = CHUNK_SIZE as f32 * voxel_size;
        let chunks_per_edge = (WORLD_AREA_SIZE / chunk_world_size).ceil() as usize;
        let total_chunks = chunks_per_edge * chunks_per_edge;

        info!("  Voxel size: {}m", voxel_size);
        info!("  Chunk world size: {}m", chunk_world_size);
        info!("  Chunks per edge: {}", chunks_per_edge);
        info!("  Total chunks: {}", total_chunks);

        // Create material for this LOD
        let material = materials.add(StandardMaterial {
            base_color: *color,
            perceptual_roughness: 0.85,
            ..default()
        });

        // LOD patch position (side-by-side)
        let lod_base_position = Vec3::new(lod_index as f32 * lod_spacing, 0.0, 0.0);

        // Generate grid of chunks for this LOD level
        let mut chunk_count = 0;
        for grid_x in 0..chunks_per_edge {
            for grid_z in 0..chunks_per_edge {
                // World offset for this chunk
                let chunk_world_offset_x = grid_x as f32 * chunk_world_size;
                let chunk_world_offset_z = grid_z as f32 * chunk_world_size;

                // Skip chunks outside our target area
                if chunk_world_offset_x >= WORLD_AREA_SIZE || chunk_world_offset_z >= WORLD_AREA_SIZE {
                    continue;
                }

                // Generate chunk with terrain
                let chunk = generate_terrain_chunk(
                    chunk_world_offset_x,
                    chunk_world_offset_z,
                    *voxel_size,
                );

                // Generate mesh at native voxel size
                let mesh = voxel::meshing::generate_chunk_mesh(&chunk, *voxel_size);

                // Position chunk within LOD patch
                let chunk_position = lod_base_position + Vec3::new(
                    chunk_world_offset_x,
                    0.0,
                    chunk_world_offset_z,
                );

                commands.spawn((
                    Mesh3d(meshes.add(mesh)),
                    MeshMaterial3d(material.clone()),
                    Transform::from_translation(chunk_position),
                    Wireframe,
                ));

                chunk_count += 1;
            }
        }

        let elapsed = start.elapsed();
        info!("  Generated {} chunks in {:?}", chunk_count, elapsed);
        info!("  Average: {:?} per chunk", elapsed / chunk_count as u32);
    }

    info!("\n=== Terrain Generation Complete! ===");
    info!("Compare the three LOD patches:");
    info!("  Brown (left): Low detail - 1 chunk, 4m voxels, blocky");
    info!("  Blue (middle): Medium detail - 4 chunks, 1m voxels");
    info!("  Green (right): High detail - 16 chunks, 0.25m voxels, smooth");
}

/// Generate a terrain chunk at a given world offset with specified voxel size
///
/// Key: Uses world-space coordinates so terrain is consistent across all LOD levels
fn generate_terrain_chunk(world_offset_x: f32, world_offset_z: f32, voxel_size: f32) -> Chunk {
    let mut chunk = Chunk::new();

    for local_x in 0..CHUNK_SIZE {
        for local_z in 0..CHUNK_SIZE {
            // Calculate world position for this voxel
            let world_x = world_offset_x + (local_x as f32 * voxel_size);
            let world_z = world_offset_z + (local_z as f32 * voxel_size);

            // Get terrain height at this world position (in meters)
            let height_meters = terrain_height(world_x, world_z);

            // Convert to voxel coordinates
            let height_voxels = (height_meters / voxel_size) as usize;
            let max_height = height_voxels.min(CHUNK_SIZE - 1);

            // Fill column up to height
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
/// Uses layered sine waves for varied terrain
///
/// IMPORTANT: This function is scale-independent - works in world-space meters
fn terrain_height(world_x: f32, world_z: f32) -> f32 {
    // Base height
    let base = 2.0;

    // Large rolling hills (dominant features)
    let hills = (world_x * 0.1).sin() * (world_z * 0.1).cos() * 4.0;

    // Medium frequency variation
    let medium = (world_x * 0.3 + world_z * 0.2).sin() * 2.0;

    // Small detail (only visible at high LOD)
    let detail = (world_x * 0.8).cos() * (world_z * 0.7).sin() * 0.5;

    // Combine and ensure positive
    (base + hills + medium + detail).max(0.5)
}

fn main() {
    let mut app = engine::create_app();
    app.add_plugins((BrpExtrasPlugin, WireframePlugin::default(), FlyCameraPlugin));
    app.add_systems(Startup, setup_scene);
    app.run();
}
