//! Free-flying camera controller for development and editor use
//!
//! Provides WASD movement, QE for up/down, and mouse look controls.
//!
//! ## Controls
//! - **Click** window to capture mouse and enable camera rotation
//! - **Escape** to release mouse
//! - **WASD** - Move forward/left/back/right
//! - **QE** - Move down/up
//! - **Shift** - Hold for 3× faster movement
//! - **Mouse** - Rotate camera (when captured)

use bevy::{prelude::*, window::{CursorGrabMode, CursorOptions}};

/// Component that enables free-flying camera controls
#[derive(Component)]
pub struct FlyCameraController {
    /// Movement speed in units per second
    pub move_speed: f32,
    /// Mouse sensitivity for look rotation
    pub sensitivity: f32,
    /// Speed multiplier when holding Shift
    pub speed_boost: f32,
}

impl Default for FlyCameraController {
    fn default() -> Self {
        Self {
            move_speed: 10.0,
            sensitivity: 0.003,
            speed_boost: 3.0,
        }
    }
}

impl FlyCameraController {
    /// Create a new camera controller with custom settings
    pub fn new(move_speed: f32, sensitivity: f32) -> Self {
        Self {
            move_speed,
            sensitivity,
            speed_boost: 3.0,
        }
    }
}

/// System that handles mouse capture toggle
pub fn mouse_capture_system(
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut cursor_options: Query<&mut CursorOptions>,
) {
    let Ok(mut cursor) = cursor_options.single_mut() else {
        return;
    };

    // Click to capture mouse
    if mouse_button_input.just_pressed(MouseButton::Left) {
        cursor.grab_mode = CursorGrabMode::Locked;
        cursor.visible = false;
    }

    // Escape to release mouse
    if keyboard_input.just_pressed(KeyCode::Escape) {
        cursor.grab_mode = CursorGrabMode::None;
        cursor.visible = true;
    }
}

/// System that handles camera movement and rotation
pub fn fly_camera_system(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut mouse_motion: MessageReader<bevy::input::mouse::MouseMotion>,
    mut camera_query: Query<(&mut Transform, &FlyCameraController), With<Camera3d>>,
    cursor_options: Query<&CursorOptions>,
) {
    let Ok(cursor) = cursor_options.single() else {
        return;
    };
    let mouse_captured = cursor.grab_mode != CursorGrabMode::None;

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

        // Apply movement with optional speed boost
        velocity = velocity.normalize_or_zero();
        let speed = if keyboard_input.pressed(KeyCode::ShiftLeft) || keyboard_input.pressed(KeyCode::ShiftRight) {
            controller.move_speed * controller.speed_boost
        } else {
            controller.move_speed
        };
        transform.translation += velocity * speed * time.delta_secs();

        // Mouse look (only when mouse is captured)
        if mouse_captured {
            for mouse_event in mouse_motion.read() {
                let yaw = -mouse_event.delta.x * controller.sensitivity;
                let pitch = -mouse_event.delta.y * controller.sensitivity;

                transform.rotate_y(yaw);
                transform.rotate_local_x(pitch);
            }
        }
    }
}

/// Plugin that adds fly camera functionality
pub struct FlyCameraPlugin;

impl Plugin for FlyCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (mouse_capture_system, fly_camera_system).chain());
    }
}
