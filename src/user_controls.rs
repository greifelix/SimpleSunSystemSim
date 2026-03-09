use crate::ControlText;
use crate::SimulationSpeed;
use crate::constants_types::{MAX_ZOOM_DISTANCE, MIN_ZOOM_DISTANCE};
use crate::planets::{Planet, SOLAR_SYSTEM_PLANETS};
use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct CameraTarget {
    // None means Sun, Some(index) means the planet at that index
    pub planet_index: Option<usize>,
}

fn update_ui_text(
    text_query: &mut Query<&mut Text, With<ControlText>>,
    speed: f32,
    target: &CameraTarget,
) {
    let target_name = match target.planet_index {
        None => "Sun",
        Some(idx) => SOLAR_SYSTEM_PLANETS[idx]._name,
    };

    for mut text in text_query {
        *text = Text::new(format!(
            "Controls:\nArrow Keys: Move/Rotate Camera\nSpace + L/R Arrow: Rotate Up/Down\n+/-: Adjust Simulation Speed (Current: 1 second -> {:.2} sim hours)\n0-8: Select Focus (0=Sun, 1-8=Planets)\nTarget: {}",
            speed / 3600.,
            target_name
        ));
    }
}

pub fn planet_selection(
    keys: Res<ButtonInput<KeyCode>>,
    mut target: ResMut<CameraTarget>,
    speed: Res<SimulationSpeed>,
    mut text_query: Query<&mut Text, With<ControlText>>,
) {
    let mut changed_target = false;

    if keys.just_pressed(KeyCode::Digit0) || keys.just_pressed(KeyCode::Numpad0) {
        target.planet_index = None;
        changed_target = true;
    } else if keys.just_pressed(KeyCode::Digit1) || keys.just_pressed(KeyCode::Numpad1) {
        target.planet_index = Some(0); // Mercury
        changed_target = true;
    } else if keys.just_pressed(KeyCode::Digit2) || keys.just_pressed(KeyCode::Numpad2) {
        target.planet_index = Some(1); // Venus
        changed_target = true;
    } else if keys.just_pressed(KeyCode::Digit3) || keys.just_pressed(KeyCode::Numpad3) {
        target.planet_index = Some(2); // Earth
        changed_target = true;
    } else if keys.just_pressed(KeyCode::Digit4) || keys.just_pressed(KeyCode::Numpad4) {
        target.planet_index = Some(3); // Mars
        changed_target = true;
    } else if keys.just_pressed(KeyCode::Digit5) || keys.just_pressed(KeyCode::Numpad5) {
        target.planet_index = Some(4); // Jupiter
        changed_target = true;
    } else if keys.just_pressed(KeyCode::Digit6) || keys.just_pressed(KeyCode::Numpad6) {
        target.planet_index = Some(5); // Saturn
        changed_target = true;
    } else if keys.just_pressed(KeyCode::Digit7) || keys.just_pressed(KeyCode::Numpad7) {
        target.planet_index = Some(6); // Uranus
        changed_target = true;
    } else if keys.just_pressed(KeyCode::Digit8) || keys.just_pressed(KeyCode::Numpad8) {
        target.planet_index = Some(7); // Neptune
        changed_target = true;
    }

    if changed_target {
        update_ui_text(&mut text_query, speed.0, &target);
    }
}

pub fn handle_simulation_speed(
    keys: Res<ButtonInput<KeyCode>>,
    mut speed: ResMut<SimulationSpeed>,
    target: Res<CameraTarget>,
    mut text_query: Query<&mut Text, With<ControlText>>,
) {
    let mut changed_speed = false;
    let speed_step = 3600.;

    if keys.pressed(KeyCode::Equal)
        || keys.pressed(KeyCode::NumpadAdd)
        || keys.pressed(KeyCode::BracketRight)
    {
        speed.0 += speed_step;
        changed_speed = true;
    }
    if keys.pressed(KeyCode::Minus)
        || keys.pressed(KeyCode::NumpadSubtract)
        || keys.pressed(KeyCode::Slash)
    {
        speed.0 -= speed_step;
        changed_speed = true;
    }

    if changed_speed {
        update_ui_text(&mut text_query, speed.0, &target);
    }
}

pub fn camera_zoomer(
    mut camera_query: Single<(&Camera3d, &mut Transform)>,
    planet_query: Query<(&Transform, &Planet), Without<Camera3d>>,
    target: Res<CameraTarget>,
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    // Sun is default target for the camera
    let mut target_pos = crate::constants_types::SUN_POSITION;

    // Change target in case a planet is selected
    if let Some(index) = target.planet_index {
        for (transform, planet) in &planet_query {
            if planet.index == index {
                target_pos = transform.translation;
                break;
            }
        }
    }

    let camera_transform = &mut camera_query.1;

    // Get current offset from target
    let current_offset = camera_transform.translation - target_pos;

    // Convert to spherical coordinates relative to target to avoid resetting issues
    let mut r = current_offset.length();
    if r < 0.001 {
        r = MIN_ZOOM_DISTANCE; // Fallback to avoid division by zero
    }

    let mut theta = (current_offset.y / r).acos(); // Polar angle
    let mut phi = current_offset.z.atan2(current_offset.x); // Azimuthal angle

    let zoom_dist = 30.0 * time.delta_secs();
    let rot_angle = 2.0 * time.delta_secs();

    // Zooming
    if keys.pressed(KeyCode::ArrowUp) {
        r = (r - zoom_dist).max(MIN_ZOOM_DISTANCE);
    }
    if keys.pressed(KeyCode::ArrowDown) {
        r = (r + zoom_dist).min(MAX_ZOOM_DISTANCE);
    }

    // Rotation
    if keys.pressed(KeyCode::Space) {
        // Space pressed: rotate up and down (polar angle)
        if keys.pressed(KeyCode::ArrowLeft) {
            theta -= rot_angle;
        }
        if keys.pressed(KeyCode::ArrowRight) {
            theta += rot_angle;
        }
    } else {
        // Normal: rotate left and right (azimuthal angle)
        if keys.pressed(KeyCode::ArrowLeft) {
            phi -= rot_angle;
        }
        if keys.pressed(KeyCode::ArrowRight) {
            phi += rot_angle;
        }
    }

    // Clamp theta to prevent gimbal lock / flipping over the poles
    theta = theta.clamp(0.01, std::f32::consts::PI - 0.01);

    // Convert back to Cartesian coordinates
    let new_x = r * theta.sin() * phi.cos();
    let new_y = r * theta.cos();
    let new_z = r * theta.sin() * phi.sin();

    camera_transform.translation = target_pos + Vec3::new(new_x, new_y, new_z);
    camera_transform.look_at(target_pos, Vec3::Y);
}
