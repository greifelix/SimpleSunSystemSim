use bevy::prelude::*;

use crate::ControlText;

pub fn environment_setup(mut commands: Commands) {
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

pub fn background_setup(
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
