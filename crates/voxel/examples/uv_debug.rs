//! UV mapping debug - close-up examination of texture coordinates
//!
//! This example creates single faces at different sizes and orientations
//! to test UV coordinate generation in the greedy meshing algorithm.
//!
//! What to look for:
//! 1. **Correct UV mapping** - Grid should be square, not stretched
//! 2. **Consistent scaling** - All faces should show similar grid density
//! 3. **Orientation independence** - X/Y/Z faces should map identically
//! 4. **Size independence** - Small and large quads should use same UV range
//!
//! The scene contains walls showing:
//! - Different face sizes (1x1, 4x4, 8x8, 16x4)
//! - Different orientations (XY, XZ, YZ planes)
//! - Both front and back faces
//!
//! Each wall is positioned separately so you can orbit around and inspect.
//!
//! Controls:
//! - Click to capture mouse
//! - Escape to release mouse
//! - WASD: Move horizontally
//! - QE: Move up/down
//! - Mouse: Look around
//!
//! Run with: cargo run -p voxel --example uv_debug

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
    // Generate checkerboard atlas for UV debugging
    let atlas_image = generate_checkerboard_atlas();
    let atlas_texture = images.add(atlas_image);

    let material = materials.add(StandardMaterial {
        base_color_texture: Some(atlas_texture),
        perceptual_roughness: 0.8,
        unlit: false,
        ..default()
    });

    // Row 1: Different sizes, same orientation (XY plane - front faces)
    spawn_wall(&mut commands, &mut meshes, &material, 1, 1, VoxelType::Stone, Vec3::new(0.0, 0.0, 0.0), "1x1");
    spawn_wall(&mut commands, &mut meshes, &material, 4, 4, VoxelType::Stone, Vec3::new(2.0, 0.0, 0.0), "4x4");
    spawn_wall(&mut commands, &mut meshes, &material, 8, 8, VoxelType::Stone, Vec3::new(6.0, 0.0, 0.0), "8x8");
    spawn_wall(&mut commands, &mut meshes, &material, 16, 4, VoxelType::Stone, Vec3::new(12.0, 0.0, 0.0), "16x4");

    // Row 2: Different orientations (all 8x8)
    spawn_horizontal_wall(&mut commands, &mut meshes, &material, 8, 8, VoxelType::Dirt, Vec3::new(0.0, 3.0, 0.0), "Horizontal XZ");
    spawn_depth_wall(&mut commands, &mut meshes, &material, 8, 8, VoxelType::Grass, Vec3::new(6.0, 3.0, 0.0), "Depth YZ");

    // Camera positioned to see test walls
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(8.0, 4.0, 12.0).looking_at(Vec3::new(8.0, 2.0, 0.0), Vec3::Y),
        FlyCameraController {
            move_speed: 5.0,
            ..default()
        },
    ));

    // Bright even lighting
    commands.spawn((
        DirectionalLight {
            illuminance: 10000.0,
            shadows_enabled: false,
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, -0.5, 0.3, 0.0)),
    ));

    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 800.0,
        affects_lightmapped_meshes: false,
    });

    commands.insert_resource(ClearColor(Color::srgb(0.2, 0.2, 0.3)));
}

/// Create vertical wall (XY plane, front facing)
fn spawn_wall(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    material: &Handle<StandardMaterial>,
    width: usize,
    height: usize,
    voxel_type: VoxelType,
    position: Vec3,
    _label: &str,
) {
    let mut chunk = Chunk::new();

    // Create single-voxel-deep wall
    for x in 0..width.min(CHUNK_SIZE) {
        for y in 0..height.min(CHUNK_SIZE) {
            chunk.set_voxel(x, y, 0, Voxel {
                voxel_type,
                density: 255,
            });
        }
    }

    let voxel_size = 0.25;
    let mesh = voxel::meshing::generate_chunk_mesh(&chunk, voxel_size);

    commands.spawn((
        Mesh3d(meshes.add(mesh)),
        MeshMaterial3d(material.clone()),
        Transform::from_translation(position),
    ));
}

