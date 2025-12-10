use bevy::prelude::*;

mod game;
use game::prelude::*;
use game::plugins::{core::CorePlugin, spawn::SpawnPlugin, input::InputPlugin, physics::PhysicsPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(CorePlugin)
        .add_plugins(SpawnPlugin)
        .add_plugins(InputPlugin)
        .add_plugins(PhysicsPlugin)
        .insert_resource(Score::new(0))
        .insert_resource(GameState::Menu)
        .add_systems(Update, update_scoreboard)
        .add_observer(play_collition_sound)
        .run();
}

fn update_scoreboard(
    score: Res<Score>,
    score_root: Single<Entity, (With<ScoreboardUi>, With<Text>)>,
    mut writer: TextUiWriter,
) {
    *writer.text(*score_root, 1) = score.to_string();
}

fn play_collition_sound(
    _collided: On<BallCollided>,
    mut commands: Commands,
    sound: Res<CollisionSound>
) {
    commands.spawn((AudioPlayer(sound.clone()),PlaybackSettings::DESPAWN));
}