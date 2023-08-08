use bevy::math::Vec3Swizzles;
use bevy::prelude::*;
use crate::movement::linear_movement::components::LinearMovementComponent;

pub fn linear_movement_added(
    mut query: Query<(&mut Transform, &LinearMovementComponent), Changed<LinearMovementComponent>>,
) {
    for (mut transform, movement) in query.iter_mut() {
        let direction_vector_norm = (movement.to - transform.translation).normalize();
        if direction_vector_norm.x > 0.0 {
            transform.rotation = Quat::from_rotation_y(0.0);
        } else {
            transform.rotation = Quat::from_rotation_y(std::f32::consts::PI);
        }
    }
}

pub fn linear_movement_executor(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform, &mut LinearMovementComponent)>,
    time: Res<Time>,
) {
    for (entity, mut transform, mut movement) in query.iter_mut() {
        let delta = time.delta_seconds();
        let direction_vector_norm = (movement.to - transform.translation)
            .xy()
            .extend(0.0)
            .normalize();
        transform.translation += direction_vector_norm * movement.velocity * delta;
        if transform.translation.distance(movement.to) < 0.5 {
            commands.entity(entity).remove::<LinearMovementComponent>();
        }
    }
}
