use crate::chunk_map::ChunkMap;
use crate::{CHUNK_HEIGHT, CHUNK_WIDTH, PIXEL_SCALE};
use avian2d::parry::math::IVector;
use avian2d::prelude::{Collider, RigidBody};
use bevy::prelude::{Commands, Component, ResMut, Transform};
#[derive(Component)]
pub struct ChunkCollider;
pub fn update_colliders(world_ref: ResMut<ChunkMap>, mut commands: Commands) {
    let world = world_ref.into_inner();
    for (i, chunk) in world.chunks.iter_mut() {
        if !chunk.voxels_modified {
            continue;
        }
        chunk.voxels_modified = false;
        if let Some(ent) = chunk.collider {
            commands.entity(ent).despawn();
        }
        if chunk.voxels == 0 {
            chunk.collider = None;
            continue;
        }
        let base_vector = IVector::new(
            (CHUNK_WIDTH.strict_cast::<u32>() * i.x.strict_cast::<u32>()).cast_signed(),
            (CHUNK_HEIGHT.strict_cast::<u32>() * i.y.strict_cast::<u32>()).cast_signed(),
        );
        let collider = Collider::from(chunk.shape.clone());
        let ent = commands.spawn((
            RigidBody::Static,
            collider,
            Transform::from_xyz(
                PIXEL_SCALE * base_vector.x as f32,
                PIXEL_SCALE * base_vector.y as f32,
                0.0,
            ),
            ChunkCollider,
        ));
        chunk.collider = Some(ent.id());
    }
}
