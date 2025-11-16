//! Visual test example - small scale with proper colors and lighting
//!
//! Purpose: Make it easy to see and understand the terrain
//! - Small world (64m) to load quickly
//! - Colored materials (green grass, brown dirt, gray stone)
//! - Good lighting (softer shadows, better ambient)
//! - Camera positioned to see the terrain clearly
//!
//! Run with: cargo run -p voxel --example visual_test

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

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Create a small world that loads quickly
    // Use heightmap mode (proven, fast) not octree
    commands.spawn(VoxelTerrain::planar(64.0));

    // Camera positioned ABOVE terrain, looking down at it
    // Start at 30m up, looking at the center of the terrain
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(32.0, 30.0, 32.0)
            .looking_at(Vec3::new(32.0, 0.0, 32.0), Vec3::Y),
        FlyCameraController {
            move_speed: 10.0, // Slower speed for small world
            ..default()
        },
    ));

    // Softer directional light (reduced shadows)
    commands.spawn((
        DirectionalLight {
            illuminance: 8000.0, // Slightly less harsh
            shadows_enabled: true,
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, -0.7, 0.3, 0.0)),
    ));

    // Add ambient light so we can see into shadows
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 500.0, // Good ambient to fill in shadows
        affects_lightmapped_meshes: false,
    });

    // TODO: Once we add colored materials, we'll create them here
    // and pass them to the terrain
}
