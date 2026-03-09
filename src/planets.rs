use std::f32;

use crate::SimulationSpeed;
use crate::constants_types;

use bevy::prelude::*;

#[derive(Component)]
pub struct Planet {
    pub focal: f32,
    pub short_axis: f32,
    pub long_axis: f32,
    pub theta: f32,
    pub index: usize,
}

#[derive(Component)]
pub struct Star;

#[derive(Component)]
pub struct Moon;

#[derive(Component)]
pub struct Earth;

impl Planet {
    pub fn new(
        focal: f32,
        short_axis: f32,
        long_axis: f32,
        angle_start: f32,
        index: usize,
    ) -> Self {
        Self {
            focal,
            short_axis,
            long_axis,
            theta: angle_start,
            index,
        }
    }
}

pub struct PlanetParams {
    pub _name: &'static str,
    pub focal: f32,
    pub short_axis: f32,
    pub long_axis: f32,
    pub angle_start: f32,
    pub exact_radius: f32,
    pub inclination_angle: f32,
    pub texture: &'static str,
}

/// Scales the exact radius (in Earth radii)
pub fn scale_radius(exact_radius: f32) -> f32 {
    exact_radius.powf(0.3) * 0.1
}

/// The exact radius of the Sun in Earth radii.
pub const SUN_EXACT_RADIUS: f32 = 109.2;
pub const MOON_RADIUS: f32 = 0.1 * 0.27270444200282534; // 0.1 for scaling

pub const PLANET_ORBIT_TIMES: [f32; 8] = [
    88.0, 224.7, 365.25, 687.0, 4331.0, 10747.0, 30589.0, 59800.0,
];

pub const PLANET_ROTATION_TIMES: [f32; 8] = [58.6, -243.0, 1.0, 1.03, 0.41, 0.45, -0.72, 0.67];

pub const MOON_ORBIT_TIME: f32 = 27.3;
pub const _MOON_ROTATION_TIME: f32 = 27.3;

pub const SOLAR_SYSTEM_PLANETS: &[PlanetParams] = &[
    PlanetParams {
        _name: "Mercury",
        focal: 0.0796,
        short_axis: 0.3787,
        long_axis: 0.3871,
        angle_start: 0.0,
        exact_radius: 0.383,
        inclination_angle: 0.034 * f32::consts::PI / 180.0,
        texture: "2k_mercury.jpg",
    },
    PlanetParams {
        _name: "Venus",
        focal: 0.0049,
        short_axis: 0.7233,
        long_axis: 0.7233,
        angle_start: 1.2,
        exact_radius: 0.950,
        inclination_angle: 177.36 * f32::consts::PI / 180.0,
        texture: "2k_venus_surface.jpg",
    },
    PlanetParams {
        _name: "Earth",
        focal: 0.0167,
        short_axis: 0.9999,
        long_axis: 1.0000,
        angle_start: 2.5,
        exact_radius: 1.000,
        inclination_angle: 23.44 * f32::consts::PI / 180.0,
        texture: "2k_earth_daymap.jpg",
    },
    PlanetParams {
        _name: "Mars",
        focal: 0.1423,
        short_axis: 1.5174,
        long_axis: 1.5237,
        angle_start: 4.0,
        exact_radius: 0.532,
        inclination_angle: 25.19 * f32::consts::PI / 180.0,
        texture: "2k_mars.jpg",
    },
    PlanetParams {
        _name: "Jupiter",
        focal: 0.2515,
        short_axis: 5.1982,
        long_axis: 5.2028,
        angle_start: 1.5,
        exact_radius: 10.973,
        inclination_angle: 3.13 * f32::consts::PI / 180.0,
        texture: "2k_jupiter.jpg",
    },
    PlanetParams {
        _name: "Saturn",
        focal: 0.5345,
        short_axis: 9.5231,
        long_axis: 9.5388,
        angle_start: 3.2,
        exact_radius: 9.140,
        inclination_angle: 26.73 * f32::consts::PI / 180.0,
        texture: "2k_saturn.jpg",
    },
    PlanetParams {
        _name: "Uranus",
        focal: 0.8950,
        short_axis: 19.1645,
        long_axis: 19.1914,
        angle_start: 0.8,
        exact_radius: 3.981,
        inclination_angle: 97.77 * f32::consts::PI / 180.0,
        texture: "2k_uranus.jpg",
    },
    PlanetParams {
        _name: "Neptune",
        focal: 0.2588,
        short_axis: 30.0611,
        long_axis: 30.0689,
        angle_start: 5.1,
        exact_radius: 3.865,
        inclination_angle: 28.32 * f32::consts::PI / 180.0,
        texture: "2k_neptune.jpg",
    },
];

pub fn moon_setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    asset_server: Res<AssetServer>,
) {
    let moon_texture = asset_server.load("2k_moon.jpg");
    commands.spawn((
        Mesh3d(meshes.add(Sphere::new(scale_radius(MOON_RADIUS)).mesh().uv(128, 64))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color_texture: Some(moon_texture),
            ..default()
        })),
        Transform::from_translation(constants_types::SUN_POSITION)
            .with_rotation(Quat::from_rotation_x(-std::f32::consts::PI / 2.)),
        Moon,
    ));
}

