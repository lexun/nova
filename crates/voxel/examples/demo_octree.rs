//! Octree terrain example demonstrating 3D LOD with cubic terrain
//!
//! **STATUS: BROKEN** - Too poorly optimized to run at scale. When debugging,
//! start by generating less terrain to isolate issues.
//!
//! This example shows the new octree-based LOD system:
//! - Uses `VoxelTerrain::cubic(world_size)` for 3D octree LOD
//! - Supports unlimited height and 3D features (future: caves, tunnels)
//! - View-dependent adaptive subdivision
//!
//! Controls:
//! - Click to capture mouse
//! - Escape to release mouse
//! - WASD: Move camera horizontally
//! - QE: Move camera up/down
//! - Shift: Hold for faster movement
//! - Mouse: Look around (when captured)
//!
//! Run with: cargo run -p voxel --example demo_octree

use bevy::prelude::*;
use engine::{FlyCameraController, FlyCameraPlugin};
use voxel::{VoxelTerrain, VoxelTerrainPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((VoxelTerrainPlugin, FlyCameraPlugin))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    // Spawn terrain with octree LOD strategy
    commands.spawn(VoxelTerrain::cubic(512.0));

    // Camera with fly controller
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 50.0, 0.0).looking_at(Vec3::new(50.0, 0.0, 50.0), Vec3::Y),
        FlyCameraController {
            move_speed: 50.0,
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
}
