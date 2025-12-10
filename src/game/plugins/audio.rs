use bevy::prelude::*;
use crate::game::prelude::*;

pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(play_collition_sound);
    }
}

fn play_collition_sound(
    _collided: On<BallCollided>,
    mut commands: Commands,
    sound: Res<CollisionSound>
) {
    commands.spawn((AudioPlayer(sound.clone()), PlaybackSettings::DESPAWN));
}