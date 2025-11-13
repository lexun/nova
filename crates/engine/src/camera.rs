//! Free-flying camera controller for development and editor use
//!
//! Provides WASD movement, QE for up/down, and mouse look controls.

use bevy::prelude::*;

/// Component that enables free-flying camera controls
#[derive(Component)]
pub struct FlyCameraController {
    /// Movement speed in units per second
    pub move_speed: f32,
    /// Mouse sensitivity for look rotation
    pub sensitivity: f32,
}

impl Default for FlyCameraController {
    fn default() -> Self {
        Self {
            move_speed: 10.0,
            sensitivity: 0.003,
        }
    }
}

impl FlyCameraController {
    /// Create a new camera controller with custom settings
    pub fn new(move_speed: f32, sensitivity: f32) -> Self {
        Self {
            move_speed,
            sensitivity,
        }
    }
}

/// System that handles camera movement and rotation
pub fn fly_camera_system(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut mouse_motion: MessageReader<bevy::input::mouse::MouseMotion>,
    mut camera_query: Query<(&mut Transform, &FlyCameraController), With<Camera3d>>,
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

        // QE for up/down
        if keyboard_input.pressed(KeyCode::KeyQ) {
            velocity -= up;
        }
        if keyboard_input.pressed(KeyCode::KeyE) {
            velocity += up;
        }

        // Apply movement
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

/// Plugin that adds fly camera functionality
pub struct FlyCameraPlugin;

impl Plugin for FlyCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, fly_camera_system);
    }
}
