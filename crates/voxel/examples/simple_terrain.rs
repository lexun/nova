//! Simple terrain example demonstrating the high-level VoxelTerrain API
//!
//! This example shows how easy it is to create voxel terrain with sensible defaults:
//! - Just spawn `VoxelTerrain::planar(world_size)`
//! - Plugin handles LOD management, chunk generation, and rendering
//!
//! Controls:
//! - Click to capture mouse
//! - Escape to release mouse
//! - WASD: Move camera horizontally
//! - QE: Move camera up/down
//! - Shift: Hold for faster movement
//! - Mouse: Look around (when captured)
//!
//! Run with: cargo run -p voxel --example simple_terrain

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
    // Spawn terrain with one line!
    commands.spawn(VoxelTerrain::planar(512.0));

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
