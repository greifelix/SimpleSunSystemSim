use bevy::{
    color::palettes::basic::{RED, SILVER},
    prelude::*,
};

mod camera_helpers;
mod constants;
mod planets;

use camera_helpers::camera_zoomer;
use constants::START_SIM_SPEED;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(SimulationSpeed::default())
        .add_systems(Startup, (environment_setup, planet_setup))
        .add_systems(Update, (orbit, camera_zoomer, update_simulation_speed))
        .run();
}

#[derive(Resource)]
struct SimulationSpeed(f32);

#[derive(Component)]
struct ControlText;

#[derive(Component)]
struct Planet {
    focal: f32,
    short_axis: f32,
    long_axis: f32,
    theta: f32,
}

#[derive(Component)]
struct Star;

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

impl Default for SimulationSpeed {
    fn default() -> Self {
        SimulationSpeed(START_SIM_SPEED)
    }
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

    commands.spawn((
        Text::new("Controls:\nArrow Keys: Move/Rotate Camera\n+/-: Adjust Simulation Speed (Current: 2.0x)"),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(12.0),
            left: Val::Px(12.0),
            ..default()
        },
        ControlText
    ));
}

fn update_simulation_speed(
    keys: Res<ButtonInput<KeyCode>>,
    mut speed: ResMut<SimulationSpeed>,
    mut text_query: Query<&mut Text, With<ControlText>>,
) {
    let mut changed = false;
    if keys.just_pressed(KeyCode::Equal)
        || keys.just_pressed(KeyCode::NumpadAdd)
        || keys.just_pressed(KeyCode::BracketRight)
    {
        speed.0 += 1.0;
        changed = true;
    }
    if keys.just_pressed(KeyCode::Minus)
        || keys.just_pressed(KeyCode::NumpadSubtract)
        || keys.just_pressed(KeyCode::Slash)
    {
        speed.0 -= 1.0;
        changed = true;
    }

    if changed {
        for mut text in &mut text_query {
            *text = Text::new(format!(
                "Controls:\nArrow Keys: Move/Rotate Camera\n+/-: Adjust Simulation Speed (Current: {:.1}x)",
                speed.0
            ));
        }
    }
}

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
        Transform::from_translation(constants::SUN_POSITION),
    ));

    for p in planets::SOLAR_SYSTEM_PLANETS {
        commands.spawn((
            Mesh3d(meshes.add(Sphere::new(planets::scale_radius(p.exact_radius)))),
            MeshMaterial3d(materials.add(p.color)),
            Planet::new(p.focal, p.short_axis, p.long_axis, p.angle_start),
            Transform::from_translation(constants::SUN_POSITION),
        ));
    }

    // ground plane
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(50.0, 50.0).subdivisions(0))),
        MeshMaterial3d(materials.add(Color::from(SILVER))),
    ));
}

fn orbit(
    mut query: Query<(&mut Transform, &mut Planet)>,
    time: Res<Time>,
    speed: Res<SimulationSpeed>,
) {
    let dt = time.delta_secs();

    for (mut transform, mut p) in &mut query {
        // NOTE: This needs double checking
        let radius = get_planet_radius(&p);
        let h = p.short_axis / p.long_axis.sqrt();
        let dtheta = (h / radius.powi(2)) * speed.0 * dt;
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
