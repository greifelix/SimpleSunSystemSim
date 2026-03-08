use bevy::prelude::*;

mod constants;
mod planets;
mod user_controls;

use constants::START_SIM_SPEED;
use planets::{orbit, planet_setup, sun_setup};
use user_controls::{CameraTarget, camera_zoomer, handle_simulation_speed, planet_selection};

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

#[derive(Resource)]
pub struct SimulationSpeed(pub f32);

#[derive(Component)]
pub struct ControlText;

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

    commands.insert_resource(GlobalAmbientLight {
        brightness: 1000.0,
        ..default()
    });

    commands.spawn((
        Text::new("Controls:\nArrow Keys: Move/Rotate Camera\nSpace + L/R Arrow: Rotate Up/Down\n+/-: Adjust Simulation Speed (Current: 2.00x)\n0-8: Select Focus (0=Sun, 1-8=Planets)\nTarget: Sun"),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(12.0),
            left: Val::Px(12.0),
            ..default()
        },
        ControlText
    ));
}

fn background_setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    asset_server: Res<AssetServer>,
) {
    let star_texture = asset_server.load("2k_stars_milky_way.jpg");
    let plane_mesh = meshes.add(Plane3d::default().mesh().size(50.0, 50.0));
    let plane_material = materials.add(StandardMaterial {
        base_color_texture: Some(star_texture.clone()),
        emissive: bevy::color::LinearRgba::rgb(1.5, 1.5, 1.5),
        emissive_texture: Some(star_texture),
        reflectance: 0.0,
        ..default()
    });

    for x in -2..2 {
        for z in -2..2 {
            let rotation_steps = (x as i32 * 3 + z as i32 * 7).rem_euclid(4);
            let rotation =
                Quat::from_rotation_y(rotation_steps as f32 * std::f32::consts::FRAC_PI_2);
            commands.spawn((
                Mesh3d(plane_mesh.clone()),
                MeshMaterial3d(plane_material.clone()),
                Transform::from_xyz((x as f32 * 50.0) + 25.0, 0.0, (z as f32 * 50.0) + 25.0)
                    .with_rotation(rotation),
            ));
        }
    }
}
