//! Simple terrain example demonstrating the high-level VoxelTerrain API
//!
//! This example shows how easy it is to create voxel terrain with sensible defaults:
//! - Just spawn `VoxelTerrain::planar(world_size)`
//! - Plugin handles LOD management, chunk generation, and rendering
//!
//! Controls:
//! - WASD: Move camera horizontally
//! - Space/Shift: Move camera up/down
//! - Mouse: Look around
//!
//! Run with: cargo run -p voxel --example simple_terrain

use bevy::prelude::*;
use voxel::{VoxelTerrain, VoxelTerrainPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(VoxelTerrainPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, camera_controller)
        .run();
}

fn setup(mut commands: Commands) {
    // Spawn terrain with one line!
    commands.spawn(VoxelTerrain::planar(512.0));

    // Camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 50.0, 0.0).looking_at(Vec3::new(50.0, 0.0, 50.0), Vec3::Y),
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

fn camera_controller(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut camera: Query<&mut Transform, With<Camera3d>>,
    time: Res<Time>,
) {
    let Ok(mut transform) = camera.single_mut() else {
        return;
    };
    let speed = 50.0 * time.delta_secs();

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
