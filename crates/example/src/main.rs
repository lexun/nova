use bevy::{input::mouse::MouseMotion, prelude::*, window::CursorGrabMode};

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

    // Cube to look at
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.8, 0.2, 0.2))),
        Transform::from_xyz(0.0, 0.5, 0.0),
    ));

    // Ground plane
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(10.0, 10.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.3, 0.5, 0.3))),
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
    app.add_systems(Startup, setup_scene);
    app.add_systems(Update, camera_movement);
    app.run();
}
