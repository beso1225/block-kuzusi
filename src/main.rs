use bevy::prelude::*;

mod game;
use game::prelude::*;
use game::plugins::{
    core::CorePlugin,
    spawn::SpawnPlugin,
    input::InputPlugin,
    physics::PhysicsPlugin,
    ui::UiPlugin,
    audio::AudioPlugin,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(CorePlugin)
        .add_plugins(SpawnPlugin)
        .add_plugins(InputPlugin)
        .add_plugins(PhysicsPlugin)
        .add_plugins(UiPlugin)
        .add_plugins(AudioPlugin)
        .insert_resource(Score::new(0))
        .insert_resource(GameState::Menu)
        .run();
}