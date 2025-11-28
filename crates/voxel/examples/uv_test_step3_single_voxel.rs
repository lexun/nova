//! Step 3: Single 0.25m voxel cube
//!
//! Verifies that a single smallest voxel (0.25m) shows exactly 1 tile of texture on each face.
//! Should see complete 8×8 checkerboard on each visible face.

use bevy::prelude::*;
use bevy_brp_extras::BrpExtrasPlugin;
use voxel::{Chunk, Voxel, VoxelType};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, BrpExtrasPlugin))
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut images: ResMut<Assets<Image>>,
) {
    // Create camera - much closer to see texture detail
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.5, 0.4, 0.8).looking_at(Vec3::new(0.125, 0.125, 0.125), Vec3::Y),
    ));

    // Add light
    commands.spawn((
        DirectionalLight {
            illuminance: 10000.0,
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, -0.5, -0.5, 0.0)),
    ));

    // Create chunk with single voxel at origin
    let mut chunk = Chunk::new();
    chunk.set_voxel(0, 0, 0, Voxel { voxel_type: VoxelType::Stone, density: 255 });

    // Generate and spawn mesh
    let voxel_size = 0.25;
    let mesh = voxel::meshing::generate_chunk_mesh(&chunk, voxel_size);

    // Generate procedural atlas texture
    let atlas_image = voxel::atlas::generate_atlas();
    let texture_handle = images.add(atlas_image);

    commands.spawn((
        Mesh3d(meshes.add(mesh)),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color_texture: Some(texture_handle),
            unlit: true,
            ..default()
        })),
    ));
}
