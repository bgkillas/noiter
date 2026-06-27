use crate::chunk_map::VoxelChunkMap;
use crate::{CHUNK_HEIGHT, CHUNK_WIDTH, PIXEL_SCALE};
use avian2d::parry::math::IVector;
use avian2d::prelude::{Collider, RigidBody};
use bevy::prelude::{Commands, Component, Query, ResMut, Transform, With};
#[derive(Component)]
pub struct ChunkCollider;
pub fn update_colliders(
    mut world: ResMut<VoxelChunkMap>,
    mut commands: Commands,
    mut colliders: Query<&mut Collider, With<ChunkCollider>>,
) {
    world.update_colliders(&mut commands, &mut colliders);
}
impl VoxelChunkMap {
    pub fn update_colliders(
        &mut self,
        commands: &mut Commands,
        colliders: &mut Query<&mut Collider, With<ChunkCollider>>,
    ) {
        for (i, chunk) in self.chunks.iter_mut() {
            if !chunk.voxels_modified {
                continue;
            }
            chunk.voxels_modified = false;
            if chunk.voxels == 0 {
                if let Some(ent) = chunk.collider {
                    commands.entity(ent).despawn();
                }
                chunk.collider = None;
                continue;
            }
            let collider = Collider::from(chunk.shape.clone());
            if let Some(e) = chunk.collider {
                *colliders.get_mut(e).unwrap() = collider;
            } else {
                let base_vector = IVector::new(
                    (CHUNK_WIDTH.strict_cast::<u32>() * i.x.strict_cast::<u32>()).cast_signed(),
                    (CHUNK_HEIGHT.strict_cast::<u32>() * i.y.strict_cast::<u32>()).cast_signed(),
                );
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
    }
}
