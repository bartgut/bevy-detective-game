use bevy::math::Vec3Swizzles;
use bevy::prelude::*;
use crate::movement::linear_movement::components::Linear2DMovementComponent;

pub fn linear_2d_movement_executor(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform, &mut Linear2DMovementComponent)>,
    time: Res<Time>,
) {
    for (entity, mut transform, movement) in query.iter_mut() {
        let delta = time.delta_seconds();
        let direction_vector_norm = (movement.to - transform.translation.xy()).normalize();

        rotate_depending_on_direction(&mut transform, &direction_vector_norm);

        transform.translation +=
            direction_vector_norm.extend(0.0) * movement.velocity.extend(0.0) * delta;

        if direction_vector_norm.x > 0.0 {
            if transform.translation.x > movement.to.x {
                transform.translation = movement.to.extend(0.0)
            }
        } else {
            if transform.translation.x < movement.to.x {
                transform.translation = movement.to.extend(0.0)
            }
        }
        if transform.translation.xy().distance(movement.to) == 0.0 {
            commands
                .entity(entity)
                .remove::<Linear2DMovementComponent>();
        }
    }
}

pub fn linear_2d_movement_stop(
    mut commands: Commands,
    query: Query<Entity, With<Linear2DMovementComponent>>,
) {
    for entity in query.iter() {
        commands
            .entity(entity)
            .remove::<Linear2DMovementComponent>();
    }
}
fn rotate_depending_on_direction(transform: &mut Transform, direction_vector_norm: &Vec2) {
    if direction_vector_norm.x > 0.0 {
        transform.rotation = Quat::from_rotation_y(0.0);
    } else {
        transform.rotation = Quat::from_rotation_y(std::f32::consts::PI);
    }
}
