use bevy::prelude::*;
use crate::game::prelude::*;

pub fn game_over(
    mut commands: Commands,
    mut score: ResMut<Score>,
    mut state: ResMut<GameState>,
    all_query: Query<Entity, Or<(With<Paddle>, With<Ball>, With<Brick>, With<Collider>, With<Wall>, With<ScoreboardUi>)>>,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // despawn all game entities
    for e in &all_query {
        commands.entity(e).despawn();
    }
    // reset score
    let this_score = score.get();
    score.set_zero();
    // insert menu state
    *state = GameState::Menu;
    // spawn Game Over UI overlay
    spawn_gameover_ui(&mut commands, this_score);
    info!("Game Over! Returned to menu and reset game");
    // respawn level in background for menu
    spawn_breakout_ui(commands, meshes, materials, asset_server);
}

fn spawn_gameover_ui(
    commands: &mut Commands,
    this_score: usize,
) {
    commands.spawn((
            Sprite::from_color(Color::srgba(0.0, 0.0, 0.0, 0.5), Vec2::new(RIGHT_WALL - LEFT_WALL + 200.0, TOP_WALL - BOTTOM_WALL + 200.0)),
            Transform::from_translation(Vec3::new(0.0, 0.0, 200.0)),
            StartUi,
        ));
    // game over text
    commands.spawn((
        Text::new(format!("Game Over!\nYour Score: {}", this_score)),
        TextFont {
            font_size: START_UI_TITLE_FONT_SIZE,
            ..default()
        },
        TextColor(TITLE_TEXT_COLOR),
        StartUi,
        Node {
            position_type: PositionType::Absolute,
            top: Val::Percent(35.0),
            left: Val::Percent(20.0),
            ..default()
        },
    ));
}