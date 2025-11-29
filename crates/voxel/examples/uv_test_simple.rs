//! UV Test: Verify basic tiling with 2×2 red/white texture
//!
//! This test uses a simple procedurally generated 2×2 checkerboard texture
//! (red and white squares) to verify that UV tiling works correctly.
//!
//! Expected results:
//! - 1×1×1 cube: Each face shows 2×2 squares (1 texture tile per voxel face)
//! - 2×2×2 cube: Each face shows 4×4 squares (2×2 texture tiles)
//! - 4×1×1 bar: Top face shows 8×2 squares (4×1 texture tiles)
//!
//! Current status:
//! - ✓ Texture tiling works correctly with Repeat mode
//! - ✓ Greedy meshing preserves correct UV scaling
//! - ⚠ UV orientation differs between faces (some are mirrored)
//!   This is acceptable for symmetric textures but would be visible
//!   with directional textures (arrows, text, etc.)

use bevy::prelude::*;
use bevy::asset::RenderAssetUsages;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use bevy::image::{ImageAddressMode, ImageSampler, ImageSamplerDescriptor};
use voxel::{Chunk, Voxel, VoxelType, CHUNK_SIZE};
use engine::{FlyCameraController, FlyCameraPlugin};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            bevy_brp_extras::BrpExtrasPlugin,
            FlyCameraPlugin,
        ))
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut images: ResMut<Assets<Image>>,
) {
    println!("=== UV Test Setup ===");
    println!("Texture: 64×64 with 2×2 red/white checkerboard (each square = 32×32px)");
    println!("Expected: 1 voxel face shows 2×2 squares (1 texture tile)");
    
    let texture = create_debug_texture();
    let texture_handle = images.add(texture);

    let material = materials.add(StandardMaterial {
        base_color_texture: Some(texture_handle),
        unlit: true,
        ..default()
    });

    // Test 1: 1×1×1 - expect 2×2 squares per face
    let chunk1 = create_cube(1, 1, 1);
    let mesh1 = voxel::meshing::generate_chunk_mesh(&chunk1, 0.25);
    commands.spawn((
        Mesh3d(meshes.add(mesh1)),
        MeshMaterial3d(material.clone()),
        Transform::from_xyz(-1.0, 0.0, 0.0),
    ));
    println!("Shape 1: 1×1×1 cube at x=-1.0");

    // Test 2: 2×2×2 - expect 4×4 squares per face
    let chunk2 = create_cube(2, 2, 2);
    let mesh2 = voxel::meshing::generate_chunk_mesh(&chunk2, 0.25);
    commands.spawn((
        Mesh3d(meshes.add(mesh2)),
        MeshMaterial3d(material.clone()),
        Transform::from_xyz(0.5, 0.0, 0.0),
    ));
    println!("Shape 2: 2×2×2 cube at x=0.5");

    // Test 3: 4×1×1 - expect 8×2 squares on long face
    let chunk3 = create_cube(4, 1, 1);
    let mesh3 = voxel::meshing::generate_chunk_mesh(&chunk3, 0.25);
    commands.spawn((
        Mesh3d(meshes.add(mesh3)),
        MeshMaterial3d(material.clone()),
        Transform::from_xyz(2.5, 0.0, 0.0),
    ));
    println!("Shape 3: 4×1×1 bar at x=2.5");

    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(1.0, 2.0, 3.0).looking_at(Vec3::new(0.5, 0.0, 0.0), Vec3::Y),
        FlyCameraController {
            move_speed: 2.0,
            ..default()
        },
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

fn create_debug_texture() -> Image {
    let size = 64u32;
    let mut data = vec![0u8; (size * size * 4) as usize];

    for y in 0..size {
        for x in 0..size {
            let cell_x = x / 32;
            let cell_y = y / 32;
            let is_white = (cell_x + cell_y) % 2 == 0;

            let idx = ((y * size + x) * 4) as usize;
            if is_white {
                data[idx] = 255; data[idx + 1] = 255; data[idx + 2] = 255;
            } else {
                data[idx] = 255; data[idx + 1] = 0; data[idx + 2] = 0;
            }
            data[idx + 3] = 255;
        }
    }

    let mut image = Image::new(
        Extent3d { width: size, height: size, depth_or_array_layers: 1 },
        TextureDimension::D2,
        data,
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::RENDER_WORLD,
    );

    image.sampler = ImageSampler::Descriptor(ImageSamplerDescriptor {
        address_mode_u: ImageAddressMode::Repeat,
        address_mode_v: ImageAddressMode::Repeat,
        ..default()
    });

    image
}

fn create_cube(width: usize, height: usize, depth: usize) -> Chunk {
    let mut chunk = Chunk::new();
    for x in 0..width.min(CHUNK_SIZE) {
        for y in 0..height.min(CHUNK_SIZE) {
            for z in 0..depth.min(CHUNK_SIZE) {
                chunk.set_voxel(x, y, z, Voxel {
                    voxel_type: VoxelType::Stone,
                    density: 255,
                });
            }
        }
    }
    chunk
}
