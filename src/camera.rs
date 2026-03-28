//! Define the behaviour of the main Camera

use bevy::{input::mouse::MouseWheel, prelude::*};

const CAMERA_SPEED: f32 = 0.1;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (move_camera, handle_camera_zoom));
    }
}

/// Marker component for the main camera
#[derive(Component, Debug)]
pub struct MainCamera;

/// Bundle for easier MainCamera instantiation
#[derive(Bundle)]
pub struct MainCameraBundle {
    main_camera: MainCamera,
    camera_3d: Camera3d,
    projection: Projection,
}

impl MainCameraBundle {
    pub fn new() -> Self {
        MainCameraBundle {
            main_camera: MainCamera,
            camera_3d: Camera3d::default(),
            projection: Projection::Perspective(PerspectiveProjection {
                fov: core::f32::consts::PI / 8.,
                ..default()
            }),
        }
    }
}

/// Move the camera with WASD
fn move_camera(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    cameras: Query<&mut Transform, With<MainCamera>>,
) {
    let mut direction = Vec2::ZERO;

    if keyboard_input.pressed(KeyCode::KeyW) {
        direction.y -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyS) {
        direction.y += 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyD) {
        direction.x += 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyA) {
        direction.x -= 1.0;
    }

    direction *= CAMERA_SPEED;
    for mut camera in cameras {
        camera.translation.x += direction.x;
        camera.translation.z += direction.y;
    }
}

/// Handle the zoom of the Camera
fn handle_camera_zoom(
    mut evr_scroll: MessageReader<MouseWheel>,
    cameras: Query<&mut Transform, With<MainCamera>>,
) {
    let mut zoom: f32 = 0.0;
    for ev in evr_scroll.read() {
        zoom += ev.y;
    }

    for mut camera in cameras {
        camera.translation.y += zoom;
    }
}
