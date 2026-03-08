use crate::SimulationSpeed;
use crate::constants;
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
    pub texture: &'static str,
}

/// Scales the exact radius (in Earth radii)
pub fn scale_radius(exact_radius: f32) -> f32 {
    exact_radius.powf(0.3) * 0.15
}

/// Scales the exact radius of the sun for visualization
pub fn scale_sun_radius(exact_radius: f32) -> f32 {
    exact_radius.powf(0.3) * 0.15
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
        texture: "2k_mercury.jpg",
    },
    PlanetParams {
        _name: "Venus",
        focal: 0.0049,
        short_axis: 0.7233,
        long_axis: 0.7233,
        angle_start: 1.2,
        exact_radius: 0.950,
        texture: "2k_venus_surface.jpg",
    },
    PlanetParams {
        _name: "Earth",
        focal: 0.0167,
        short_axis: 0.9999,
        long_axis: 1.0000,
        angle_start: 2.5,
        exact_radius: 1.000,
        texture: "2k_earth_daymap.jpg",
    },
    PlanetParams {
        _name: "Mars",
        focal: 0.1423,
        short_axis: 1.5174,
        long_axis: 1.5237,
        angle_start: 4.0,
        exact_radius: 0.532,
        texture: "2k_mars.jpg",
    },
    PlanetParams {
        _name: "Jupiter",
        focal: 0.2515,
        short_axis: 5.1982,
        long_axis: 5.2028,
        angle_start: 1.5,
        exact_radius: 10.973,
        texture: "2k_jupiter.jpg",
    },
    PlanetParams {
        _name: "Saturn",
        focal: 0.5345,
        short_axis: 9.5231,
        long_axis: 9.5388,
        angle_start: 3.2,
        exact_radius: 9.140,
        texture: "2k_saturn.jpg",
    },
    PlanetParams {
        _name: "Uranus",
        focal: 0.8950,
        short_axis: 19.1645,
        long_axis: 19.1914,
        angle_start: 0.8,
        exact_radius: 3.981,
        texture: "2k_uranus.jpg",
    },
    PlanetParams {
        _name: "Neptune",
        focal: 0.2588,
        short_axis: 30.0611,
        long_axis: 30.0689,
        angle_start: 5.1,
        exact_radius: 3.865,
        texture: "2k_neptune.jpg",
    },
];

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
                Sphere::new(scale_sun_radius(SUN_EXACT_RADIUS))
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
        Transform::from_translation(constants::SUN_POSITION)
            .with_rotation(Quat::from_rotation_x(-std::f32::consts::PI / 2.)),
    ));

    commands.spawn((
        PointLight {
            shadows_enabled: true,
            intensity: 100_000.,
            ..default()
        },
        Transform::from_translation(constants::SUN_POSITION),
    ));
}

pub fn planet_setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    asset_server: Res<AssetServer>,
) {
    for (i, p) in SOLAR_SYSTEM_PLANETS.iter().enumerate() {
        commands.spawn((
            Mesh3d(meshes.add(Sphere::new(scale_radius(p.exact_radius)).mesh().uv(128, 64))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color_texture: Some(asset_server.load(p.texture)),
                ..default()
            })),
            Planet::new(p.focal, p.short_axis, p.long_axis, p.angle_start, i),
            Transform::from_translation(constants::SUN_POSITION)
                .with_rotation(Quat::from_rotation_x(-std::f32::consts::PI / 2.)),
        ));
    }
}

pub fn orbit(
    mut query: Query<(&mut Transform, &mut Planet)>,
    time: Res<Time>,
    speed: Res<SimulationSpeed>,
) {
    let dt = time.delta_secs();

    for (mut transform, mut p) in &mut query {
        let radius = get_planet_radius(&p);
        let h = p.short_axis / p.long_axis.sqrt();
        let dtheta = (h / radius.powi(2)) * speed.0 * dt;
        p.theta += dtheta;

        let (x, z) = get_planet_pos(&p);

        transform.translation.x = x;
        transform.translation.z = z;
    }
}

pub fn get_planet_radius(planet: &Planet) -> f32 {
    let p = planet.short_axis.powi(2) / planet.long_axis;
    let eps = planet.focal / planet.long_axis;
    p / (1. + eps * planet.theta.cos())
}

pub fn get_planet_pos(planet: &Planet) -> (f32, f32) {
    let r = get_planet_radius(planet);
    let x = r * planet.theta.cos();
    let z = r * planet.theta.sin();

    (x, z)
}
