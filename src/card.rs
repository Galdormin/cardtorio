//! Define the card behaviour

use bevy::{color::palettes::tailwind::*, prelude::*};

use crate::camera::MainCamera;

pub struct CardPlugin;

impl Plugin for CardPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, on_card_added)
            .add_systems(PostUpdate, update_card_stack);
    }
}

/// Card carker component
#[derive(Component, Debug)]
pub struct Card;

#[derive(Component, Debug, Clone, Copy)]
#[relationship(relationship_target = StackedWith)]
pub struct StackedOn(pub Entity);

#[derive(Component, Debug, Clone, Copy)]
#[relationship_target(relationship = StackedOn)]
pub struct StackedWith(Entity);

// Systems

fn on_card_added(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    cards: Query<Entity, Added<Card>>,
) {
    let normal_mat = materials.add(Color::from(RED_300));
    let hover_mat = materials.add(Color::from(CYAN_300));

    for card in cards {
        commands
            .entity(card)
            .insert((
                Mesh3d(meshes.add(Plane3d::default().mesh().size(0.62, 1.))),
                MeshMaterial3d(normal_mat.clone()),
            ))
            .observe(update_material_on::<Pointer<Over>>(hover_mat.clone()))
            .observe(update_material_on::<Pointer<Out>>(normal_mat.clone()))
            .observe(move_on_drag)
            .observe(on_drag_start)
            .observe(on_drag_end);
    }
}

fn update_card_stack(
    roots: Query<(&Transform, &StackedWith), (With<Card>, Without<StackedOn>)>,
    mut cards: Query<(&mut Transform, Option<&StackedWith>), (With<Card>, With<StackedOn>)>,
) {
    for (root_pos, root_stack) in roots {
        let mut position = *root_pos;
        let mut stack = *root_stack;
        while let Ok((mut card_pos, card_stack)) = cards.get_mut(stack.0) {
            position.translation.z += 0.2;
            position.translation.y += 0.01;
            *card_pos = position;

            stack = match card_stack {
                Some(s) => *s,
                _ => break,
            };
        }
    }
}

// Observers
fn update_material_on<E: EntityEvent>(
    new_material: Handle<StandardMaterial>,
) -> impl Fn(On<E>, Query<&mut MeshMaterial3d<StandardMaterial>>) {
    move |event, mut query| {
        if let Ok(mut material) = query.get_mut(event.event_target()) {
            material.0 = new_material.clone();
        }
    }
}

fn on_drag_start(
    event: On<Pointer<DragStart>>,
    mut commands: Commands,
    mut cards: Query<&mut Transform, With<Card>>,
) -> Result {
    let mut transform = cards.get_mut(event.entity)?;
    transform.translation.y = 0.5;

    // Remove from stack
    commands.entity(event.entity).try_remove::<StackedOn>();

    Ok(())
}

fn on_drag_end(
    event: On<Pointer<DragEnd>>,
    mut commands: Commands,
    mut cards: Query<(Entity, &mut Transform), With<Card>>,
    stacked_with: Query<&StackedWith, With<Card>>,
) -> Result {
    info!("Drag end: {}", event.entity);

    let dragged = event.entity;
    let dragged_pos = cards.get(dragged).map(|(_, t)| t.translation)?;

    // Collect the dragged card's own sub-stack (itself + cards stacked on it)
    // to avoid creating cycles when checking collisions
    let mut own_stack = vec![dragged];
    let mut cursor = dragged;
    while let Ok(sw) = stacked_with.get(cursor) {
        own_stack.push(sw.0);
        cursor = sw.0;
    }

    // Find a card to stack on (AABB check on XZ plane)
    let mut stack_top = None;
    for (entity, transform) in cards.iter() {
        if own_stack.contains(&entity) {
            continue;
        }

        let dx = (dragged_pos.x - transform.translation.x).abs();
        let dz = (dragged_pos.z - transform.translation.z).abs();
        if dx < 0.31 && dz < 0.5 {
            // Traverse to the top of this card's stack
            let mut top = entity;
            while let Ok(sw) = stacked_with.get(top) {
                top = sw.0;
            }
            stack_top = Some(top);
            break;
        }
    }

    cards.get_mut(dragged)?.1.translation.y = 0.01;

    if let Some(top) = stack_top {
        commands.entity(dragged).insert(StackedOn(top));
    }

    Ok(())
}

fn move_on_drag(
    event: On<Pointer<Drag>>,
    mut cards: Query<&mut Transform, With<Card>>,
    camera: Single<(&Camera, &GlobalTransform), With<MainCamera>>,
) -> Result {
    let (camera, camera_transform) = *camera;
    let mut card = cards.get_mut(event.entity)?;

    let current_pos = event.pointer_location.position;
    let plane_origin = Vec3::new(0.0, card.translation.y, 0.0);
    let plane = InfinitePlane3d::new(Vec3::Y);

    let current_ray = camera.viewport_to_world(camera_transform, current_pos)?;

    let current_dist = current_ray
        .intersect_plane(plane_origin, plane)
        .ok_or("Ray don't interset plane")?;

    card.translation = current_ray
        .get_point(current_dist)
        .with_y(card.translation.y);

    Ok(())
}
