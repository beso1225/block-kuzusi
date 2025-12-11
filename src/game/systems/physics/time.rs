use bevy::prelude::*;
use crate::game::prelude::*;

pub fn apply_velocity(mut query: Query<(&mut Transform, &Velocity)>, time: Res<Time>, game_state: Res<GameState>) {
    if *game_state != GameState::Playing { return; }
    for (mut transform, velocity) in &mut query {
        transform.translation.x += velocity.x * time.delta_secs();
        transform.translation.y += velocity.y * time.delta_secs();
    }
}

pub fn tick_paddle_cooldowns(time: Res<Time>, mut query: Query<&mut PaddleCooldown, With<Ball>>) {
    let dt = time.delta_secs();
    for mut cooldown in &mut query {
        cooldown.0 = (cooldown.0 - dt).max(0.0);
    }
}