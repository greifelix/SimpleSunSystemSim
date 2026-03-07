use bevy::{
    color::palettes::basic::{RED, SILVER},
    prelude::*,
};

mod planets;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (environment_setup, planet_setup))
        .add_systems(Update, orbit)
        .run();
}

fn environment_setup(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0., 20.0, 30.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    commands.spawn((
        PointLight {
            shadows_enabled: true,
            intensity: 10_000_000.,
            range: 100.0,
            shadow_depth_bias: 0.2,
            ..default()
        },
        Transform::from_xyz(8.0, 16.0, 8.0),
    ));
}

#[derive(Component)]
struct Planet {
    focal: f32,
    short_axis: f32,
    long_axis: f32,
    theta: f32,
}

impl Planet {
    fn new(focal: f32, short_axis: f32, long_axis: f32, angle_start: f32) -> Self {
        Self {
            focal,
            short_axis,
            long_axis,
            theta: angle_start,
        }
    }
}

#[derive(Component)]
struct Star;

fn planet_setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    // Sun
    commands.spawn((
        Mesh3d(meshes.add(Sphere::new(planets::scale_sun_radius(
            planets::SUN_EXACT_RADIUS,
        )))),
        MeshMaterial3d(materials.add(Color::from(RED))),
        Star,
        Transform::from_translation(Vec3::new(0.0, 2.0, 0.0)),
    ));

    for p in planets::SOLAR_SYSTEM_PLANETS {
        commands.spawn((
            Mesh3d(meshes.add(Sphere::new(planets::scale_radius(p.exact_radius)))),
            MeshMaterial3d(materials.add(p.color)),
            Planet::new(p.focal, p.short_axis, p.long_axis, p.angle_start),
            Transform::from_translation(Vec3::new(0.0, 2.0, 0.0)),
        ));
    }

    // ground plane
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(50.0, 50.0).subdivisions(0))),
        MeshMaterial3d(materials.add(Color::from(SILVER))),
    ));
}

fn orbit(mut query: Query<(&mut Transform, &mut Planet)>, time: Res<Time>) {
    let dt = time.delta_secs();
    let speed = 2.0;

    for (mut transform, mut p) in &mut query {
        // NOTE: This needs double checking
        let radius = get_planet_radius(&p);
        let h = p.short_axis / p.long_axis.sqrt();
        let dtheta = (h / radius.powi(2)) * speed * dt;
        p.theta += dtheta;

        let (x, z) = get_planet_pos(&p);

        transform.translation.x = x;
        transform.translation.z = z;
    }
}

fn get_planet_radius(planet: &Planet) -> f32 {
    let p = planet.short_axis.powi(2) / planet.long_axis;
    let eps = planet.focal / planet.long_axis;
    p / (1. + eps * planet.theta.cos())
}

fn get_planet_pos(planet: &Planet) -> (f32, f32) {
    let r = get_planet_radius(planet);
    let x = r * planet.theta.cos();
    let z = r * planet.theta.sin();

    (x, z)
}
