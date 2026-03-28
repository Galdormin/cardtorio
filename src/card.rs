//! Define the card behaviour

use bevy::prelude::*;

use crate::camera::MainCamera;

pub struct CardPlugin;

impl Plugin for CardPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, on_card_added);
    }
}

/// Card carker component
#[derive(Component, Debug)]
pub struct Card;

fn on_card_added(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    cards: Query<Entity, Added<Card>>,
) {
    for card in cards {
        commands
            .entity(card)
            .insert((
                Mesh3d(meshes.add(Plane3d::default().mesh().size(0.62, 1.))),
                MeshMaterial3d(materials.add(Color::srgb(0.85, 0.40, 0.40))),
            ))
            .observe(update_height_on::<Pointer<Over>>(0.2))
            .observe(update_height_on::<Pointer<Out>>(0.01))
            .observe(move_on_drag);
    }
}

// Observers
fn update_height_on<E: EntityEvent>(
    new_height: f32,
) -> impl Fn(On<E>, Query<&mut Transform, With<Card>>) {
    move |event, mut cards| {
        if let Ok(mut card) = cards.get_mut(event.event_target()) {
            card.translation.y = new_height;
        }
    }
}

fn move_on_drag(
    event: On<Pointer<Drag>>,
    mut cards: Query<&mut Transform, With<Card>>,
    camera: Single<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    let (camera, camera_transform) = *camera;

    let Ok(mut card) = cards.get_mut(event.entity) else {
        return;
    };

    let current_pos = event.pointer_location.position;
    let prev_pos = current_pos - event.delta;

    let plane_origin = Vec3::new(0.0, card.translation.y, 0.0);
    let plane = InfinitePlane3d::new(Vec3::Y);

    let Ok(current_ray) = camera.viewport_to_world(camera_transform, current_pos) else {
        return;
    };
    let Ok(prev_ray) = camera.viewport_to_world(camera_transform, prev_pos) else {
        return;
    };

    let Some(current_dist) = current_ray.intersect_plane(plane_origin, plane) else {
        return;
    };
    let Some(prev_dist) = prev_ray.intersect_plane(plane_origin, plane) else {
        return;
    };

    let world_delta = current_ray.get_point(current_dist) - prev_ray.get_point(prev_dist);
    card.translation.x += world_delta.x;
    card.translation.z += world_delta.z;
}
