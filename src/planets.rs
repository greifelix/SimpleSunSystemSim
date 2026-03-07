use bevy::prelude::Color;

pub struct PlanetParams {
    pub _name: &'static str,
    pub focal: f32,
    pub short_axis: f32,
    pub long_axis: f32,
    pub angle_start: f32,
    pub exact_radius: f32,
    pub color: Color,
}

/// Scales the exact radius (in Earth radii)
pub fn scale_radius(exact_radius: f32) -> f32 {
    exact_radius.powf(0.3) * 0.15
}

/// Scales the exact radius of the sun for visualization
pub fn scale_sun_radius(exact_radius: f32) -> f32 {
    exact_radius.powf(0.1) * 0.15
}

/// The exact radius of the Sun in Earth radii.
pub const SUN_EXACT_RADIUS: f32 = 109.2;

pub const SOLAR_SYSTEM_PLANETS: &[PlanetParams] = &[
    PlanetParams {
        _name: "Mercury",
        focal: 0.0796,
        short_axis: 0.3787,
        long_axis: 0.3871,
        angle_start: 0.0,
        exact_radius: 0.383,
        color: Color::srgb(0.6, 0.6, 0.6), // Gray
    },
    PlanetParams {
        _name: "Venus",
        focal: 0.0049,
        short_axis: 0.7233,
        long_axis: 0.7233,
        angle_start: 1.2,
        exact_radius: 0.950,
        color: Color::srgb(0.9, 0.8, 0.5), // Yellowish
    },
    PlanetParams {
        _name: "Earth",
        focal: 0.0167,
        short_axis: 0.9999,
        long_axis: 1.0000,
        angle_start: 2.5,
        exact_radius: 1.000,
        color: Color::srgb(0.2, 0.4, 0.8), // Blue
    },
    PlanetParams {
        _name: "Mars",
        focal: 0.1423,
        short_axis: 1.5174,
        long_axis: 1.5237,
        angle_start: 4.0,
        exact_radius: 0.532,
        color: Color::srgb(0.8, 0.3, 0.2), // Red
    },
    PlanetParams {
        _name: "Jupiter",
        focal: 0.2515,
        short_axis: 5.1982,
        long_axis: 5.2028,
        angle_start: 1.5,
        exact_radius: 10.973,
        color: Color::srgb(0.8, 0.6, 0.4), // Orange/Brown
    },
    PlanetParams {
        _name: "Saturn",
        focal: 0.5345,
        short_axis: 9.5231,
        long_axis: 9.5388,
        angle_start: 3.2,
        exact_radius: 9.140,
        color: Color::srgb(0.9, 0.8, 0.6), // Pale Gold
    },
    PlanetParams {
        _name: "Uranus",
        focal: 0.8950,
        short_axis: 19.1645,
        long_axis: 19.1914,
        angle_start: 0.8,
        exact_radius: 3.981,
        color: Color::srgb(0.5, 0.8, 0.9), // Light Blue
    },
    PlanetParams {
        _name: "Neptune",
        focal: 0.2588,
        short_axis: 30.0611,
        long_axis: 30.0689,
        angle_start: 5.1,
        exact_radius: 3.865,
        color: Color::srgb(0.2, 0.2, 0.8), // Dark Blue
    },
];
