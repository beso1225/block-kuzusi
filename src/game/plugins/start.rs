use bevy::prelude::*;
use crate::game::{prelude::*, systems::ui::start_ui};

pub struct StartPlugin;

impl Plugin for StartPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_start_ui);
    }
}

fn setup_start_ui(
    mut commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // Camera
    // commands.spawn(Camera2d);

    // Simple start screen as a large sprite; text rendering setup may vary across projects,
    // so we keep the start screen visual minimal here.
    // semi-transparent full-screen overlay for start menu
    start_ui::spawn_start_ui(&mut commands);

    // Preload collision sound resource so it's available when level spawns
    let ball_collision_sound = asset_server.load("sounds/breakout_collision.ogg");
    commands.insert_resource(CollisionSound::new(ball_collision_sound));

    // Spawn the level in the background while showing the StartUi overlay.
    // Systems that update movement/collisions check `GameState` and won't run
    // until the player starts the game, so the scene will be static behind the overlay.
    spawn_breakout_ui(commands, meshes, materials, asset_server);
}