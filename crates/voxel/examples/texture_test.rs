//! Texture mapping test - expose UV stretching and mapping issues
//!
//! This example creates simple geometric shapes to test how the greedy meshing
//! algorithm handles texture mapping. Look for:
//!
//! 1. **Texture stretching** - Do quads with different aspect ratios stretch the texture?
//! 2. **UV seams** - Do merged quads maintain correct UV coordinates?
//! 3. **Orientation** - Do vertical/horizontal/depth faces map textures consistently?
//! 4. **Greedy merging artifacts** - Does the meshing create visual discontinuities?
//!
//! The scene contains:
//! - Single stone cube (8x8x8 voxels) - uniform material
//! - Tall stone wall (16x2x8 voxels) - wide horizontal merging
//! - Thin stone pillar (2x16x2 voxels) - tall vertical merging
//! - Stone stairs (step pattern) - mixed face sizes
//!
//! Controls:
//! - Click to capture mouse
//! - Escape to release mouse
//! - WASD: Move horizontally
//! - QE: Move up/down
//! - Mouse: Look around
//!
//! Run with: cargo run -p voxel --example texture_test

use bevy::prelude::*;
use bevy::image::ImageSampler;
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
    // Generate simple grid texture (not an atlas)
    let grid_image = generate_grid_texture();
    let grid_texture = images.add(grid_image);

    let material = materials.add(StandardMaterial {
        base_color_texture: Some(grid_texture),
        perceptual_roughness: 0.8,
        ..default()
    });

    // Cube: 8x8x8 stone voxels - tests uniform merging
    let cube_chunk = create_cube(8, 8, 8, VoxelType::Stone);
    spawn_test_shape(&mut commands, &mut meshes, &material, cube_chunk, Vec3::new(0.0, 0.0, 0.0), "Cube 8x8x8");

    // Wide wall: 16x2x8 - tests horizontal stretching
    let wall_chunk = create_cube(16, 2, 8, VoxelType::Stone);
    spawn_test_shape(&mut commands, &mut meshes, &material, wall_chunk, Vec3::new(5.0, 0.0, 0.0), "Wall 16x2x8");

    // Tall pillar: 2x16x2 - tests vertical stretching
    let pillar_chunk = create_cube(2, 16, 2, VoxelType::Stone);
    spawn_test_shape(&mut commands, &mut meshes, &material, pillar_chunk, Vec3::new(10.0, 0.0, 0.0), "Pillar 2x16x2");

    // Stairs: step pattern - tests mixed face sizes
    let stairs_chunk = create_stairs(8, VoxelType::Stone);
    spawn_test_shape(&mut commands, &mut meshes, &material, stairs_chunk, Vec3::new(15.0, 0.0, 0.0), "Stairs");

    // Camera positioned to see all shapes
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(8.0, 8.0, 15.0).looking_at(Vec3::new(8.0, 4.0, 0.0), Vec3::Y),
        FlyCameraController {
            move_speed: 5.0,
            ..default()
        },
    ));

    // Good lighting
    commands.spawn((
        DirectionalLight {
            illuminance: 8000.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, -0.6, 0.4, 0.0)),
    ));

    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 400.0,
        affects_lightmapped_meshes: false,
    });

    // Add sky color
    commands.insert_resource(ClearColor(Color::srgb(0.53, 0.81, 0.92)));
}

/// Create a solid rectangular cuboid of voxels
fn create_cube(width: usize, height: usize, depth: usize, voxel_type: VoxelType) -> Chunk {
    let mut chunk = Chunk::new();

    for x in 0..width.min(CHUNK_SIZE) {
        for y in 0..height.min(CHUNK_SIZE) {
            for z in 0..depth.min(CHUNK_SIZE) {
                chunk.set_voxel(x, y, z, Voxel {
                    voxel_type,
                    density: 255,
                });
            }
        }
    }

    chunk
}

/// Create stairs with increasing height
fn create_stairs(num_steps: usize, voxel_type: VoxelType) -> Chunk {
    let mut chunk = Chunk::new();

    for step in 0..num_steps.min(CHUNK_SIZE) {
        let step_height = step + 1;

        // Each step is 2 voxels deep
        for z in (step * 2)..(step * 2 + 2).min(CHUNK_SIZE) {
            for y in 0..step_height.min(CHUNK_SIZE) {
                for x in 0..4.min(CHUNK_SIZE) {
                    chunk.set_voxel(x, y, z, Voxel {
                        voxel_type,
                        density: 255,
                    });
                }
            }
        }
    }

    chunk
}

/// Spawn a test shape with label
fn spawn_test_shape(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    material: &Handle<StandardMaterial>,
    chunk: Chunk,
    position: Vec3,
    _label: &str,
) {
    let voxel_size = 0.25;
    let mesh = voxel::meshing::generate_chunk_mesh(&chunk, voxel_size);

    commands.spawn((
        Mesh3d(meshes.add(mesh)),
        MeshMaterial3d(material.clone()),
        Transform::from_translation(position),
    ));
}

/// Generate simple grid texture that tiles
fn generate_grid_texture() -> Image {
    use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
    use bevy::asset::RenderAssetUsages;

    // Simple 64x64 grid texture that tiles perfectly
    const SIZE: u32 = 64;
    const GRID_SIZE: u32 = 8; // Grid lines every 8 pixels

    let mut data = vec![0u8; (SIZE * SIZE * 4) as usize];

    for y in 0..SIZE {
        for x in 0..SIZE {
            // Create visible grid lines
            let is_grid_line = (x % GRID_SIZE == 0) || (y % GRID_SIZE == 0);

            let value = if is_grid_line {
                0.2 // Dark grid lines
            } else {
                0.6 // Light background
            };

            let color_u8 = (value * 255.0) as u8;

            let pixel_index = ((y * SIZE + x) * 4) as usize;
            data[pixel_index] = color_u8;
            data[pixel_index + 1] = color_u8;
            data[pixel_index + 2] = color_u8;
            data[pixel_index + 3] = 255;
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

    // Set texture to repeat mode so UVs > 1.0 tile the texture
    use bevy::image::{ImageSamplerDescriptor, ImageAddressMode};
    image.sampler = ImageSampler::Descriptor(ImageSamplerDescriptor {
        address_mode_u: ImageAddressMode::Repeat,
        address_mode_v: ImageAddressMode::Repeat,
        address_mode_w: ImageAddressMode::Repeat,
        ..default()
    });

    image
}

#[allow(dead_code)]
fn fill_solid_region(data: &mut [u8], material_index: u32, color: [u8; 4]) {
    const ATLAS_WIDTH: u32 = 256;
    const ATLAS_HEIGHT: u32 = 64;
    const MATERIAL_WIDTH: u32 = 64;

    let start_x = material_index * MATERIAL_WIDTH;
    let end_x = start_x + MATERIAL_WIDTH;

    for y in 0..ATLAS_HEIGHT {
        for x in start_x..end_x {
            let pixel_index = ((y * ATLAS_WIDTH + x) * 4) as usize;
            data[pixel_index] = color[0];
            data[pixel_index + 1] = color[1];
            data[pixel_index + 2] = color[2];
            data[pixel_index + 3] = color[3];
        }
    }
}
