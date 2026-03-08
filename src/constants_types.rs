use bevy::prelude::Vec3;
use bevy::prelude::*;

pub const SUN_POSITION: Vec3 = Vec3::new(0.0, 2.0, 0.0);

pub const MIN_ZOOM_DISTANCE: f32 = 2.0;
pub const MAX_ZOOM_DISTANCE: f32 = 50.0;
pub const START_SIM_SPEED: f32 = 2.0;

pub const EARTH_MOON_DIST_AU: f32 = 10_000. * 0.00257; // 10k for scaling

#[derive(Resource)]
pub struct SimulationSpeed(pub f32);

#[derive(Component)]
pub struct ControlText;

impl Default for SimulationSpeed {
    fn default() -> Self {
        SimulationSpeed(START_SIM_SPEED)
    }
}