pub fn sun_setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    asset_server: Res<AssetServer>,
) {
    let sun_texture = asset_server.load("2k_sun.jpg");
    commands.spawn((
        Mesh3d(
            meshes.add(
                Sphere::new(scale_radius(SUN_EXACT_RADIUS))
                    .mesh()
                    .uv(128, 64),
            ),
        ),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color_texture: Some(sun_texture.clone()),
            emissive: bevy::color::LinearRgba::rgb(5.0, 5.0, 5.0),
            emissive_texture: Some(sun_texture),
            reflectance: 0.0,
            ..default()
        })),
        Star,
        Transform::from_translation(constants_types::SUN_POSITION)
            .with_rotation(Quat::from_rotation_x(-std::f32::consts::PI / 2.)),
    ));

    commands.spawn((
        PointLight {
            shadows_enabled: true,
            intensity: 100_000.,
            ..default()
        },
        Transform::from_translation(constants_types::SUN_POSITION),
    ));
}

pub fn planet_setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    asset_server: Res<AssetServer>,
) {
    for (i, p) in SOLAR_SYSTEM_PLANETS.iter().enumerate() {
        let mut x = commands.spawn((
            Mesh3d(meshes.add(Sphere::new(scale_radius(p.exact_radius)).mesh().uv(128, 64))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color_texture: Some(asset_server.load(p.texture)),
                ..default()
            })),
            Planet::new(p.focal, p.short_axis, p.long_axis, p.angle_start, i),
            Transform::from_translation(constants_types::SUN_POSITION).with_rotation(
                Quat::from_rotation_x(p.inclination_angle)
                    * Quat::from_rotation_x(-std::f32::consts::PI / 2.),
            ),
        ));

        if p._name == "Earth" {
            x.insert(Earth);
        }

        if p._name == "Saturn" {
            let ring_texture = asset_server.load("2k_saturn_ring_alpha.png");
            let inner_radius = 1.11 * scale_radius(p.exact_radius);
            let outer_radius = 2.27 * scale_radius(p.exact_radius);

            x.with_children(|parent| {
                parent.spawn((
                    Mesh3d(meshes.add(Extrusion::new(
                        Annulus::new(inner_radius, outer_radius),
                        0.001,
                    ))),
                    MeshMaterial3d(materials.add(StandardMaterial {
                        base_color_texture: Some(ring_texture),
                        alpha_mode: AlphaMode::Blend,
                        cull_mode: None,
                        ..default()
                    })),
                ));
            });
        }
    }
}

pub fn moon_orbit(
    mut moon_query: Single<(&mut Transform,), (With<Moon>, Without<Earth>)>,
    earth_query: Single<(&Transform,), (With<Earth>, Without<Moon>)>,
    time: Res<Time>,
    speed: Res<SimulationSpeed>,
    mut moon_theta: Local<f32>,
) {
    let moon_transform = &mut moon_query.0;
    let earth_transform = earth_query.0;
    let dt = time.delta_secs() / (3600. * 24.);

    let speed_factor = speed.0 * (2. * f32::consts::PI / MOON_ORBIT_TIME);
    *moon_theta = (*moon_theta - dt * speed_factor) % (2. * f32::consts::PI);

    moon_transform.translation.x = earth_transform.translation.x
        + scale_radius(constants_types::EARTH_MOON_DIST_AU) * (*moon_theta).cos();
    moon_transform.translation.z = earth_transform.translation.z
        + scale_radius(constants_types::EARTH_MOON_DIST_AU) * (*moon_theta).sin();
}

pub fn orbit(
    mut query: Query<(&mut Transform, &mut Planet)>,
    time: Res<Time>,
    speed: Res<SimulationSpeed>,
) {
    let dt = time.delta_secs() / (3600. * 24.);

    for (mut transform, mut p) in &mut query {
        let dtheta = -(2. * f32::consts::PI / PLANET_ORBIT_TIMES[p.index]) * speed.0 * dt;
        p.theta += dtheta;
        p.theta = p.theta % (2. * f32::consts::PI);

        let (x, z) = get_planet_cartesian_pos(&p);

        transform.translation.x = x;
        transform.translation.z = z;
    }
}

pub fn get_planet_polar_radius(planet: &Planet) -> f32 {
    let p = planet.short_axis.powi(2) / planet.long_axis;
    let eps = planet.focal / planet.long_axis;
    let polar_radius = p / (1. + eps * planet.theta.cos());

    // Scale the radius such that distances are more managable
    2. * polar_radius.powf(0.5)
}

pub fn get_planet_cartesian_pos(planet: &Planet) -> (f32, f32) {
    let r = get_planet_polar_radius(planet);
    let x = r * planet.theta.cos();
    let z = r * planet.theta.sin();

    (x, z)
}

pub fn earth_rotation(
    mut earth_query: Single<&mut Transform, With<Earth>>,
    time: Res<Time>,
    speed: Res<SimulationSpeed>,
    mut angle: Local<f32>,
) {
    let dt = time.delta_secs() / (3600. * 24.);
    let speed_factor = speed.0 * (2. * f32::consts::PI / PLANET_ROTATION_TIMES[2]); // Earth is index 2
    *angle = (*angle + dt * speed_factor) % (2. * f32::consts::PI);
    let inclination_angle = 23.44 * f32::consts::PI / 180.0;

    earth_query.rotation = Quat::from_rotation_x(inclination_angle)
        * Quat::from_rotation_x(-std::f32::consts::PI / 2.)
        * Quat::from_rotation_z(*angle);
}
