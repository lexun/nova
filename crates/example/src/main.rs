use bevy::{
    input::mouse::MouseMotion,
    pbr::wireframe::{Wireframe, WireframePlugin},
    prelude::*,
    window::CursorGrabMode,
};
use voxel::{Chunk, Voxel, VoxelType};

#[derive(Component)]
struct CameraController {
    move_speed: f32,
    sensitivity: f32,
}

impl Default for CameraController {
    fn default() -> Self {
        Self {
            move_speed: 5.0,
            sensitivity: 0.003,
        }
    }
}

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut windows: Query<&mut Window>,
) {
    // Setup cursor capture
    if let Ok(mut window) = windows.single_mut() {
        window.cursor_options.grab_mode = CursorGrabMode::Locked;
        window.cursor_options.visible = false;
    }
    // Camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(3.0, 3.0, 3.0).looking_at(Vec3::ZERO, Vec3::Y),
        CameraController::default(),
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

    // Visualize the chunk with debug cubes
    let voxel_size = 0.25; // 25cm voxels (good for building scale)
    let chunk_offset = Vec3::new(-4.0, 0.125, -4.0); // Offset so voxel bottoms sit on bedrock

    for x in 0..32 {
        for y in 0..32 {
            for z in 0..32 {
                if let Some(voxel) = chunk.get_voxel(x, y, z) {
                    if voxel.voxel_type != VoxelType::Air && voxel.density > 0 {
                        let position = chunk_offset
                            + Vec3::new(
                                x as f32 * voxel_size,
                                y as f32 * voxel_size,
                                z as f32 * voxel_size,
                            );

                        let color = match voxel.voxel_type {
                            VoxelType::Stone => Color::srgb(0.6, 0.6, 0.6),
                            VoxelType::Dirt => Color::srgb(0.4, 0.2, 0.1),
                            VoxelType::Grass => Color::srgb(0.2, 0.8, 0.2),
                            VoxelType::Air => continue,
                        };

                        commands.spawn((
                            Mesh3d(meshes.add(Cuboid::new(voxel_size, voxel_size, voxel_size))),
                            MeshMaterial3d(materials.add(color)),
                            Transform::from_translation(position),
                            Wireframe,
                        ));
                    }
                }
            }
        }
    }

    info!("Created test chunk with voxels");
    info!("Stone tower at (10, 0-4, 10)");
    info!("Grass base around (8-12, 0, 8-12)");
    info!("Dirt foundation at (8-12, 1-2, 8-12)");
}

fn camera_movement(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut mouse_motion: EventReader<MouseMotion>,
    mut camera_query: Query<(&mut Transform, &CameraController), With<Camera3d>>,
) {
    for (mut transform, controller) in camera_query.iter_mut() {
        let mut velocity = Vec3::ZERO;
        let forward = -*transform.local_z();
        let right = *transform.local_x();
        let up = Vec3::Y;

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

        // Mouse look (captured)
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
    app.add_plugins(WireframePlugin::default());
    app.add_systems(Startup, setup_scene);
    app.add_systems(Update, camera_movement);
    app.run();
}
