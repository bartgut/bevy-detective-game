use bevy::prelude::*;
use crate::movement::sin_movement::components::SinMovementComponent;

/// y = Amplitude * sin(Frequency * x)
pub fn sin_movement_executor(
    mut query: Query<(Entity, &mut Transform, &SinMovementComponent)>,
    time: Res<Time>,
) {
    for (entity, mut transform, movement) in query.iter_mut() {
        let delta = time.delta_seconds();
        let direction_vector_norm = (movement.to - transform.translation.xy()).normalize();

        transform.translation +=
            direction_vector_norm.extend(0.0) * movement.velocity.extend(0.0) * delta;

        let curr_x = transform.translation.x.clone();

        transform.translation.y = movement.amplitude * (movement.frequency * curr_x).sin()
    }
}
