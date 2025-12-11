use bevy::prelude::*;
use crate::game::{prelude::*, systems::ui::start_ui};

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, input_start_game)
            .add_systems(Update, input_return_to_menu);
    }
}

fn input_start_game(
    keys: Res<ButtonInput<KeyCode>>,
    mut state: ResMut<GameState>,
    mut commands: Commands,
    start_ui: Query<Entity, With<StartUi>>,
) {
    if *state == GameState::Menu && (keys.just_pressed(KeyCode::Space ) || keys.just_pressed(KeyCode::Enter)) {
        // remove any StartUi so it doesn't cover the scene
        for e in &start_ui {
            commands.entity(e).despawn();
        }
        // just transition to Playing; level already exists in background
        *state = GameState::Playing;
        info!("Game started");
    }
}

fn input_return_to_menu(
    keys: Res<ButtonInput<KeyCode>>,
    mut state: ResMut<GameState>,
    mut commands: Commands,
    mut score: ResMut<Score>,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
    query: Query<Entity, Or<(With<Paddle>, With<Ball>, With<Brick>, With<Collider>, With<Wall>, With<ScoreboardUi>)>>,
) {
    if *state == GameState::Playing && keys.just_pressed(KeyCode::Escape) {
        // despawn all game entities
        for e in &query {
            commands.entity(e).despawn();
        }
        // reset score
        score.set_zero();
        // insert menu state
        *state = GameState::Menu;
        // spawn Start UI overlay
        start_ui::spawn_start_ui(&mut commands);
        // respawn level in background (systems disabled because state=Menu)
        spawn_breakout_ui(commands, meshes, materials, asset_server);
        info!("Returned to menu and reset game");
    }
}