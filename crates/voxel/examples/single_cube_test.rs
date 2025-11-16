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
    asset_server: Res<AssetServer>,
) {
    // Load the grid texture
    let texture_handle = asset_server.load("textures/debug_grid_8x8.png");

    // Create material with grid texture
    let material = materials.add(StandardMaterial {
        base_color_texture: Some(texture_handle),
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
