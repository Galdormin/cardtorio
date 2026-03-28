use bevy::prelude::*;

use crate::{
    camera::{CameraPlugin, MainCameraBundle},
    card::{Card, CardPlugin},
};

mod camera;
mod card;

fn main() -> AppExit {
    let mut app = App::new();

    // Bevy Plugins
    app.add_plugins((DefaultPlugins, MeshPickingPlugin));

    // 3rd party plugins

    // Crate plugins
    app.add_plugins((CameraPlugin, CardPlugin));

    // Setup world
    app.add_systems(Startup, setup);

    app.run()
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Camera
    commands.spawn((
        MainCameraBundle::new(),
        Transform::from_xyz(0., 10., 3.).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // Plane
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(30.0, 30.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.3, 0.5, 0.3))),
    ));

    // Light
    commands.spawn((
        DirectionalLight {
            illuminance: light_consts::lux::OVERCAST_DAY,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(0.0, 10.0, 2.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // Cards
    commands.spawn((Card, Transform::from_xyz(0., 0.01, 0.)));
    commands.spawn((Card, Transform::from_xyz(2., 0.01, 0.)));
    commands.spawn((Card, Transform::from_xyz(-2., 0.01, 0.)));
}
