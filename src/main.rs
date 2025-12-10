use bevy::prelude::*;

mod game;
use game::prelude::*;
use game::plugins::{
    core::CorePlugin,
    spawn::SpawnPlugin,
    input::InputPlugin,
    physics::PhysicsPlugin,
    ui::UiPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(CorePlugin)
        .add_plugins(SpawnPlugin)
        .add_plugins(InputPlugin)
        .add_plugins(PhysicsPlugin)
        .add_plugins(UiPlugin)
        .insert_resource(Score::new(0))
        .insert_resource(GameState::Menu)
        .add_observer(play_collition_sound)
        .run();
}

fn play_collition_sound(
    _collided: On<BallCollided>,
    mut commands: Commands,
    sound: Res<CollisionSound>
) {
    commands.spawn((AudioPlayer(sound.clone()),PlaybackSettings::DESPAWN));
}