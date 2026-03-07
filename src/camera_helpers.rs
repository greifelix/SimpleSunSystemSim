use super::constants::{MAX_ZOOM_DISTANCE, MIN_ZOOM_DISTANCE};
use bevy::prelude::*;

pub fn camera_zoomer(
    mut camera_query: Single<(&Camera3d, &mut Transform)>,
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let sun_pos = crate::constants::SUN_POSITION;
    let zoom_dist = 30.0 * time.delta_secs();
    let rot_angle = 2.0 * time.delta_secs();

    let transform = &mut camera_query.1;
    let dist = transform.translation.distance(sun_pos);
    let dir = (sun_pos - transform.translation).normalize_or_zero();

    // Zooming in
    if keys.pressed(KeyCode::ArrowUp) {
        if dist - zoom_dist > MIN_ZOOM_DISTANCE {
            transform.translation += dir * zoom_dist;
        }
    }

    // Zooming out
    if keys.pressed(KeyCode::ArrowDown) {
        if dist + zoom_dist < MAX_ZOOM_DISTANCE {
            transform.translation -= dir * zoom_dist;
        }
    }

    // Rotating left
    if keys.pressed(KeyCode::ArrowLeft) {
        transform.rotate_around(sun_pos, Quat::from_rotation_y(-rot_angle));
    }

    // Rotating right
    if keys.pressed(KeyCode::ArrowRight) {
        transform.rotate_around(sun_pos, Quat::from_rotation_y(rot_angle));
    }
}
