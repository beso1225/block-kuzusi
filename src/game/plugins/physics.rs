use bevy::{prelude::*, math::bounding::{Aabb2d, BoundingCircle}};

use crate::game::{prelude::*, systems::ui::gameover_ui};
use crate::game::systems::physics::{collisions, paddle, movement};


pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            movement::apply_velocity,
            paddle::move_paddle,
            check_for_collisions
        ).chain());
    }
}

fn check_for_collisions(
    mut commands: Commands,
    mut score: ResMut<Score>,
    ball_query: Single<(&mut Velocity, &Transform), With<Ball>>,
    collider_query: Query<(Entity, &Transform, Option<&Brick>), With<Collider>>,
    state: ResMut<GameState>,
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
        let collision = collisions::ball_collision(
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

            collisions::ball_reflection(&mut ball_velocity, collision);
        }
    }

    if game_over {
        gameover_ui::game_over(
            commands,
            score,
            state,
            all_query,
            meshes,
            materials,
            asset_server,
        );
    }

}