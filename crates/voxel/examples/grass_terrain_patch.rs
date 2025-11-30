//! Grass Terrain Patch
//!
//! Tests grass rendering on a larger terrain with varied heights
//! Creates rolling hills to verify:
//! - Top layer voxels are Grass (green top, grass-side on sides)
//! - Underground voxels are Dirt (brown on all faces)
//! - Proper material separation and meshing
//! - Texture tiling across greedy-meshed quads

use bevy::prelude::*;
use bevy_brp_extras::BrpExtrasPlugin;
use engine::{FlyCameraController, FlyCameraPlugin};
use voxel::{Chunk, Voxel, VoxelType, CHUNK_SIZE};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((BrpExtrasPlugin, FlyCameraPlugin))
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut images: ResMut<Assets<Image>>,
) {
    println!("\n=== Grass Terrain Patch ===");
    println!("16×16 terrain with rolling hills (height varies 1-5 voxels)");
    println!("\nControls:");
    println!("  Click to capture mouse");
    println!("  Escape to release mouse");
    println!("  WASD: Move horizontally");
    println!("  QE: Move up/down");
    println!("  Mouse: Look around\n");

    // Generate separate material textures
    use std::collections::HashMap;
    use voxel::FaceDir;
    use voxel::meshing::MaterialKey;

    let material_textures = voxel::textures::generate_material_textures(&mut images);

    let mut material_map = HashMap::new();
    for voxel_type in [VoxelType::Stone, VoxelType::Dirt, VoxelType::Grass] {
        for face_dir in [FaceDir::Top, FaceDir::Bottom, FaceDir::Side] {
            let texture = material_textures.get_texture(voxel_type, face_dir);
            let material = materials.add(StandardMaterial {
                base_color_texture: Some(texture),
                unlit: false, // Use lighting for better depth perception
                ..default()
            });
            material_map.insert(MaterialKey::new(voxel_type, face_dir), material);
        }
    }

    // Create terrain with rolling hills
    let chunk = create_terrain_patch(16, 16);
    let meshes_by_material = voxel::meshing::generate_chunk_meshes(&chunk, 1.0);

    let mut total_vertices = 0;
    for (material_key, mesh) in meshes_by_material {
        total_vertices += mesh.count_vertices();
        if let Some(material) = material_map.get(&material_key) {
            commands.spawn((
                Mesh3d(meshes.add(mesh)),
                MeshMaterial3d(material.clone()),
                Transform::from_xyz(0.0, 0.0, 0.0),
            ));
        }
    }

    println!("Mesh stats:");
    println!("  Total vertices: {}", total_vertices);
    println!("  Triangles: ~{}\n", total_vertices / 3);

    // Camera positioned to see the whole terrain
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(8.0, 12.0, 20.0)
            .looking_at(Vec3::new(8.0, 2.0, 8.0), Vec3::Y),
        FlyCameraController {
            move_speed: 8.0,
            sensitivity: 0.003,
            ..default()
        },
    ));

    // Directional light (sun)
    commands.spawn((
        DirectionalLight {
            illuminance: 10000.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, -0.8, 0.4, 0.0)),
    ));

    // Ambient light
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 150.0,
        ..default()
    });

    commands.insert_resource(ClearColor(Color::srgb(0.53, 0.81, 0.92))); // Sky blue
}

/// Create a 16×16 terrain patch with rolling hills
fn create_terrain_patch(width: usize, depth: usize) -> Chunk {
    let mut chunk = Chunk::new();

    // Generate height map with rolling hills
    for x in 0..width.min(CHUNK_SIZE) {
        for z in 0..depth.min(CHUNK_SIZE) {
            // Simple procedural height using sine waves
            let fx = x as f32;
            let fz = z as f32;

            let height = 1.0
                + 2.0 * ((fx * 0.3).sin() * (fz * 0.3).cos())
                + 1.5 * ((fx * 0.15 + fz * 0.2).sin());

            let height_voxels = height.max(1.0).min(8.0) as usize;

            // Fill column: Dirt underground, Grass on top layer only
            for y in 0..height_voxels.min(CHUNK_SIZE) {
                let voxel_type = if y == height_voxels - 1 {
                    VoxelType::Grass  // Top layer has grass
                } else {
                    VoxelType::Dirt   // Everything below is dirt
                };
                chunk.set_voxel(x, y, z, Voxel {
                    voxel_type,
                    density: 255,
                });
            }
        }
    }

    chunk
}
