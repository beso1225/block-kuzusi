use bevy::{math::bounding::{Aabb2d, BoundingCircle, IntersectsVolume}, prelude::*};

mod game;
use game::prelude::*;
use game::plugins::core::CorePlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(CorePlugin)
        .insert_resource(Score::new(0))
        .insert_resource(GameState::Menu)
        .add_systems(Startup, setup_start_ui)
        .add_systems(
            FixedUpdate,
            (
                apply_velocity,
                move_paddle,
                check_for_collisions
            ).chain(),
        )
        .add_systems(Update, update_scoreboard)
        .add_systems(Update, input_start_game)
        .add_systems(Update, input_return_to_menu)
        .add_observer(play_collition_sound)
        .run();
}

enum WallLocation {
    Left,
    Right,
    Bottom,
    Top,
}

impl WallLocation {
    fn position(&self) -> Vec2 {
        match self {
            WallLocation::Left => Vec2::new(LEFT_WALL, 0.0),
            WallLocation::Right => Vec2::new(RIGHT_WALL, 0.0),
            WallLocation::Bottom => Vec2::new(0.0, BOTTOM_WALL),
            WallLocation::Top => Vec2::new(0.0, TOP_WALL),
        }
    }

    fn size(&self) -> Vec2 {
        let arena_height = TOP_WALL - BOTTOM_WALL;
        let arena_width = RIGHT_WALL - LEFT_WALL;
        assert!(arena_height > 0.0);
        assert!(arena_width > 0.0);

        match self {
            WallLocation::Left | WallLocation::Right => {
                Vec2::new(
                    WALL_THICKNESS,
                    arena_height + WALL_THICKNESS,
                )
            }
            WallLocation::Bottom | WallLocation::Top => {
                Vec2::new(
                    arena_width + WALL_THICKNESS,
                    WALL_THICKNESS,
                )
            }
        }
    }
}

impl Wall {
    fn new(location: WallLocation) -> (Wall, Sprite, Transform) {
        (
            Wall,
            Sprite::from_color(WALL_COLOR, Vec2::ONE),
            Transform {
                translation: location.position().extend(0.0),
                scale: location.size().extend(1.0),
                ..default()
            },
        )
    }
}

