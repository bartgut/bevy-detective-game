use bevy::math::vec3;
use bevy::prelude::*;
use crate::movement::linear_movement::components::LinearMovementComponent;

pub fn linear_movement_executor(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform, &mut LinearMovementComponent)>,
    time: Res<Time>,
) {
    for (entity, mut transform, mut movement) in query.iter_mut() {
        let delta = time.delta_seconds();
        let direction_vector_norm = Vec3::new(movement.to.x - transform.translation.x, movement.to.y - transform.translation.y, 0.0).normalize();
        transform.translation += direction_vector_norm * movement.velocity * delta;
        if transform.translation.distance(movement.to) < 0.5 {
            commands.entity(entity).remove::<LinearMovementComponent>();
        }
    }
}