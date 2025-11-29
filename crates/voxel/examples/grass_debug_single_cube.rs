//! Grass Debug: Single grass cube with face-by-face inspection
//!
//! Press 1-6 to view each face directly:
//! 1 = X+ (right), 2 = X- (left)
//! 3 = Y+ (top), 4 = Y- (bottom)
//! 5 = Z+ (front), 6 = Z- (back)
//!
//! Expected textures:
//! - Top (Y+): Green grass texture
//! - Bottom (Y-): Brown dirt texture
//! - All sides (X+, X-, Z+, Z-): Dirt with grass transition in top quarter
//!
//! Or use fly camera controls:
//! - Click to capture mouse
//! - Escape to release mouse
//! - WASD: Move horizontally
//! - QE: Move up/down
//! - Mouse: Look around

use bevy::prelude::*;
use bevy_brp_extras::BrpExtrasPlugin;
use engine::{FlyCameraController, FlyCameraPlugin};
use voxel::{Chunk, Voxel, VoxelType, CHUNK_SIZE};

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
    println!("\n=== Grass Debug: Single Cube Face Inspection ===");
    println!("Testing Minecraft-style grass block textures");
    println!("\nControls:");
    println!("  Number keys 1-6: Jump to face view");
    println!("    1 = X+ face (right side)");
    println!("    2 = X- face (left side)");
    println!("    3 = Y+ face (top)");
    println!("    4 = Y- face (bottom)");
    println!("    5 = Z+ face (front) - INITIAL VIEW");
    println!("    6 = Z- face (back)");
    println!("\n  Fly camera: WASD + QE + Mouse");
    println!("\nExpected textures:");
    println!("  - Top (Y+): Green grass");
    println!("  - Bottom (Y-): Brown dirt");
    println!("  - Sides (X+, X-, Z+, Z-): Dirt with grass in top quarter\n");

    // Generate separate material textures
    use std::collections::HashMap;
    use voxel::atlas::FaceDir;
    use voxel::meshing::MaterialKey;

    let material_textures = voxel::textures::generate_material_textures(&mut images);

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

    // Single 1×1×1 grass cube at origin
    let chunk = create_grass_cube(1, 1, 1);
    let meshes_by_material = voxel::meshing::generate_chunk_meshes(&chunk, 1.0);

    for (material_key, mesh) in meshes_by_material {
        if let Some(material) = material_map.get(&material_key) {
            commands.spawn((
                Mesh3d(meshes.add(mesh)),
                MeshMaterial3d(material.clone()),
                Transform::from_xyz(0.0, 0.0, 0.0),
            ));
        }
    }

    // Camera starts viewing Z+ face (front)
    // Cube center is at (0.5, 0.5, 0.5) for a 1×1×1 voxel cube
    let cube_center = Vec3::new(0.5, 0.5, 0.5);
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.5, 0.5, 3.5).looking_at(cube_center, Vec3::Y),
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
        let distance = 3.0;
        let cube_center = Vec3::new(0.5, 0.5, 0.5);

        if keyboard.just_pressed(KeyCode::Digit1) {
            // X+ face (right side)
            *transform = Transform::from_xyz(distance + 0.5, 0.5, 0.5)
                .looking_at(cube_center, Vec3::Y);
            println!("\n[1] Viewing X+ face (right)");
            println!("    Expected: Dirt with grass in top quarter");
        } else if keyboard.just_pressed(KeyCode::Digit2) {
            // X- face (left side)
            *transform = Transform::from_xyz(-distance + 0.5, 0.5, 0.5)
                .looking_at(cube_center, Vec3::Y);
            println!("\n[2] Viewing X- face (left)");
            println!("    Expected: Dirt with grass in top quarter");
        } else if keyboard.just_pressed(KeyCode::Digit3) {
            // Y+ face (top)
            *transform = Transform::from_xyz(0.5, distance + 0.5, 0.5)
                .looking_at(cube_center, Vec3::NEG_Z);
            println!("\n[3] Viewing Y+ face (top)");
            println!("    Expected: Green grass texture");
        } else if keyboard.just_pressed(KeyCode::Digit4) {
            // Y- face (bottom)
            *transform = Transform::from_xyz(0.5, -distance + 0.5, 0.5)
                .looking_at(cube_center, Vec3::Z);
            println!("\n[4] Viewing Y- face (bottom)");
            println!("    Expected: Brown dirt texture");
        } else if keyboard.just_pressed(KeyCode::Digit5) {
            // Z+ face (front)
            *transform = Transform::from_xyz(0.5, 0.5, distance + 0.5)
                .looking_at(cube_center, Vec3::Y);
            println!("\n[5] Viewing Z+ face (front)");
            println!("    Expected: Dirt with grass in top quarter");
        } else if keyboard.just_pressed(KeyCode::Digit6) {
            // Z- face (back)
            *transform = Transform::from_xyz(0.5, 0.5, -distance + 0.5)
                .looking_at(cube_center, Vec3::Y);
            println!("\n[6] Viewing Z- face (back)");
            println!("    Expected: Dirt with grass in top quarter");
        }
    }
}

fn create_grass_cube(width: usize, height: usize, depth: usize) -> Chunk {
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
