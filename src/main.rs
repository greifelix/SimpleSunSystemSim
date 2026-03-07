use bevy::{
    color::palettes::basic::{BLUE, GREEN, RED, SILVER},
    prelude::*,
};

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
        Transform::from_xyz(0., 7.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
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
    center: f32,
    radius: f32,
}

#[derive(Component)]
struct Star;

fn planet_setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    commands.spawn((
        Mesh3d(meshes.add(Sphere::default())),
        MeshMaterial3d(materials.add(Color::from(RED))),
        Star,
        Transform::from_translation(Vec3::new(0., 1., 0.)),
    ));

    commands.spawn((
        Mesh3d(meshes.add(Sphere::default())),
        MeshMaterial3d(materials.add(Color::from(BLUE))),
        Planet {
            center: 0.,
            radius: 4.,
        },
        Transform::from_translation(Vec3::new(0., 1., 0.)),
    ));

    commands.spawn((
        Mesh3d(meshes.add(Sphere::default())),
        MeshMaterial3d(materials.add(Color::from(GREEN))),
        Planet {
            center: 0.,
            radius: 5.,
        },
        Transform::from_translation(Vec3::new(0., 1., 0.)),
    ));

    // ground plane
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(50.0, 50.0).subdivisions(0))),
        MeshMaterial3d(materials.add(Color::from(SILVER))),
    ));
}

fn orbit(mut query: Query<(&mut Transform, &Planet)>, time: Res<Time>) {
    let t = time.elapsed_secs();

    let speed = 2.0;

    for (mut transform, p) in &mut query {
        let x = p.center + (t * speed).cos() * p.radius;
        let z = p.center + (t * speed).sin() * p.radius;

        transform.translation.x = x;
        transform.translation.z = z;
    }
}
