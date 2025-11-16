//! Single 8x8x8 cube test for UV mapping verification
//! This minimal example lets us inspect one shape closely from all angles

use bevy::prelude::*;
use bevy_brp_extras::BrpExtrasPlugin;
use engine::{FlyCameraController, FlyCameraPlugin};
use voxel::{meshing::generate_chunk_mesh, Chunk, Voxel, VoxelType};

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
    // Generate simple grid texture procedurally
    let grid_image = generate_grid_texture();
    let grid_texture = images.add(grid_image);

    // Create material with grid texture
    let material = materials.add(StandardMaterial {
        base_color_texture: Some(grid_texture),
        unlit: true,
        ..default()
    });

    // Create a single 8x8x8 cube of stone voxels
    let mut chunk = Chunk::new();
    for x in 0..8 {
        for y in 0..8 {
            for z in 0..8 {
                chunk.set_voxel(
                    x,
                    y,
                    z,
                    Voxel {
                        voxel_type: VoxelType::Stone,
                        density: 255,
                    },
                );
            }
        }
    }

    // Generate mesh
    let mesh = generate_chunk_mesh(&chunk, 0.25);
    let mesh_handle = meshes.add(mesh);

    // Spawn the cube
    commands.spawn((
        Mesh3d(mesh_handle),
        MeshMaterial3d(material),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    // Add light
    commands.spawn((
        DirectionalLight {
            illuminance: 10000.0,
            shadows_enabled: false,
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, -0.5, -0.5, 0.0)),
    ));

    // Spawn camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(5.0, 3.0, 5.0).looking_at(Vec3::new(1.0, 1.0, 1.0), Vec3::Y),
        FlyCameraController::default(),
    ));
}

/// Generate simple 64x64 grid texture that tiles
fn generate_grid_texture() -> Image {
    use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
    use bevy::asset::RenderAssetUsages;
    use bevy::image::{ImageSampler, ImageSamplerDescriptor, ImageAddressMode};

    const SIZE: u32 = 64;
    const GRID_SIZE: u32 = 8;

    let mut data = vec![0u8; (SIZE * SIZE * 4) as usize];

    for y in 0..SIZE {
        for x in 0..SIZE {
            let is_grid_line = (x % GRID_SIZE == 0) || (y % GRID_SIZE == 0);
            let value = if is_grid_line { 0.2 } else { 0.6 };
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

    image.sampler = ImageSampler::Descriptor(ImageSamplerDescriptor {
        address_mode_u: ImageAddressMode::Repeat,
        address_mode_v: ImageAddressMode::Repeat,
        address_mode_w: ImageAddressMode::Repeat,
        ..default()
    });

    image
}
