use bevy::{
    input::mouse::MouseMotion,
    pbr::wireframe::{Wireframe, WireframePlugin},
    prelude::*,
    window::CursorGrabMode,
};
use bevy_brp_extras::BrpExtrasPlugin;
use voxel::VoxelType;

const VOXEL_SIZE: f32 = 0.25; // 25cm voxels
const PLANET_RADIUS: f32 = 500.0; // 500m radius = 1km diameter

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

    // Camera positioned above planet surface
    let camera_height = PLANET_RADIUS + 100.0;
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, camera_height, 0.0).looking_at(Vec3::new(100.0, PLANET_RADIUS, 0.0), Vec3::Y),
        CameraController::default(),
    ));

    // Sun
    commands.spawn((
        DirectionalLight {
            illuminance: 10000.0,
            shadows_enabled: false, // Disable shadows for now (performance)
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, -0.7, 0.4, 0.0)),
    ));

    info!("Generating planet surface...");

    // Generate planet surface voxels
    // For now, we'll generate a sparse set of voxels to visualize the planet shape
    // In a real implementation, we'd use proper chunk management

    let samples_per_face = 128; // How many samples per cube face dimension
    let total_voxels = samples_per_face * samples_per_face * 6; // 6 faces

    info!("Generating {} surface samples", total_voxels);

    let stone_material = materials.add(Color::srgb(0.6, 0.6, 0.6));
    let dirt_material = materials.add(Color::srgb(0.4, 0.2, 0.1));
    let grass_material = materials.add(Color::srgb(0.2, 0.8, 0.2));

    let cube_mesh = meshes.add(Cuboid::new(VOXEL_SIZE, VOXEL_SIZE, VOXEL_SIZE));

    // Generate voxels for each cube face
    for face in 0..6 {
        for i in 0..samples_per_face {
            for j in 0..samples_per_face {
                // Convert to normalized coordinates [-1, 1]
                let u = (i as f32 / (samples_per_face - 1) as f32) * 2.0 - 1.0;
                let v = (j as f32 / (samples_per_face - 1) as f32) * 2.0 - 1.0;

                // Get sphere direction
                let direction = cube_to_sphere(face, u, v);

                // Base surface position
                let base_pos = direction * PLANET_RADIUS;

                // Add terrain variation
                let height = terrain_noise(base_pos);
                let surface_pos = direction * (PLANET_RADIUS + height);

                // Determine voxel type based on height
                let (_voxel_type, material) = if height > 10.0 {
                    (VoxelType::Stone, stone_material.clone())
                } else if height > 0.0 {
                    (VoxelType::Grass, grass_material.clone())
                } else {
                    (VoxelType::Dirt, dirt_material.clone())
                };

                // Spawn voxel
                commands.spawn((
                    Mesh3d(cube_mesh.clone()),
                    MeshMaterial3d(material),
                    Transform::from_translation(surface_pos),
                    Wireframe,
                ));
            }
        }
    }

    info!("Planet generation complete!");
    info!("Planet radius: {}m", PLANET_RADIUS);
    info!("Voxel size: {}m", VOXEL_SIZE);
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
    app.add_plugins(WireframePlugin::default());
    app.add_plugins(BrpExtrasPlugin);

    app.add_systems(Startup, setup_scene);
    app.add_systems(Update, camera_movement);

    app.run();
}
