use bevy::prelude::*;
use bevy_brp_extras::BrpExtrasPlugin;
use engine::{FlyCameraController, FlyCameraPlugin};
use voxel::{Chunk, Voxel, VoxelType};

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(3.0, 3.0, 3.0).looking_at(Vec3::ZERO, Vec3::Y),
        FlyCameraController::new(5.0, 0.003),
    ));

    // Human scale reference (1.8m tall person)
    commands.spawn((
        Mesh3d(meshes.add(Capsule3d::new(0.3, 1.8))),
        MeshMaterial3d(materials.add(Color::srgb(0.3, 0.3, 0.8))),
        Transform::from_xyz(0.0, 0.9, 0.0), // Position so bottom touches ground
    ));

    // Ground plane (neutral concrete color)
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(20.0, 20.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.7, 0.7, 0.7))),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    // Light
    commands.spawn((
        DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, -0.7, 0.4, 0.0)),
    ));

    // Create a test chunk with some voxels
    let mut chunk = Chunk::new();

    // Add some test voxels - a small tower
    for y in 0..5 {
        chunk.set_voxel(
            10,
            y,
            10,
            Voxel {
                voxel_type: VoxelType::Stone,
                density: 255,
            },
        );
    }

    // Add dirt foundation first
    for x in 8..13 {
        for z in 8..13 {
            if x != 10 || z != 10 {
                // Don't place under the stone tower
                chunk.set_voxel(
                    x,
                    0,
                    z,
                    Voxel {
                        voxel_type: VoxelType::Dirt,
                        density: 255,
                    },
                );
            }
        }
    }

    // Add grass layer on top of dirt
    for x in 8..13 {
        for z in 8..13 {
            if x != 10 || z != 10 {
                // Don't place under the stone tower
                chunk.set_voxel(
                    x,
                    1,
                    z,
                    Voxel {
                        voxel_type: VoxelType::Grass,
                        density: 255,
                    },
                );
            }
        }
    }

    // Generate mesh from chunk using greedy meshing
    let voxel_size = 0.25; // 25cm voxels (good for building scale)
    let chunk_offset = Vec3::new(-4.0, 0.125, -4.0); // Offset so voxel bottoms sit on bedrock

    let chunk_mesh = voxel::meshing::generate_chunk_mesh(&chunk, voxel_size);

    // Spawn single entity for entire chunk
    commands.spawn((
        Mesh3d(meshes.add(chunk_mesh)),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.5, 0.5, 0.5),
            perceptual_roughness: 0.8,
            ..default()
        })),
        Transform::from_translation(chunk_offset),
    ));

    info!("Created test chunk with greedy meshing");
    info!("Single mesh entity instead of individual voxels");
    info!("Stone tower at (10, 0-4, 10)");
    info!("Grass base around (8-12, 0, 8-12)");
    info!("Dirt foundation at (8-12, 1-2, 8-12)");
}

fn main() {
    let mut app = engine::create_app();

    // MCP Server Integration - Enable autonomous AI development and testing
    // BrpExtrasPlugin includes RemoteHttpPlugin + extras (screenshots, keyboard input, window control)
    app.add_plugins((BrpExtrasPlugin, FlyCameraPlugin));  // Bevy Remote Protocol (port 15702)
    // Note: bevy_debugger_mcp is an external MCP server (doesn't require a Bevy plugin)

    app.add_systems(Startup, setup_scene);
    app.run();
}