fn spawn_level(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // Note: Camera is spawned once in `setup_start_ui` to avoid multiple active cameras.
    // Do not spawn another Camera here.

    // Sound
    let ball_collision_sound = asset_server.load("sounds/breakout_collision.ogg");
    commands.insert_resource(CollisionSound::new(ball_collision_sound));

    // Paddle
    let paddle_y = BOTTOM_WALL + GAP_BETWEEN_PADDLE_AND_FLOOR;

    commands.spawn((
        Sprite::from_color(PADDLE_COLOR, Vec2::ONE),
        Transform {
            translation: Vec3::new(0.0, paddle_y, 0.0),
            scale: PADDLE_SIZE.extend(1.0),
            ..default()
        },
        Paddle,
        Collider,
    ));

    // Ball
    commands.spawn((
        Mesh2d(meshes.add(Circle::default())),
        MeshMaterial2d(materials.add(BALL_COLOR)),
        Transform::from_translation(BALL_STARTING_POSITION)
            .with_scale(Vec2::splat(BALL_DIAMETER).extend(1.0)),
        Ball,
        Velocity::new(INITIAL_BALL_DIRECTION.normalize() * BALL_SPEED),
    ));

    // Scoreboard
    commands.spawn((
        Text::new("Score: "),
        TextFont {
            font_size: SCOREBOARD_FONT_SIZE,
            ..default()
        },
        TextColor(TEXT_COLOR),
        ScoreboardUi,
        Node {
            position_type: PositionType::Absolute,
            top: SCOREBOARD_TEXT_PADDING,
            left: SCOREBOARD_TEXT_PADDING,
            ..default()
        },
        children![(
            TextSpan::default(),
            TextFont {
                font_size: SCOREBOARD_FONT_SIZE,
                ..default()
            },
            TextColor(SCORE_COLOR)
        )],
    ));

    // Walls
    commands.spawn(Wall::new(WallLocation::Left));
    commands.spawn(Wall::new(WallLocation::Right));
    // bottom wall: mark with BottomWall so we can detect game-over collisions
    let (w, s, t) = Wall::new(WallLocation::Bottom);
    commands.spawn((w, s, t, BottomWall));
    commands.spawn(Wall::new(WallLocation::Top));

    // Bricks
    let total_width_of_bricks = (RIGHT_WALL - LEFT_WALL) - 2.0 * GAP_BETWEEN_BRICKS_AND_SIDES;
    let bottom_edge_of_bricks = paddle_y + GAP_BETWEEN_PADDLE_AND_BRICKS;
    let total_height_of_bricks = TOP_WALL - bottom_edge_of_bricks - GAP_BETWEEN_BRICKS_AND_CEILING;

    assert!(total_width_of_bricks > 0.0);
    assert!(total_height_of_bricks > 0.0);

    let n_columns = (total_width_of_bricks / (BRICK_SIZE.x + GAP_BETWEEN_BRICKS)).floor() as usize;
    let n_rows = (total_height_of_bricks / (BRICK_SIZE.y + GAP_BETWEEN_BRICKS)).floor() as usize;
    let n_vertical_gaps = n_columns - 1;

    let center_of_bricks = (LEFT_WALL + RIGHT_WALL) / 2.0;
    let left_edge_of_bricks = center_of_bricks
        - (n_columns as f32 / 2.0 * BRICK_SIZE.x)
        - n_vertical_gaps as f32 / 2.0 * GAP_BETWEEN_BRICKS;

    let offset_x = left_edge_of_bricks + BRICK_SIZE.x / 2.0;
    let offset_y = bottom_edge_of_bricks + BRICK_SIZE.y / 2.0;

    for row in 0..n_rows {
        for column in 0..n_columns {
            let brick_position = Vec2::new(
                offset_x + column as f32 * (BRICK_SIZE.x + GAP_BETWEEN_BRICKS),
                offset_y + row as f32 * (BRICK_SIZE.y + GAP_BETWEEN_BRICKS),
            );

            // Bricks
            commands.spawn((
                Sprite {
                    color: BRICK_COLOR,
                    ..default()
                },
                Transform {
                    translation: brick_position.extend(0.0),
                    scale: Vec3::new(BRICK_SIZE.x, BRICK_SIZE.y, 1.0),
                    ..default()
                },
                Brick,
                Collider,
            ));
        }
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
        commands.spawn((
            Sprite::from_color(Color::srgba(0.0, 0.0, 0.0, 0.5), Vec2::new(RIGHT_WALL - LEFT_WALL + 200.0, TOP_WALL - BOTTOM_WALL + 200.0)),
            Transform::from_translation(Vec3::new(0.0, 0.0, 200.0)),
            StartUi,
        ));

    // Preload collision sound resource so it's available when level spawns
    let ball_collision_sound = asset_server.load("sounds/breakout_collision.ogg");
    commands.insert_resource(CollisionSound::new(ball_collision_sound));

    // Spawn the level in the background while showing the StartUi overlay.
    // Systems that update movement/collisions check `GameState` and won't run
    // until the player starts the game, so the scene will be static behind the overlay.
    spawn_level(commands, meshes, materials, asset_server);
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
        commands.spawn((
            Sprite::from_color(Color::srgba(0.0, 0.0, 0.0, 0.5), Vec2::new(RIGHT_WALL - LEFT_WALL + 200.0, TOP_WALL - BOTTOM_WALL + 200.0)),
            Transform::from_translation(Vec3::new(0.0, 0.0, 200.0)),
            StartUi,
        ));
        // respawn level in background (systems disabled because state=Menu)
        spawn_level(commands, meshes, materials, asset_server);
        info!("Returned to menu and reset game");
    }
}

fn apply_velocity(mut query: Query<(&mut Transform, &Velocity)>, time: Res<Time>, game_state: Res<GameState>) {
    if *game_state != GameState::Playing { return; }
    for (mut transform, velocity) in &mut query {
        transform.translation.x += velocity.x * time.delta_secs();
        transform.translation.y += velocity.y * time.delta_secs();
    }
}

