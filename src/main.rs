use bevy::{math::bounding::{Aabb2d, BoundingCircle, IntersectsVolume}, prelude::*};

mod game;
use game::prelude::*;
use game::plugins::{core::CorePlugin, spawn::{self, SpawnPlugin}, input::InputPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(CorePlugin)
        .add_plugins(SpawnPlugin)
        .add_plugins(InputPlugin)
        .insert_resource(Score::new(0))
        .insert_resource(GameState::Menu)
        .add_systems(
            FixedUpdate,
            (
                apply_velocity,
                move_paddle,
                check_for_collisions
            ).chain(),
        )
        .add_systems(Update, update_scoreboard)
        .add_observer(play_collition_sound)
        .run();
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
        spawn::spawn_level(commands, meshes, materials, asset_server);
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