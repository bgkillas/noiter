use crate::chunk_map::ChunkMap;
use crate::{CHUNK_HEIGHT, CHUNK_WIDTH, PIXEL_SCALE};
use avian2d::math::Vector;
use avian2d::parry::math::IVector;
use avian2d::prelude::{Collider, RigidBody};
use bevy::math::Vec3;
use bevy::prelude::{Commands, ResMut, Transform};
pub fn update_colliders(world_ref: ResMut<ChunkMap>, mut commands: Commands) {
    let world = world_ref.into_inner();
    for (i, chunk) in world.chunks.iter() {
        if !world.modified[i] {
            continue;
        }
        let mut vec = Vec::<IVector>::with_capacity(CHUNK_WIDTH * CHUNK_HEIGHT);
        world.modified[i] = false;
        let base_vector = IVector::new(
            (CHUNK_WIDTH.strict_cast::<u32>() * i.x.strict_cast::<u32>()).cast_signed(),
            (CHUNK_HEIGHT.strict_cast::<u32>() * i.y.strict_cast::<u32>()).cast_signed(),
        );
        for (i, _) in chunk.cells.iter_enumerate() {
            vec.push(IVector::new(i.x.strict_cast(), i.y.strict_cast()));
        }
        let ent = commands
            .spawn((
                RigidBody::Static,
                Collider::voxels(Vector::splat(1.0), &vec),
                Transform::from_xyz(
                    PIXEL_SCALE * base_vector.x as f32,
                    PIXEL_SCALE * base_vector.y as f32,
                    0.0,
                )
                .with_scale(Vec3::splat(PIXEL_SCALE)),
            ))
            .id();
        world.collider_entities[i] = Some(ent);
        return;
    }
}
