//! # Test: Procedural Terrain Generation
//!
//! Generates a coherent multi-chunk terrain using continuous noise.
//! Chunks seamlessly connect with matching edges.
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
use voxel::{Chunk, Voxel, VoxelType, CHUNK_SIZE};

const VOXEL_SIZE: f32 = 0.25;
const TERRAIN_GRID_SIZE: usize = 5; // 5x5 grid of chunks

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Camera positioned to view terrain
    let grid_world_size = (TERRAIN_GRID_SIZE as f32 * CHUNK_SIZE as f32 * VOXEL_SIZE) / 2.0;
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(grid_world_size, 30.0, grid_world_size + 40.0)
            .looking_at(Vec3::new(grid_world_size, 0.0, grid_world_size), Vec3::Y),
        FlyCameraController::new(15.0, 0.003),
    ));

    // Directional light (sun)
    commands.spawn((
        DirectionalLight {
            illuminance: 10000.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, -0.7, 0.4, 0.0)),
    ));

    // Materials
    let grass_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.3, 0.7, 0.2),
        perceptual_roughness: 0.9,
        ..default()
    });

    let dirt_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.5, 0.3, 0.1),
        perceptual_roughness: 0.85,
        ..default()
    });

    let stone_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.5, 0.5, 0.5),
        perceptual_roughness: 0.8,
        ..default()
    });

    info!("Generating {}x{} terrain grid...", TERRAIN_GRID_SIZE, TERRAIN_GRID_SIZE);

    let chunk_world_size = CHUNK_SIZE as f32 * VOXEL_SIZE;

    // Generate terrain chunks
    for grid_x in 0..TERRAIN_GRID_SIZE {
        for grid_z in 0..TERRAIN_GRID_SIZE {
            let chunk_offset_x = grid_x * CHUNK_SIZE;
            let chunk_offset_z = grid_z * CHUNK_SIZE;

            let chunk = generate_terrain_chunk(chunk_offset_x, chunk_offset_z);
            let mesh = voxel::meshing::generate_chunk_mesh(&chunk, VOXEL_SIZE);

            let position = Vec3::new(
                grid_x as f32 * chunk_world_size,
                0.0,
                grid_z as f32 * chunk_world_size,
            );

            // Choose material based on average terrain height (simple approach)
            let material = if grid_x == 0 || grid_z == 0 {
                stone_material.clone()
            } else if grid_x % 2 == 0 {
                dirt_material.clone()
            } else {
                grass_material.clone()
            };

            commands.spawn((
                Mesh3d(meshes.add(mesh)),
                MeshMaterial3d(material),
                Transform::from_translation(position),
                Wireframe,
            ));
        }
    }

    info!("Terrain generation complete!");
}

/// Generate a terrain chunk at a given world offset
/// Uses continuous noise so adjacent chunks match perfectly
fn generate_terrain_chunk(chunk_offset_x: usize, chunk_offset_z: usize) -> Chunk {
    let mut chunk = Chunk::new();

    for local_x in 0..CHUNK_SIZE {
        for local_z in 0..CHUNK_SIZE {
            // World coordinates (continuous across chunks)
            let world_x = (chunk_offset_x + local_x) as f32;
            let world_z = (chunk_offset_z + local_z) as f32;

            // Generate terrain height using multiple octaves of noise
            let height = terrain_height(world_x, world_z);

            // Fill from bottom to height
            let height_voxels = (height / VOXEL_SIZE) as usize;
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

/// Calculate terrain height at world coordinates
/// Uses layered sine waves (simple procedural approach)
/// Replace with proper noise (Perlin/Simplex) for production
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
