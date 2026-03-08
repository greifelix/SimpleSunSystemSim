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
    let space_mesh = meshes.add(Sphere::new(150.0).mesh().uv(128, 64));
    let sky_material = materials.add(StandardMaterial {
        base_color_texture: Some(star_texture.clone()),
        emissive: bevy::color::LinearRgba::rgb(1.5, 1.5, 1.5),
        emissive_texture: Some(star_texture),
        reflectance: 0.0,
        // unlit: true,
        ..default()
    });

    commands.spawn((
        Mesh3d(space_mesh),
        MeshMaterial3d(sky_material),
        // A negative scale on one axis flips the normals so the texture is visible from the inside
        Transform::from_scale(Vec3::new(-1.0, 1.0, 1.0)),
    ));
}
