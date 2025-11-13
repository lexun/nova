//! # Test: Chunk Gap Diagnosis
//!
//! Simple 2x2 flat grid of chunks to isolate gap issues.
//! All chunks solid green for easy visualization.
//!
//! Controls:
//! - WASD: Move camera
//! - QE: Up/Down
//! - Mouse: Look around

use bevy::{
    pbr::wireframe::{Wireframe, WireframePlugin},
    prelude::*,
};
use bevy_brp_extras::BrpExtrasPlugin;
use voxel::{Chunk, Voxel, VoxelType, CHUNK_SIZE};

const VOXEL_SIZE: f32 = 0.25;

#[derive(Component)]
struct CameraController {
    move_speed: f32,
    sensitivity: f32,
}

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Camera positioned to view the 2x2 grid
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(20.0, 20.0, 20.0).looking_at(Vec3::new(4.0, 0.0, 4.0), Vec3::Y),
        CameraController {
            move_speed: 10.0,
            sensitivity: 0.003,
        },
    ));

    // Directional light
    commands.spawn((
        DirectionalLight {
            illuminance: 10000.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, -0.7, 0.4, 0.0)),
    ));

    let material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.2, 0.8, 0.2),
        perceptual_roughness: 0.85,
        ..default()
    });

    info!("Generating 2x2 flat grid of chunks...");

    // Generate 2x2 grid of chunks
    for grid_x in 0..2 {
        for grid_z in 0..2 {
            let mut chunk = Chunk::new();

            // Fill entire chunk with solid voxels
            for x in 0..CHUNK_SIZE {
                for y in 0..CHUNK_SIZE {
                    for z in 0..CHUNK_SIZE {
                        chunk.set_voxel(x, y, z, Voxel {
                            voxel_type: VoxelType::Grass,
                            density: 255,
                        });
                    }
                }
            }

            // Generate mesh
            let mesh = voxel::meshing::generate_chunk_mesh(&chunk, VOXEL_SIZE);

            // Position chunk in grid
            // Each chunk is CHUNK_SIZE * VOXEL_SIZE = 32 * 0.25 = 8 meters
            let chunk_world_size = CHUNK_SIZE as f32 * VOXEL_SIZE;
            let position = Vec3::new(
                grid_x as f32 * chunk_world_size,
                0.0,
                grid_z as f32 * chunk_world_size,
            );

            commands.spawn((
                Mesh3d(meshes.add(mesh)),
                MeshMaterial3d(material.clone()),
                Transform::from_translation(position),
                Wireframe,
            ));

            info!("Spawned chunk at grid position ({}, {}), world position {:?}",
                  grid_x, grid_z, position);
        }
    }

    info!("2x2 grid generation complete!");
}

fn camera_movement(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut mouse_motion: MessageReader<bevy::input::mouse::MouseMotion>,
    mut camera_query: Query<(&mut Transform, &CameraController), With<Camera3d>>,
) {
    for (mut transform, controller) in camera_query.iter_mut() {
        let mut velocity = Vec3::ZERO;
        let forward = -*transform.local_z();
        let right = *transform.local_x();
        let up = *transform.local_y();

        // WASD movement
        if keyboard_input.pressed(KeyCode::KeyW) {
            velocity += forward;
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            velocity -= forward;
        }
        if keyboard_input.pressed(KeyCode::KeyA) {
            velocity -= right;
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            velocity += right;
        }
        if keyboard_input.pressed(KeyCode::KeyQ) {
            velocity -= up;
        }
        if keyboard_input.pressed(KeyCode::KeyE) {
            velocity += up;
        }

        velocity = velocity.normalize_or_zero();
        transform.translation += velocity * controller.move_speed * time.delta_secs();

        // Mouse look
        for mouse_event in mouse_motion.read() {
            let yaw = -mouse_event.delta.x * controller.sensitivity;
            let pitch = -mouse_event.delta.y * controller.sensitivity;

            transform.rotate_y(yaw);
            transform.rotate_local_x(pitch);
        }
    }
}

fn main() {
    let mut app = engine::create_app();
    app.add_plugins((BrpExtrasPlugin, WireframePlugin::default()));
    app.add_systems(Startup, setup_scene);
    app.add_systems(Update, camera_movement);
    app.run();
}
