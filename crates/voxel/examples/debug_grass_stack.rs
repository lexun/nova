//! Grass Debug: Stacked grass cubes
//!
//! Tests that stacked grass voxels correctly show:
//! - Only the TOP voxel shows grass on its top face
//! - The BOTTOM voxel's top face should NOT be visible (culled by greedy meshing)
//! - Side faces should show grass transition correctly
//!
//! Press 1-6 to view each face directly:
//! 1 = X+ (right), 2 = X- (left)
//! 3 = Y+ (top), 4 = Y- (bottom)
//! 5 = Z+ (front), 6 = Z- (back)

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
    println!("\n=== Grass Debug: Stacked Cubes ===");
    println!("Testing 2 grass voxels stacked vertically (1×2×1 tower)");
    println!("\nControls:");
    println!("  Number keys 1-6: Jump to face view");
    println!("    1 = X+ face (right side)");
    println!("    2 = X- face (left side)");
    println!("    3 = Y+ face (top)");
    println!("    4 = Y- face (bottom)");
    println!("    5 = Z+ face (front) - INITIAL VIEW");
    println!("    6 = Z- face (back)");
    println!("\n  Fly camera: WASD + QE + Mouse");
    println!("\nExpected behavior:");
    println!("  - Top voxel (Grass): Green grass on top, grass-side on sides");
    println!("  - Bottom voxel (Dirt): Brown dirt on bottom, brown dirt on sides");
    println!("  - Internal face between voxels: SHOULD NOT BE VISIBLE (culled)");
    println!("  - This tests realistic stacking: only exposed dirt grows grass\n");

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
                unlit: true,
                ..default()
            });
            material_map.insert(MaterialKey::new(voxel_type, face_dir), material);
        }
    }

    // Create 1×2×1 tower (2 grass voxels stacked vertically)
    let chunk = create_grass_tower(1, 2, 1);
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
    // Tower extends from (0,0,0) to (1,2,1), center at (0.5, 1.0, 0.5)
    let tower_center = Vec3::new(0.5, 1.0, 0.5);
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.5, 1.0, 4.0).looking_at(tower_center, Vec3::Y),
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
        let distance = 4.0;
        let tower_center = Vec3::new(0.5, 1.0, 0.5);

        if keyboard.just_pressed(KeyCode::Digit1) {
            // X+ face (right side)
            *transform = Transform::from_xyz(distance + 0.5, 1.0, 0.5)
                .looking_at(tower_center, Vec3::Y);
            println!("\n[1] Viewing X+ face (right)");
            println!("    Expected: Top half = grass-side, Bottom half = dirt");
        } else if keyboard.just_pressed(KeyCode::Digit2) {
            // X- face (left side)
            *transform = Transform::from_xyz(-distance + 0.5, 1.0, 0.5)
                .looking_at(tower_center, Vec3::Y);
            println!("\n[2] Viewing X- face (left)");
            println!("    Expected: Top half = grass-side, Bottom half = dirt");
        } else if keyboard.just_pressed(KeyCode::Digit3) {
            // Y+ face (top)
            *transform = Transform::from_xyz(0.5, distance + 1.0, 0.5)
                .looking_at(tower_center, Vec3::NEG_Z);
            println!("\n[3] Viewing Y+ face (top)");
            println!("    Expected: Green grass texture (from TOP voxel only)");
        } else if keyboard.just_pressed(KeyCode::Digit4) {
            // Y- face (bottom)
            *transform = Transform::from_xyz(0.5, -distance + 1.0, 0.5)
                .looking_at(tower_center, Vec3::Z);
            println!("\n[4] Viewing Y- face (bottom)");
            println!("    Expected: Brown dirt texture (from BOTTOM voxel only)");
        } else if keyboard.just_pressed(KeyCode::Digit5) {
            // Z+ face (front)
            *transform = Transform::from_xyz(0.5, 1.0, distance + 0.5)
                .looking_at(tower_center, Vec3::Y);
            println!("\n[5] Viewing Z+ face (front)");
            println!("    Expected: 2 vertically stacked grass-side textures");
        } else if keyboard.just_pressed(KeyCode::Digit6) {
            // Z- face (back)
            *transform = Transform::from_xyz(0.5, 1.0, -distance + 0.5)
                .looking_at(tower_center, Vec3::Y);
            println!("\n[6] Viewing Z- face (back)");
            println!("    Expected: 2 vertically stacked grass-side textures");
        }
    }
}

fn create_grass_tower(width: usize, height: usize, depth: usize) -> Chunk {
    let mut chunk = Chunk::new();
    for x in 0..width.min(CHUNK_SIZE) {
        for y in 0..height.min(CHUNK_SIZE) {
            for z in 0..depth.min(CHUNK_SIZE) {
                // Bottom voxel (y=0) is Dirt, top voxel (y=1) is Grass
                let voxel_type = if y == height - 1 {
                    VoxelType::Grass  // Top voxel has grass
                } else {
                    VoxelType::Dirt   // Bottom voxel is just dirt
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
