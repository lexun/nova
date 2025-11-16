//! Large-scale terrain example demonstrating extended view distance
//!
//! This example showcases the high-level VoxelTerrain API with production settings:
//! - 2048m × 2048m world (16x larger than simple_terrain)
//! - 1000m view distance with 5 LOD levels
//! - Smooth transitions between detail levels
//!
//! Controls:
//! - WASD: Move camera horizontally
//! - Space/Shift: Move camera up/down
//! - Mouse: Look around
//!
//! Run with: cargo run -p voxel --example large_scale_terrain --release

use bevy::prelude::*;
use voxel::{VoxelTerrain, VoxelTerrainPlugin, lod::LodSettings};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(VoxelTerrainPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, camera_controller)
        .run();
}

fn setup(mut commands: Commands) {
    // Large-scale terrain with production LOD settings
    commands.spawn(
        VoxelTerrain::planar(2048.0)
            .with_lod_settings(LodSettings::production())
    );

    // Camera positioned for dramatic view
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 150.0, 0.0).looking_at(Vec3::new(200.0, 0.0, 200.0), Vec3::Y),
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

fn camera_controller(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut camera: Query<&mut Transform, With<Camera3d>>,
    time: Res<Time>,
) {
    let Ok(mut transform) = camera.single_mut() else {
        return;
    };
    let speed = 100.0 * time.delta_secs(); // Faster movement for larger world

    // Calculate directions first (immutable borrow)
    let forward = transform.forward();
    let right = transform.right();

    // Then mutate translation
    if keyboard.pressed(KeyCode::KeyW) {
        transform.translation += forward * speed;
    }
    if keyboard.pressed(KeyCode::KeyS) {
        transform.translation -= forward * speed;
    }
    if keyboard.pressed(KeyCode::KeyA) {
        transform.translation -= right * speed;
    }
    if keyboard.pressed(KeyCode::KeyD) {
        transform.translation += right * speed;
    }
    if keyboard.pressed(KeyCode::Space) {
        transform.translation.y += speed;
    }
    if keyboard.pressed(KeyCode::ShiftLeft) {
        transform.translation.y -= speed;
    }
}
