use bevy::{
    input::mouse::MouseMotion,
    prelude::*,
    window::CursorGrabMode,
};
use bevy_brp_extras::BrpExtrasPlugin;
use voxel::{Chunk, Voxel, VoxelType, CHUNK_SIZE};

const VOXEL_SIZE: f32 = 0.25; // 25cm voxels
const PLANET_RADIUS: f32 = 500.0; // 500m radius = 1km diameter
const CHUNKS_PER_FACE_EDGE: usize = 8; // 8x8 chunks per cube face

#[derive(Component)]
struct CameraController {
    move_speed: f32,
    sensitivity: f32,
}

impl Default for CameraController {
    fn default() -> Self {
        Self {
            move_speed: 50.0, // Faster for planet scale
            sensitivity: 0.003,
        }
    }
}

/// Maps a position on a cube face to a normalized direction on a sphere
fn cube_to_sphere(face: usize, u: f32, v: f32) -> Vec3 {
    // u and v are in range [-1, 1] representing position on the cube face
    let dir = match face {
        0 => Vec3::new(1.0, v, -u),   // +X face
        1 => Vec3::new(-1.0, v, u),   // -X face
        2 => Vec3::new(u, 1.0, -v),   // +Y face
        3 => Vec3::new(u, -1.0, v),   // -Y face
        4 => Vec3::new(u, v, 1.0),    // +Z face
        5 => Vec3::new(-u, v, -1.0),  // -Z face
        _ => Vec3::ZERO,
    };
    dir.normalize()
}

/// Simple noise function for terrain height
fn terrain_noise(pos: Vec3) -> f32 {
    // Very basic noise using sine waves
    // In a real implementation, use proper noise (Perlin, Simplex, etc.)
    let scale = 0.05;
    let x = pos.x * scale;
    let y = pos.y * scale;
    let z = pos.z * scale;

    let n1 = (x * 1.2).sin() * (y * 0.9).cos() * (z * 1.5).sin();
    let n2 = (x * 2.4).cos() * (y * 1.8).sin() * (z * 3.0).cos();
    let n3 = (x * 4.8).sin() * (y * 3.6).cos() * (z * 6.0).sin();

    (n1 + n2 * 0.5 + n3 * 0.25) * 20.0 // Amplitude of ~20m
}

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut cursor_options: Query<&mut bevy::window::CursorOptions>,
) {
    // Setup cursor capture
    if let Ok(mut cursor) = cursor_options.single_mut() {
        cursor.grab_mode = CursorGrabMode::Locked;
        cursor.visible = false;
    }

    // Camera positioned to view planet from distance
    let camera_distance = PLANET_RADIUS * 2.5; // 1.25km from center
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(camera_distance, 0.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
        CameraController::default(),
    ));

    // Sun
    commands.spawn((
        DirectionalLight {
            illuminance: 10000.0,
            shadows_enabled: false,
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, -0.7, 0.4, 0.0)),
    ));

    info!("Generating planet surface with chunks...");

    let total_chunks = CHUNKS_PER_FACE_EDGE * CHUNKS_PER_FACE_EDGE * 6;
    info!("Generating {} chunks ({} per face edge)", total_chunks, CHUNKS_PER_FACE_EDGE);

    let chunk_world_size = CHUNK_SIZE as f32 * VOXEL_SIZE; // 8 meters per chunk

    // Materials for different terrain types
    let stone_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.6, 0.6, 0.6),
        perceptual_roughness: 0.8,
        ..default()
    });
    let dirt_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.4, 0.2, 0.1),
        perceptual_roughness: 0.9,
        ..default()
    });
    let grass_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.2, 0.8, 0.2),
        perceptual_roughness: 0.85,
        ..default()
    });

    let mut chunks_generated = 0;

    // Generate chunks for each cube face
    for face in 0..6 {
        for chunk_u in 0..CHUNKS_PER_FACE_EDGE {
            for chunk_v in 0..CHUNKS_PER_FACE_EDGE {
                // Generate chunk at this grid position
                let chunk = generate_planet_chunk(face, chunk_u, chunk_v);

                // Generate mesh from chunk
                let mesh = voxel::meshing::generate_chunk_mesh(&chunk, VOXEL_SIZE);

                // Calculate chunk center position on sphere
                let u_start = (chunk_u as f32 / CHUNKS_PER_FACE_EDGE as f32) * 2.0 - 1.0;
                let v_start = (chunk_v as f32 / CHUNKS_PER_FACE_EDGE as f32) * 2.0 - 1.0;
                let u_center = u_start + (1.0 / CHUNKS_PER_FACE_EDGE as f32);
                let v_center = v_start + (1.0 / CHUNKS_PER_FACE_EDGE as f32);

                let chunk_direction = cube_to_sphere(face, u_center, v_center);
                let chunk_position = chunk_direction * PLANET_RADIUS;

                // Calculate rotation to orient chunk with sphere surface
                // Rotate so chunk's local Y-axis points radially outward
                let rotation = Quat::from_rotation_arc(Vec3::Y, chunk_direction);

                // Choose material based on average height (simplified)
                let material = grass_material.clone();

                // Spawn chunk mesh with position and rotation
                commands.spawn((
                    Mesh3d(meshes.add(mesh)),
                    MeshMaterial3d(material),
                    Transform::from_translation(chunk_position).with_rotation(rotation),
                ));

                chunks_generated += 1;
            }
        }
    }

    info!("Planet generation complete!");
    info!("Generated {} chunks", chunks_generated);
    info!("Planet radius: {}m", PLANET_RADIUS);
    info!("Chunk size: {}m", chunk_world_size);
}

