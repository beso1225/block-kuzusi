use bevy::prelude::*;
use crate::game::prelude::*;

pub fn apply_velocity(mut query: Query<(&mut Transform, &Velocity)>, time: Res<Time>, game_state: Res<GameState>) {
    if *game_state != GameState::Playing { return; }
    for (mut transform, velocity) in &mut query {
        transform.translation.x += velocity.x * time.delta_secs();
        transform.translation.y += velocity.y * time.delta_secs();
    }
}