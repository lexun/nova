//! # Test: Complex Chunk Meshing
//!
//! Tests greedy meshing with various voxel patterns to verify robustness:
//! - Hollow structures
//! - Partial fills
//! - Scattered voxels
//! - Layered structures
//! - Mixed densities
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

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Camera positioned to view all test chunks
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(40.0, 30.0, 40.0).looking_at(Vec3::new(16.0, 0.0, 0.0), Vec3::Y),
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

    let material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.3, 0.7, 0.9),
        perceptual_roughness: 0.85,
        ..default()
    });

    let chunk_world_size = CHUNK_SIZE as f32 * VOXEL_SIZE;
    let spacing = chunk_world_size + 2.0; // 2m gap between chunks

    info!("Generating test chunks with various patterns...");

    // Test 1: Solid cube (baseline)
    let chunk = create_solid_chunk();
    spawn_chunk(&mut commands, &mut meshes, &material, chunk,
                 Vec3::new(0.0, 0.0, 0.0), "Solid");

    // Test 2: Hollow box
    let chunk = create_hollow_box();
    spawn_chunk(&mut commands, &mut meshes, &material, chunk,
                 Vec3::new(spacing, 0.0, 0.0), "Hollow Box");

    // Test 3: Sphere
    let chunk = create_sphere();
    spawn_chunk(&mut commands, &mut meshes, &material, chunk,
                 Vec3::new(spacing * 2.0, 0.0, 0.0), "Sphere");

    // Test 4: Stairs
    let chunk = create_stairs();
    spawn_chunk(&mut commands, &mut meshes, &material, chunk,
                 Vec3::new(spacing * 3.0, 0.0, 0.0), "Stairs");

    // Test 5: Checkerboard pattern
    let chunk = create_checkerboard();
    spawn_chunk(&mut commands, &mut meshes, &material, chunk,
                 Vec3::new(0.0, 0.0, spacing), "Checkerboard");

    // Test 6: Scattered voxels
    let chunk = create_scattered();
    spawn_chunk(&mut commands, &mut meshes, &material, chunk,
                 Vec3::new(spacing, 0.0, spacing), "Scattered");

    // Test 7: Layers
    let chunk = create_layers();
    spawn_chunk(&mut commands, &mut meshes, &material, chunk,
                 Vec3::new(spacing * 2.0, 0.0, spacing), "Layers");

    // Test 8: Random terrain-like
    let chunk = create_terrain();
    spawn_chunk(&mut commands, &mut meshes, &material, chunk,
                 Vec3::new(spacing * 3.0, 0.0, spacing), "Terrain");

    info!("Test chunk generation complete!");
}

fn spawn_chunk(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    material: &Handle<StandardMaterial>,
    chunk: Chunk,
    position: Vec3,
    label: &str,
) {
    let mesh = voxel::meshing::generate_chunk_mesh(&chunk, VOXEL_SIZE);

    commands.spawn((
        Mesh3d(meshes.add(mesh)),
        MeshMaterial3d(material.clone()),
        Transform::from_translation(position),
        Wireframe,
    ));

    info!("Spawned '{}' chunk at {:?}", label, position);
}

// Test pattern generators

fn create_solid_chunk() -> Chunk {
    let mut chunk = Chunk::new();
    for x in 0..CHUNK_SIZE {
        for y in 0..CHUNK_SIZE {
            for z in 0..CHUNK_SIZE {
                chunk.set_voxel(x, y, z, Voxel {
                    voxel_type: VoxelType::Grass,
                    density: 255,
                });
            }
        }
    }
    chunk
}

fn create_hollow_box() -> Chunk {
    let mut chunk = Chunk::new();
    for x in 0..CHUNK_SIZE {
        for y in 0..CHUNK_SIZE {
            for z in 0..CHUNK_SIZE {
                // Only fill outer shell
                if x == 0 || x == CHUNK_SIZE - 1 ||
                   y == 0 || y == CHUNK_SIZE - 1 ||
                   z == 0 || z == CHUNK_SIZE - 1 {
                    chunk.set_voxel(x, y, z, Voxel {
                        voxel_type: VoxelType::Grass,
                        density: 255,
                    });
                }
            }
        }
    }
    chunk
}