/// Generate a chunk of voxels for a specific position on the planet
fn generate_planet_chunk(face: usize, chunk_u: usize, chunk_v: usize) -> Chunk {
    let mut chunk = Chunk::new();

    // Calculate the UV range and center for this chunk on the cube face
    let u_start = (chunk_u as f32 / CHUNKS_PER_FACE_EDGE as f32) * 2.0 - 1.0;
    let v_start = (chunk_v as f32 / CHUNKS_PER_FACE_EDGE as f32) * 2.0 - 1.0;
    let u_center = u_start + (1.0 / CHUNKS_PER_FACE_EDGE as f32);
    let v_center = v_start + (1.0 / CHUNKS_PER_FACE_EDGE as f32);

    // Get chunk's position and orientation
    let chunk_direction = cube_to_sphere(face, u_center, v_center);
    let chunk_position = chunk_direction * PLANET_RADIUS;
    let chunk_rotation = Quat::from_rotation_arc(Vec3::Y, chunk_direction);

    // Fill chunk with voxels
    for x in 0..CHUNK_SIZE {
        for y in 0..CHUNK_SIZE {
            for z in 0..CHUNK_SIZE {
                // Calculate voxel's local position within chunk
                let local_x = (x as f32 - CHUNK_SIZE as f32 / 2.0) * VOXEL_SIZE;
                let local_y = (y as f32 - CHUNK_SIZE as f32 / 2.0) * VOXEL_SIZE;
                let local_z = (z as f32 - CHUNK_SIZE as f32 / 2.0) * VOXEL_SIZE;
                let local_pos = Vec3::new(local_x, local_y, local_z);

                // Transform to world space
                let world_pos = chunk_position + chunk_rotation * local_pos;

                // Calculate radial distance from planet center
                let radial_distance = world_pos.length();

                // Get terrain surface radius at this direction
                let world_direction = world_pos.normalize();
                let surface_pos = world_direction * PLANET_RADIUS;
                let terrain_height = terrain_noise(surface_pos);
                let surface_radius = PLANET_RADIUS + terrain_height;

                // Voxel is solid if it's underground (closer to center than surface)
                if radial_distance < surface_radius {
                    // Depth below surface determines material type
                    let depth_below_surface = surface_radius - radial_distance;

                    let voxel_type = if terrain_height > 10.0 {
                        VoxelType::Stone
                    } else if depth_below_surface > 0.5 {
                        VoxelType::Dirt
                    } else {
                        VoxelType::Grass
                    };

                    chunk.set_voxel(x, y, z, Voxel {
                        voxel_type,
                        density: 255,
                    });
                }
            }
        }
    }

    chunk
}

fn camera_movement(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut mouse_motion: MessageReader<MouseMotion>,
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

    // Plugins
    app.add_plugins(BrpExtrasPlugin);

    app.add_systems(Startup, setup_scene);
    app.add_systems(Update, camera_movement);

    app.run();
}
