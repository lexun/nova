//! Grass with Separate Textures Test
//!
//! Tests the new separate textures approach (no atlas):
//! - Each material gets its own texture and mesh
//! - Textures can tile freely without atlas limitations
//! - Tests 2×1×1 grass voxels side-by-side
//!
//! Press 1-6 to view each face directly

use bevy::prelude::*;
use bevy_brp_extras::BrpExtrasPlugin;
use engine::{FlyCameraController, FlyCameraPlugin};
use voxel::{Chunk, Voxel, VoxelType, CHUNK_SIZE};
use std::collections::HashMap;

#[derive(Component)]
struct DebugCamera;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((BrpExtrasPlugin, FlyCameraPlugin))
        .add_systems(Startup, setup)
        .add_systems(Update, camera_control)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut images: ResMut<Assets<Image>>,
) {
    println!("\n=== Grass Separate Textures Test (2×1×1) ===");
    println!("Testing separate textures (no atlas) with 2 grass voxels");
    println!("\nControls:");
    println!("  1 = X+ face (right side)");
    println!("  2 = X- face (left side)");
    println!("  3 = Y+ face (top)");
    println!("  4 = Y- face (bottom)");
    println!("  5 = Z+ face (front)");
    println!("  6 = Z- face (back)");
    println!("\nExpected:");
    println!("  - Top: 2×1 merged green grass quad (2 texture tiles)");
    println!("  - Bottom: 2×1 merged brown dirt quad (2 texture tiles)");
    println!("  - Z+ and Z-: 2×1 merged grass-side quads (2 texture tiles)");
    println!("  - X+ and X-: 1×1 grass-side quads (1 texture tile each)");
    println!("  - Internal X face: CULLED (not visible)\n");

    // Generate separate material textures
    let material_textures = voxel::textures::generate_material_textures(&mut images);

    // Create materials for each voxel_type + face_dir combination
    use voxel::atlas::FaceDir;
    use voxel::meshing::MaterialKey;

    let mut material_map = HashMap::new();

    for voxel_type in [VoxelType::Stone, VoxelType::Dirt, VoxelType::Grass] {
        for face_dir in [FaceDir::Top, FaceDir::Bottom, FaceDir::Side] {
            let texture = material_textures.get_texture(voxel_type, face_dir);
            let material = materials.add(StandardMaterial {
                base_color_texture: Some(texture),
                unlit: true,
                ..default()
            });
            material_map.insert(MaterialKey::new(voxel_type, face_dir), material);
        }
    }

    // Create 2×1×1 (2 voxels side-by-side along X axis)
    let chunk = create_side_by_side(2, 1, 1);

    // Generate separate meshes per material
    let meshes_by_material = voxel::meshing::generate_chunk_meshes(&chunk, 1.0);

    println!("Generated {} separate meshes:", meshes_by_material.len());
    for (key, _) in &meshes_by_material {
        println!("  - {:?} {:?}", key.voxel_type, key.face_dir);
    }

    // Spawn one entity per material
    for (material_key, mesh) in meshes_by_material {
        if let Some(material) = material_map.get(&material_key) {
            commands.spawn((
                Mesh3d(meshes.add(mesh)),
                MeshMaterial3d(material.clone()),
                Transform::from_xyz(0.0, 0.0, 0.0),
            ));
        }
    }

    // Structure extends from (0,0,0) to (2,1,1), center at (1.0, 0.5, 0.5)
    let center = Vec3::new(1.0, 0.5, 0.5);
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(1.0, 0.5, 4.0).looking_at(center, Vec3::Y),
        FlyCameraController {
            move_speed: 2.0,
            ..default()
        },
        DebugCamera,
    ));

    commands.spawn((
        DirectionalLight {
            illuminance: 10000.0,
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, -0.5, 0.5, 0.0)),
    ));

    commands.insert_resource(ClearColor(Color::srgb(0.7, 0.85, 0.95)));
}

fn camera_control(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut camera_query: Query<&mut Transform, With<DebugCamera>>,
) {
    if let Ok(mut transform) = camera_query.single_mut() {
        let distance = 3.5;
        let center = Vec3::new(1.0, 0.5, 0.5);

        if keyboard.just_pressed(KeyCode::Digit1) {
            *transform = Transform::from_xyz(distance + 1.0, 0.5, 0.5)
                .looking_at(center, Vec3::Y);
            println!("\n[1] X+ (right end)");
            println!("    Expected: 1×1 grass-side texture (1 tile)");
        } else if keyboard.just_pressed(KeyCode::Digit2) {
            *transform = Transform::from_xyz(-distance + 1.0, 0.5, 0.5)
                .looking_at(center, Vec3::Y);
            println!("\n[2] X- (left end)");
            println!("    Expected: 1×1 grass-side texture (1 tile)");
        } else if keyboard.just_pressed(KeyCode::Digit3) {
            *transform = Transform::from_xyz(1.0, distance + 0.5, 0.5)
                .looking_at(center, Vec3::NEG_Z);
            println!("\n[3] Y+ (top)");
            println!("    Expected: 2×1 merged green grass quad (2 tiles)");
        } else if keyboard.just_pressed(KeyCode::Digit4) {
            *transform = Transform::from_xyz(1.0, -distance + 0.5, 0.5)
                .looking_at(center, Vec3::Z);
            println!("\n[4] Y- (bottom)");
            println!("    Expected: 2×1 merged brown dirt quad (2 tiles)");
        } else if keyboard.just_pressed(KeyCode::Digit5) {
            *transform = Transform::from_xyz(1.0, 0.5, distance + 0.5)
                .looking_at(center, Vec3::Y);
            println!("\n[5] Z+ (front)");
            println!("    Expected: 2×1 merged grass-side quad (2 tiles)");
        } else if keyboard.just_pressed(KeyCode::Digit6) {
            *transform = Transform::from_xyz(1.0, 0.5, -distance + 0.5)
                .looking_at(center, Vec3::Y);
            println!("\n[6] Z- (back)");
            println!("    Expected: 2×1 merged grass-side quad (2 tiles)");
        }
    }
}

fn create_side_by_side(width: usize, height: usize, depth: usize) -> Chunk {
    let mut chunk = Chunk::new();
    for x in 0..width.min(CHUNK_SIZE) {
        for y in 0..height.min(CHUNK_SIZE) {
            for z in 0..depth.min(CHUNK_SIZE) {
                chunk.set_voxel(x, y, z, Voxel {
                    voxel_type: VoxelType::Grass,
                    density: 255,
                });
            }
        }
    }
    chunk
}
