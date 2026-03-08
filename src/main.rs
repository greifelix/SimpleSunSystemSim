use bevy::prelude::*;

mod constants_types;
mod planets;
mod user_controls;
mod world_setup;

use constants_types::{ControlText, SimulationSpeed};
use planets::{orbit, planet_setup, sun_setup};
use user_controls::{CameraTarget, camera_zoomer, handle_simulation_speed, planet_selection};
use world_setup::{background_setup, environment_setup};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(SimulationSpeed::default())
        .insert_resource(CameraTarget::default())
        .add_systems(
            Startup,
            (environment_setup, planet_setup, sun_setup, background_setup),
        )
        .add_systems(
            Update,
            (
                orbit,
                camera_zoomer,
                planet_selection,
                handle_simulation_speed,
            ),
        )
        .run();
}