/// Create horizontal wall (XZ plane, top facing)
fn spawn_horizontal_wall(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    material: &Handle<StandardMaterial>,
    width: usize,
    depth: usize,
    voxel_type: VoxelType,
    position: Vec3,
    _label: &str,
) {
    let mut chunk = Chunk::new();

    // Create single-voxel-tall horizontal surface
    for x in 0..width.min(CHUNK_SIZE) {
        for z in 0..depth.min(CHUNK_SIZE) {
            chunk.set_voxel(x, 0, z, Voxel {
                voxel_type,
                density: 255,
            });
        }
    }

    let voxel_size = 0.25;
    let mesh = voxel::meshing::generate_chunk_mesh(&chunk, voxel_size);

    commands.spawn((
        Mesh3d(meshes.add(mesh)),
        MeshMaterial3d(material.clone()),
        Transform::from_translation(position),
    ));
}

/// Create depth wall (YZ plane, side facing)
fn spawn_depth_wall(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    material: &Handle<StandardMaterial>,
    depth: usize,
    height: usize,
    voxel_type: VoxelType,
    position: Vec3,
    _label: &str,
) {
    let mut chunk = Chunk::new();

    // Create single-voxel-wide depth wall
    for z in 0..depth.min(CHUNK_SIZE) {
        for y in 0..height.min(CHUNK_SIZE) {
            chunk.set_voxel(0, y, z, Voxel {
                voxel_type,
                density: 255,
            });
        }
    }

    let voxel_size = 0.25;
    let mesh = voxel::meshing::generate_chunk_mesh(&chunk, voxel_size);

    commands.spawn((
        Mesh3d(meshes.add(mesh)),
        MeshMaterial3d(material.clone()),
        Transform::from_translation(position),
    ));
}

/// Generate checkerboard pattern atlas
/// Black and white squares make UV mapping issues very obvious
fn generate_checkerboard_atlas() -> Image {
    use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
    use bevy::asset::RenderAssetUsages;

    const ATLAS_WIDTH: u32 = 256;
    const ATLAS_HEIGHT: u32 = 64;
    const MATERIAL_WIDTH: u32 = 64;

    let mut data = vec![0u8; (ATLAS_WIDTH * ATLAS_HEIGHT * 4) as usize];

    // Fill all material regions with checkerboard
    for material_idx in 0..4 {
        let start_x = material_idx * MATERIAL_WIDTH;
        let end_x = start_x + MATERIAL_WIDTH;

        // Get base color for this material
        let base_color = match material_idx {
            0 => [0, 0, 0, 0],         // Air - transparent
            1 => [76, 153, 51, 255],   // Grass - green
            2 => [127, 89, 51, 255],   // Dirt - brown
            3 => [127, 127, 127, 255], // Stone - gray
            _ => [255, 0, 255, 255],
        };

        // Create checkerboard pattern
        let checker_size = 4; // 4x4 pixel checkers

        for y in 0..ATLAS_HEIGHT {
            for x in start_x..end_x {
                let local_x = x - start_x;

                // Determine if this pixel is in a light or dark square
                let checker_x = (local_x / checker_size) % 2;
                let checker_y = (y / checker_size) % 2;
                let is_light = (checker_x + checker_y) % 2 == 0;

                // Modulate base color
                let color = if material_idx == 0 {
                    base_color // Keep air transparent
                } else if is_light {
                    // Lighter square - brighten base color
                    [
                        ((base_color[0] as f32 * 1.3).min(255.0)) as u8,
                        ((base_color[1] as f32 * 1.3).min(255.0)) as u8,
                        ((base_color[2] as f32 * 1.3).min(255.0)) as u8,
                        base_color[3],
                    ]
                } else {
                    // Darker square - darken base color
                    [
                        ((base_color[0] as f32 * 0.7)) as u8,
                        ((base_color[1] as f32 * 0.7)) as u8,
                        ((base_color[2] as f32 * 0.7)) as u8,
                        base_color[3],
                    ]
                };

                let pixel_index = ((y * ATLAS_WIDTH + x) * 4) as usize;
                data[pixel_index] = color[0];
                data[pixel_index + 1] = color[1];
                data[pixel_index + 2] = color[2];
                data[pixel_index + 3] = color[3];
            }
        }
    }

    Image::new(
        Extent3d {
            width: ATLAS_WIDTH,
            height: ATLAS_HEIGHT,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        data,
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::RENDER_WORLD,
    )
}
