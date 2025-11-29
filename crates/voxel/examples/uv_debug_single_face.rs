//! UV Debug: Single cube with face-by-face inspection
//!
//! Press 1-6 to view each face directly:
//! 1 = X+ (right), 2 = X- (left)
//! 3 = Y+ (top), 4 = Y- (bottom)
//! 5 = Z+ (front), 6 = Z- (back)
//!
//! Or use fly camera controls:
//! - Click to capture mouse
//! - Escape to release mouse
//! - WASD: Move horizontally
//! - QE: Move up/down
//! - Mouse: Look around

use bevy::prelude::*;
use bevy::asset::RenderAssetUsages;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use bevy::image::{ImageAddressMode, ImageSampler, ImageSamplerDescriptor};
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
    println!("\n=== UV Debug: Single Face Inspection ===");
    println!("Texture: Letter 'F' - should always appear upright");
    println!("\nControls:");
    println!("  Number keys 1-6: Jump to face view");
    println!("    1 = X+ face (right side)");
    println!("    2 = X- face (left side)");
    println!("    3 = Y+ face (top)");
    println!("    4 = Y- face (bottom)");
    println!("    5 = Z+ face (front) - INITIAL VIEW");
    println!("    6 = Z- face (back)");
    println!("\n  Fly camera: WASD + QE + Mouse");
    println!("\nExpected: 'F' should be upright on ALL faces\n");

    let texture = create_f_texture();
    let texture_handle = images.add(texture);

    let material = materials.add(StandardMaterial {
        base_color_texture: Some(texture_handle),
        unlit: true,
        ..default()
    });

    // Single 1×1×1 cube at origin
    let chunk = create_cube(1, 1, 1);
    let mesh = voxel::meshing::generate_chunk_mesh(&chunk, 1.0);
    commands.spawn((
        Mesh3d(meshes.add(mesh)),
        MeshMaterial3d(material.clone()),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));

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

/// Create texture showing letter "F"
/// F should appear upright (not rotated, flipped, or mirrored)
fn create_f_texture() -> Image {
    let size = 64u32;
    let mut data = vec![0u8; (size * size * 4) as usize];

    // Fill with dark blue background
    for i in 0..(size * size) {
        let idx = (i * 4) as usize;
        data[idx] = 20;      // R
        data[idx + 1] = 30;  // G
        data[idx + 2] = 60;  // B
        data[idx + 3] = 255; // A
    }

    // Draw letter "F" in white
    // Each cell is 8×8 pixels, we have an 8×8 grid
    // F pattern in the grid:
    // X X X X X X X .
    // X X . . . . . .
    // X X . . . . . .
    // X X X X X . . .
    // X X . . . . . .
    // X X . . . . . .
    // X X . . . . . .
    // . . . . . . . .

    let f_pattern = [
        [1,1,1,1,1,1,1,0],
        [1,1,0,0,0,0,0,0],
        [1,1,0,0,0,0,0,0],
        [1,1,1,1,1,0,0,0],
        [1,1,0,0,0,0,0,0],
        [1,1,0,0,0,0,0,0],
        [1,1,0,0,0,0,0,0],
        [0,0,0,0,0,0,0,0],
    ];

    let cell_size = 8u32;
    for grid_y in 0..8 {
        for grid_x in 0..8 {
            if f_pattern[grid_y][grid_x] == 1 {
                // Fill this 8×8 cell with white
                for py in 0..cell_size {
                    for px in 0..cell_size {
                        let img_x = grid_x as u32 * cell_size + px;
                        let img_y = grid_y as u32 * cell_size + py;
                        let idx = ((img_y * size + img_x) * 4) as usize;
                        data[idx] = 255;     // R
                        data[idx + 1] = 255; // G
                        data[idx + 2] = 255; // B
                    }
                }
            }
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

fn camera_control(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut camera_query: Query<&mut Transform, With<DebugCamera>>,
) {
    if let Ok(mut transform) = camera_query.single_mut() {
        let distance = 3.0;
        let cube_center = Vec3::new(0.5, 0.5, 0.5);

        if keyboard.just_pressed(KeyCode::Digit1) {
            // X+ face (right side) - camera looks from +X toward cube center
            *transform = Transform::from_xyz(distance + 0.5, 0.5, 0.5)
                .looking_at(cube_center, Vec3::Y);
            println!("\n[1] Viewing X+ face (right)");
            println!("    Camera: ({}, 0, 0) looking at origin, up=Y", distance);
            println!("    Expected: F upright");
        } else if keyboard.just_pressed(KeyCode::Digit2) {
            // X- face (left side) - camera looks from -X toward cube center
            *transform = Transform::from_xyz(-distance + 0.5, 0.5, 0.5)
                .looking_at(cube_center, Vec3::Y);
            println!("\n[2] Viewing X- face (left)");
            println!("    Camera: ({}, 0.5, 0.5) looking at cube center, up=Y", -distance + 0.5);
            println!("    Expected: F upright");
        } else if keyboard.just_pressed(KeyCode::Digit3) {
            // Y+ face (top) - camera looks from +Y down toward cube center
            // Up vector points north (-Z) so text is readable from north
            *transform = Transform::from_xyz(0.5, distance + 0.5, 0.5)
                .looking_at(cube_center, Vec3::NEG_Z);
            println!("\n[3] Viewing Y+ face (top)");
            println!("    Camera: (0.5, {}, 0.5) looking at cube center, up=-Z (north)", distance + 0.5);
            println!("    Expected: F upright when viewed from north");
        } else if keyboard.just_pressed(KeyCode::Digit4) {
            // Y- face (bottom) - camera looks from -Y up toward cube center
            // Up vector points north (+Z from below) so text is readable from north
            *transform = Transform::from_xyz(0.5, -distance + 0.5, 0.5)
                .looking_at(cube_center, Vec3::Z);
            println!("\n[4] Viewing Y- face (bottom)");
            println!("    Camera: (0.5, {}, 0.5) looking at cube center, up=+Z (north from below)", -distance + 0.5);
            println!("    Expected: F upright when viewed from north");
        } else if keyboard.just_pressed(KeyCode::Digit5) {
            // Z+ face (front) - camera looks from +Z toward cube center
            *transform = Transform::from_xyz(0.5, 0.5, distance + 0.5)
                .looking_at(cube_center, Vec3::Y);
            println!("\n[5] Viewing Z+ face (front)");
            println!("    Camera: (0.5, 0.5, {}) looking at cube center, up=Y", distance + 0.5);
            println!("    Expected: F upright");
        } else if keyboard.just_pressed(KeyCode::Digit6) {
            // Z- face (back) - camera looks from -Z toward cube center
            *transform = Transform::from_xyz(0.5, 0.5, -distance + 0.5)
                .looking_at(cube_center, Vec3::Y);
            println!("\n[6] Viewing Z- face (back)");
            println!("    Camera: (0.5, 0.5, {}) looking at cube center, up=Y", -distance + 0.5);
            println!("    Expected: F upright");
        }
    }
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
