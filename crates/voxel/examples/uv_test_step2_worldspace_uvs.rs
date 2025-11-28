//! UV Test Step 2: World-space UV mapping on simple plane
//!
//! Creates a 1m×1m plane with CUSTOM UVs calculated from world-space position.
//! Uses the same UV formula as voxel meshing: ((world_pos % 0.25) / 0.25)
//!
//! Expected with TEXTURE_WORLD_SIZE = 0.25m:
//! - Should show 4×4 grid of colored squares (16 total)
//! - NOT the full 8×8 grid (64 squares) like Step 1
//!
//! This verifies our world-space UV mapping formula is correct.
//!
//! Run with: cargo run -p voxel --example uv_test_step2_worldspace_uvs --release

use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use bevy::mesh::{Indices, PrimitiveTopology};
use bevy::asset::RenderAssetUsages;
use bevy_brp_extras::BrpExtrasPlugin;
use engine::{FlyCameraController, FlyCameraPlugin};

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
    // Create debug texture: 8×8 grid of red/white squares
    let debug_texture = generate_debug_checkerboard();
    let texture_handle = images.add(debug_texture);

    let material = materials.add(StandardMaterial {
        base_color_texture: Some(texture_handle),
        unlit: true,
        ..default()
    });

    // Create 1m×1m plane with world-space UVs
    let plane_mesh = create_plane_with_worldspace_uvs(1.0, 1.0);

    commands.spawn((
        Mesh3d(meshes.add(plane_mesh)),
        MeshMaterial3d(material.clone()),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    // Camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 0.0, 2.5).looking_at(Vec3::ZERO, Vec3::Y),
        FlyCameraController {
            move_speed: 2.0,
            ..default()
        },
    ));

    // Light
    commands.spawn((
        DirectionalLight {
            illuminance: 10000.0,
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, -0.5, -0.5, 0.0)),
    ));
}

/// Create a plane mesh with world-space UVs
/// Plane is centered at origin, extends from -width/2 to +width/2
fn create_plane_with_worldspace_uvs(width: f32, height: f32) -> Mesh {
    const TEXTURE_WORLD_SIZE: f32 = 0.25; // Same as meshing.rs

    let half_w = width / 2.0;
    let half_h = height / 2.0;

    // Four corners in world space (XY plane facing +Z)
    let positions = vec![
        [-half_w, -half_h, 0.0], // Bottom-left
        [half_w, -half_h, 0.0],  // Bottom-right
        [half_w, half_h, 0.0],   // Top-right
        [-half_w, half_h, 0.0],  // Top-left
    ];

    // Calculate UVs from world-space positions
    // For a 1m×1m plane with TEXTURE_WORLD_SIZE=0.25m, UVs should range 0-4
    // This will tile the texture 4 times in each direction
    let mut uvs = Vec::new();
    for pos in &positions {
        let x = pos[0];
        let y = pos[1];

        // Map world position to UV space
        // x ranges from -0.5 to 0.5 (1m total)
        // We want UV to range from 0 to 4 (1m / 0.25m = 4 tiles)
        let u = x / TEXTURE_WORLD_SIZE + (width / TEXTURE_WORLD_SIZE / 2.0);
        let v = y / TEXTURE_WORLD_SIZE + (height / TEXTURE_WORLD_SIZE / 2.0);

        println!("Position ({:.2}, {:.2}) -> UV ({:.4}, {:.4})", x, y, u, v);
        uvs.push([u, v]);
    }

    let normals = vec![
        [0.0, 0.0, 1.0],
        [0.0, 0.0, 1.0],
        [0.0, 0.0, 1.0],
        [0.0, 0.0, 1.0],
    ];

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::default());
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.insert_indices(Indices::U32(vec![
        0, 1, 2, // First triangle
        0, 2, 3, // Second triangle
    ]));
    mesh
}

/// Generate a highly visible 8×8 checkerboard texture
fn generate_debug_checkerboard() -> Image {
    const SIZE: u32 = 64;
    const SQUARE_SIZE: u32 = 8;

    let mut data = vec![0u8; (SIZE * SIZE * 4) as usize];

    for y in 0..SIZE {
        for x in 0..SIZE {
            let square_x = x / SQUARE_SIZE;
            let square_y = y / SQUARE_SIZE;
            let is_red = (square_x + square_y) % 2 == 0;

            let pixel_index = ((y * SIZE + x) * 4) as usize;
            if is_red {
                data[pixel_index] = 255;
                data[pixel_index + 1] = 0;
                data[pixel_index + 2] = 0;
                data[pixel_index + 3] = 255;
            } else {
                data[pixel_index] = 255;
                data[pixel_index + 1] = 255;
                data[pixel_index + 2] = 255;
                data[pixel_index + 3] = 255;
            }
        }
    }

    let mut image = Image::new(
        Extent3d {
            width: SIZE,
            height: SIZE,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        data,
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::RENDER_WORLD,
    );

    // CRITICAL: Set texture to repeat mode so UVs > 1.0 tile the texture
    use bevy::image::{ImageSamplerDescriptor, ImageAddressMode, ImageSampler};
    image.sampler = ImageSampler::Descriptor(ImageSamplerDescriptor {
        address_mode_u: ImageAddressMode::Repeat,
        address_mode_v: ImageAddressMode::Repeat,
        address_mode_w: ImageAddressMode::Repeat,
        ..default()
    });

    image
}
