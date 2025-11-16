//! Cave terrain example demonstrating 3D features with cave carving
//!
//! This example shows the new 3D terrain generation system:
//! - Uses `CaveTerrainGenerator` with density-based voxel generation
//! - Demonstrates caves, tunnels, and overhangs
//! - Works with octree LOD for true 3D terrain
//!
//! Controls:
//! - Click to capture mouse
//! - Escape to release mouse
//! - WASD: Move camera horizontally
//! - QE: Move camera up/down
//! - Shift: Hold for faster movement
//! - Mouse: Look around (when captured)
//!
//! Run with: cargo run -p voxel --example caves

use bevy::prelude::*;
use engine::{FlyCameraController, FlyCameraPlugin};
use voxel::{CaveTerrainGenerator, VoxelTerrain, VoxelTerrainPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((VoxelTerrainPlugin, FlyCameraPlugin))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    // Spawn cave terrain with octree LOD
    // Use cubic() for 3D octree, then set custom generator
    commands.spawn(
        VoxelTerrain::cubic(512.0)
            .with_generator(
                CaveTerrainGenerator::new()
                    .with_max_world_size(512.0)
                    .with_cave_threshold(0.25) // More caves (lower threshold = more caves)
                    .with_cave_frequency(0.06) // Slightly larger caves
            )
    );

    // Camera starts underground to see caves
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(50.0, -10.0, 50.0).looking_at(Vec3::new(60.0, -10.0, 60.0), Vec3::Y),
        FlyCameraController {
            move_speed: 30.0,
            ..default()
        },
    ));

    // Light
    commands.spawn((
        DirectionalLight {
            illuminance: 10000.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, -0.5, 0.5, 0.0)),
    ));

    // Ambient light for caves
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 300.0, // Moderate ambient light to see cave interiors
        affects_lightmapped_meshes: false,
    });
}
