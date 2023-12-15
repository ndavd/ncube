use crate::resources::OrthographicCamera;
use crate::vec::{SphericalCoordinate, SphericalCoordinateSystem};
use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use bevy::window::PrimaryWindow;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_camera, spawn_lighting))
            .add_systems(Update, update_camera);
    }
}

const WHEEL_FACTOR: f32 = if cfg!(target_family = "wasm") {
    0.001
} else {
    0.3
};
const MOTION_FACTOR: f32 = 0.3;

pub fn get_default_camera_transform() -> Transform {
    Transform::from_translation(Vec3::from_spherical(SphericalCoordinate::new(
        4.0,
        std::f32::consts::PI / 2.0,
        0.0,
    )))
    .looking_at(Vec3::ZERO, Vec3::Y)
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: get_default_camera_transform(),
        camera_3d: Camera3d {
            clear_color: bevy::core_pipeline::clear_color::ClearColorConfig::Custom(Color::BLACK),
            ..default()
        },
        ..default()
    });
}

fn spawn_lighting(mut commands: Commands) {
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            ..default()
        },
        transform: Transform::from_xyz(3.0, 8.0, 4.0),
        ..default()
    });
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            ..default()
        },
        transform: Transform::from_xyz(-4.0, 8.0, -4.0),
        ..default()
    });
}

fn update_camera(
    mut q_camera: Query<(&mut Transform, &mut Projection), With<Camera>>,
    mut q_primary_window: Query<&mut Window, With<PrimaryWindow>>,
    mut mouse_wheel_events: EventReader<bevy::input::mouse::MouseWheel>,
    mut mouse_motion_events: EventReader<bevy::input::mouse::MouseMotion>,
    mouse_button_input: Res<Input<MouseButton>>,
    keys: Res<Input<KeyCode>>,
    orthographic_camera: Res<OrthographicCamera>,
) {
    let (mut camera_transform, mut camera_projection) = q_camera.get_single_mut().unwrap();
    let mut window = q_primary_window.get_single_mut().unwrap();

    let curr_pos_spherical = camera_transform.translation.to_spherical();

    if orthographic_camera.is_changed() {
        *camera_projection = if **orthographic_camera {
            Projection::Orthographic(OrthographicProjection {
                scaling_mode: ScalingMode::WindowSize(400.0),
                ..default()
            })
        } else {
            Projection::default()
        }
    }

    if keys.pressed(KeyCode::ControlLeft) || keys.pressed(KeyCode::ControlRight) {
        mouse_wheel_events.read().for_each(|event| {
            if event.y == 0.0 {
                return;
            }
            camera_transform.translation = Vec3::from_spherical(SphericalCoordinate::new(
                curr_pos_spherical.r * 1.0 - event.y * WHEEL_FACTOR,
                curr_pos_spherical.theta,
                curr_pos_spherical.phi,
            ));
        });
    }

    if !mouse_button_input.pressed(MouseButton::Right) {
        if mouse_button_input.just_released(MouseButton::Right) {
            let new_cursor_pos = Vec2::new(window.width() / 2.0, window.height() / 2.0);
            window.set_cursor_position(Some(new_cursor_pos));
            window.cursor.visible = true;
        }
        return;
    }

    window.cursor.visible = false;

    mouse_motion_events.read().for_each(|event| {
        if event.delta.length() == 0.0 {
            return;
        }
        camera_transform.translation = Vec3::from_spherical(SphericalCoordinate::new(
            curr_pos_spherical.r,
            (curr_pos_spherical.theta + event.delta.y.to_radians() * -MOTION_FACTOR)
                // Prevent flipping effect
                .min(std::f32::consts::PI * 0.99999)
                .max(0.00001),
            curr_pos_spherical.phi + event.delta.x.to_radians() * -MOTION_FACTOR,
        ));
        *camera_transform = camera_transform.looking_at(Vec3::ZERO, Vec3::Y);
    });
}
