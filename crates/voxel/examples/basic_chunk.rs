//! # Basic Chunk Example
//!
//! Minimal demonstration of creating and rendering a single voxel chunk.
//! Shows the fundamentals of the voxel system: chunk creation, voxel manipulation, and mesh generation.
//!
//! Controls:
//! - Click to capture mouse
//! - Escape to release mouse
//! - WASD: Move camera
//! - QE: Up/Down
//! - Shift: Speed boost
//! - Mouse: Look around (when captured)

use bevy::prelude::*;
use bevy_brp_extras::BrpExtrasPlugin;
use engine::camera::{FlyCameraController, FlyCameraPlugin};
use voxel::{Chunk, Voxel, VoxelType, CHUNK_SIZE};

const VOXEL_SIZE: f32 = 0.25;

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(15.0, 15.0, 15.0).looking_at(Vec3::ZERO, Vec3::Y),
        FlyCameraController::default(),
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

    // Create a simple chunk with a pyramid shape
    let mut chunk = Chunk::new();

    // Build a pyramid: solid base that tapers to a point
    for y in 0..CHUNK_SIZE {
        let layer_size = CHUNK_SIZE - y;
        let start = y / 2;

        for x in start..start + layer_size {
            for z in start..start + layer_size {
                if x < CHUNK_SIZE && z < CHUNK_SIZE {
                    chunk.set_voxel(x, y, z, Voxel {
                        voxel_type: if y < CHUNK_SIZE / 3 {
                            VoxelType::Stone
                        } else if y < (2 * CHUNK_SIZE) / 3 {
                            VoxelType::Dirt
                        } else {
                            VoxelType::Grass
                        },
                        density: 255,
                    });
                }
            }
        }
    }

    // Generate mesh from chunk
    let mesh = voxel::meshing::generate_chunk_mesh(&chunk, VOXEL_SIZE);

    // Create material
    let material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.8, 0.7, 0.6),
        perceptual_roughness: 0.8,
        ..default()
    });

    // Spawn the chunk mesh
    commands.spawn((
        Mesh3d(meshes.add(mesh)),
        MeshMaterial3d(material),
        Transform::from_translation(Vec3::ZERO),
    ));

    info!("Basic chunk example loaded!");
    info!("Pyramid structure: {} voxels per chunk", CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE);
    info!("Voxel size: {}m", VOXEL_SIZE);
    info!("Chunk world size: {}m", CHUNK_SIZE as f32 * VOXEL_SIZE);
}

fn main() {
    let mut app = engine::create_app();
    app.add_plugins((BrpExtrasPlugin, FlyCameraPlugin));
    app.add_systems(Startup, setup_scene);
    app.run();
}