fn move_paddle(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut paddle_transform: Single<&mut Transform, With<Paddle>>,
    time: Res<Time>,
    game_state: Res<GameState>,
) {
    let mut direction = 0.0;

    if *game_state != GameState::Playing { return; }

    if keyboard_input.pressed(KeyCode::ArrowLeft) {
        direction -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::ArrowRight) {
        direction += 1.0;
    }

    let new_paddle_position = paddle_transform.translation.x + direction * PADDLE_SPEED * time.delta_secs();

    let left_bound = LEFT_WALL + WALL_THICKNESS / 2.0 + PADDLE_SIZE.x / 2.0 + PADDLE_PADDING;
    let right_bound = RIGHT_WALL - WALL_THICKNESS / 2.0 - PADDLE_SIZE.x / 2.0 - PADDLE_PADDING;

    paddle_transform.translation.x = new_paddle_position.clamp(left_bound, right_bound)
}

fn check_for_collisions(
    mut commands: Commands,
    mut score: ResMut<Score>,
    ball_query: Single<(&mut Velocity, &Transform), With<Ball>>,
    collider_query: Query<(Entity, &Transform, Option<&Brick>), With<Collider>>,
    mut state: ResMut<GameState>,
    bottom_wall_query: Query<(), With<BottomWall>>,
    all_query: Query<Entity, Or<(With<Paddle>, With<Ball>, With<Brick>, With<Collider>, With<Wall>, With<ScoreboardUi>)>>,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let (mut ball_velocity, ball_transform) = ball_query.into_inner();
    if *state != GameState::Playing { return; }

    let mut game_over = false;

    for (collider_entity, collider_transform, maybe_brick) in &collider_query {
        let collision = ball_collision(
            BoundingCircle::new(ball_transform.translation.truncate(), BALL_DIAMETER / 2.0),
            Aabb2d::new(
                collider_transform.translation.truncate(),
                collider_transform.scale.truncate(),
            )
        );
        if let Some(collision) = collision {
            commands.trigger(BallCollided);

            if maybe_brick.is_some() {
                commands.entity(collider_entity).despawn();
                **score += 1;
            } else {
                // not a brick: possibly a wall. If it's the bottom wall, trigger game over
                if bottom_wall_query.get(collider_entity).is_ok() {
                    game_over = true;
                    break;
                }
            }

            let mut reflect_x = false;
            let mut reflect_y = false;

            match collision {
                Collision::Left => reflect_x = ball_velocity.x > 0.0,
                Collision::Right => reflect_x = ball_velocity.x < 0.0,
                Collision::Top => reflect_y = ball_velocity.y < 0.0,
                Collision::Bottom => reflect_y = ball_velocity.y > 0.0,
            }

            if reflect_x {
                ball_velocity.x = -ball_velocity.x;
            }

            if reflect_y {
                ball_velocity.y = -ball_velocity.y;
            }
        }
    }

    if game_over {
        // despawn game entities
        for e in &all_query {
            commands.entity(e).despawn();
        }
        // reset score and set menu state
        **score = 0;
        *state = GameState::Menu;
        // spawn Start UI overlay
        commands.spawn((
            Sprite::from_color(Color::srgba(0.0, 0.0, 0.0, 0.5), Vec2::new(RIGHT_WALL - LEFT_WALL + 200.0, TOP_WALL - BOTTOM_WALL + 200.0)),
            Transform::from_translation(Vec3::new(0.0, 0.0, 200.0)),
            StartUi,
        ));
        // respawn level in background for menu
        spawn_level(commands, meshes, materials, asset_server);
    }

}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Collision {
    Left,
    Right,
    Top,
    Bottom,
}

fn ball_collision(
    ball: BoundingCircle,
    bounding_box: Aabb2d,
) -> Option<Collision>{
    if !ball.intersects(&bounding_box) {
        return None;
    }

    let closest = bounding_box.closest_point(ball.center);
    let offset = ball.center - closest;
    let side = if offset.x.abs() > offset.y.abs() {
        if offset.x < 0.0 {
            Collision::Left
        } else {
            Collision::Right
        }
    } else if offset.y > 0.0 {
        Collision::Top
    } else {
        Collision::Bottom
    };

    Some(side)
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