fn create_sphere() -> Chunk {
    let mut chunk = Chunk::new();
    let center = CHUNK_SIZE as f32 / 2.0;
    let radius = CHUNK_SIZE as f32 / 2.5;

    for x in 0..CHUNK_SIZE {
        for y in 0..CHUNK_SIZE {
            for z in 0..CHUNK_SIZE {
                let dx = x as f32 - center;
                let dy = y as f32 - center;
                let dz = z as f32 - center;
                let distance = (dx * dx + dy * dy + dz * dz).sqrt();

                if distance < radius {
                    chunk.set_voxel(x, y, z, Voxel {
                        voxel_type: VoxelType::Grass,
                        density: 255,
                    });
                }
            }
        }
    }
    chunk
}

fn create_stairs() -> Chunk {
    let mut chunk = Chunk::new();
    let steps = 8;
    let step_height = CHUNK_SIZE / steps;

    for x in 0..CHUNK_SIZE {
        for y in 0..CHUNK_SIZE {
            for z in 0..CHUNK_SIZE {
                let step = x / (CHUNK_SIZE / steps);
                let max_height = step * step_height;

                if y <= max_height {
                    chunk.set_voxel(x, y, z, Voxel {
                        voxel_type: VoxelType::Grass,
                        density: 255,
                    });
                }
            }
        }
    }
    chunk
}

fn create_checkerboard() -> Chunk {
    let mut chunk = Chunk::new();
    let cell_size = 4;

    for x in 0..CHUNK_SIZE {
        for y in 0..CHUNK_SIZE {
            for z in 0..CHUNK_SIZE {
                let cell_x = x / cell_size;
                let cell_y = y / cell_size;
                let cell_z = z / cell_size;

                if (cell_x + cell_y + cell_z) % 2 == 0 {
                    chunk.set_voxel(x, y, z, Voxel {
                        voxel_type: VoxelType::Grass,
                        density: 255,
                    });
                }
            }
        }
    }
    chunk
}

fn create_scattered() -> Chunk {
    let mut chunk = Chunk::new();

    // Simple pseudo-random pattern
    for x in 0..CHUNK_SIZE {
        for y in 0..CHUNK_SIZE {
            for z in 0..CHUNK_SIZE {
                let hash = (x * 73 + y * 151 + z * 283) % 100;
                if hash < 20 {
                    chunk.set_voxel(x, y, z, Voxel {
                        voxel_type: VoxelType::Grass,
                        density: 255,
                    });
                }
            }
        }
    }
    chunk
}

fn create_layers() -> Chunk {
    let mut chunk = Chunk::new();

    for x in 0..CHUNK_SIZE {
        for y in 0..CHUNK_SIZE {
            for z in 0..CHUNK_SIZE {
                // Fill every other layer
                if (y / 4) % 2 == 0 {
                    chunk.set_voxel(x, y, z, Voxel {
                        voxel_type: VoxelType::Grass,
                        density: 255,
                    });
                }
            }
        }
    }
    chunk
}

fn create_terrain() -> Chunk {
    let mut chunk = Chunk::new();

    for x in 0..CHUNK_SIZE {
        for z in 0..CHUNK_SIZE {
            // Simple height map using sine waves
            let height = ((x as f32 * 0.3).sin() + (z as f32 * 0.3).cos()) * 4.0 + 12.0;
            let height = height.max(0.0).min(CHUNK_SIZE as f32 - 1.0) as usize;

            for y in 0..=height {
                chunk.set_voxel(x, y, z, Voxel {
                    voxel_type: VoxelType::Grass,
                    density: 255,
                });
            }
        }
    }
    chunk
}

fn main() {
    let mut app = engine::create_app();
    app.add_plugins((BrpExtrasPlugin, WireframePlugin::default(), FlyCameraPlugin));
    app.add_systems(Startup, setup_scene);
    app.run();
}
