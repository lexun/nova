//! Large-scale terrain example demonstrating extended view distance
//!
//! This example showcases the high-level VoxelTerrain API with production settings:
//! - 2048m × 2048m world (16x larger than simple_terrain)
//! - 1000m view distance with 5 LOD levels
//! - Smooth transitions between detail levels
//!
//! Controls:
//! - Click to capture mouse
//! - Escape to release mouse
//! - WASD: Move camera horizontally
//! - QE: Move camera up/down
//! - Shift: Hold for faster movement (3× speed)
//! - Mouse: Look around (when captured)
//!
//! Run with: cargo run -p voxel --example large_scale_terrain --release

use bevy::prelude::*;
use engine::{FlyCameraController, FlyCameraPlugin};
use voxel::{lod::LodSettings, VoxelTerrain, VoxelTerrainPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((VoxelTerrainPlugin, FlyCameraPlugin))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    // Large-scale terrain with production LOD settings
    commands.spawn(
        VoxelTerrain::planar(2048.0).with_lod_settings(LodSettings::production()),
    );

    // Camera positioned for dramatic view with faster movement speed
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 150.0, 0.0).looking_at(Vec3::new(200.0, 0.0, 200.0), Vec3::Y),
        FlyCameraController {
            move_speed: 100.0,
            ..default()
        },
    ));

    // Atmospheric lighting
    commands.spawn((
        DirectionalLight {
            illuminance: 15000.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, -0.4, 0.7, 0.0)),
    ));

    // Ambient light for better visibility
    commands.insert_resource(AmbientLight {
        color: Color::srgb(0.8, 0.85, 0.9),
        brightness: 200.0,
        ..default()
    });
}
