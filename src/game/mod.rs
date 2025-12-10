pub mod prelude;

mod core;
mod plugins;

pub use prelude::*;
pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(plugins::core::CorePlugin)
            .add_plugins(plugins::spawn::SpawnPlugin)
            .add_plugins(plugins::input::InputPlugin)
            .add_plugins(plugins::physics::PhysicsPlugin)
            .add_plugins(plugins::ui::UiPlugin)
            .add_plugins(plugins::audio::AudioPlugin)
            .add_plugins(plugins::resource::ResourcePlugin);
    }
}