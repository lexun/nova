//! UV Test Step 1: Debug texture verification
//!
//! Creates a highly visible 8×8 grid of alternating red/white squares.
//! Each square is 8×8 pixels in a 64×64 texture.
//! This makes it IMPOSSIBLE to miss if UV mapping is stretched or incorrect.
//!
//! Expected: Clear checkerboard pattern with sharp color transitions
//!
//! Run with: cargo run -p voxel --example uv_test_step1_debug_texture --release

use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use bevy::asset::RenderAssetUsages;
use engine::{FlyCameraController, FlyCameraPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(FlyCameraPlugin)
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
        unlit: true, // Unlit so we can see colors clearly
        ..default()
    });

    // Test 1: Single 1m×1m quad plane
    // Should show 4×4 = 16 colored squares (because TEXTURE_WORLD_SIZE = 0.25m)
    let plane_mesh = Mesh::from(Plane3d::default().mesh().size(1.0, 1.0));

    // Rotate plane to face camera (Bevy planes default to XZ, we want XY facing +Z)
    commands.spawn((
        Mesh3d(meshes.add(plane_mesh)),
        MeshMaterial3d(material.clone()),
        Transform::from_xyz(0.0, 0.0, 0.0)
            .with_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2)),
    ));

    // Camera positioned in front of plane
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 0.0, 2.5).looking_at(Vec3::ZERO, Vec3::Y),
        FlyCameraController {
            move_speed: 2.0,
            ..default()
        },
    ));

    // Directional light
    commands.spawn((
        DirectionalLight {
            illuminance: 10000.0,
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, -0.5, -0.5, 0.0)),
    ));
}

/// Generate a highly visible 8×8 checkerboard texture
/// 64×64 pixels total, each square is 8×8 pixels
/// Alternating bright red and white squares - IMPOSSIBLE to miss
fn generate_debug_checkerboard() -> Image {
    const SIZE: u32 = 64;
    const SQUARE_SIZE: u32 = 8; // 64 / 8 = 8 squares per side

    let mut data = vec![0u8; (SIZE * SIZE * 4) as usize];

    for y in 0..SIZE {
        for x in 0..SIZE {
            // Determine which square we're in
            let square_x = x / SQUARE_SIZE;
            let square_y = y / SQUARE_SIZE;

            // Checkerboard pattern: alternate colors
            let is_red = (square_x + square_y) % 2 == 0;

            let pixel_index = ((y * SIZE + x) * 4) as usize;
            if is_red {
                data[pixel_index] = 255;     // R
                data[pixel_index + 1] = 0;   // G
                data[pixel_index + 2] = 0;   // B
                data[pixel_index + 3] = 255; // A
            } else {
                data[pixel_index] = 255;     // R
                data[pixel_index + 1] = 255; // G
                data[pixel_index + 2] = 255; // B
                data[pixel_index + 3] = 255; // A
            }
        }
    }

    Image::new(
        Extent3d {
            width: SIZE,
            height: SIZE,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        data,
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::RENDER_WORLD,
    )
}
