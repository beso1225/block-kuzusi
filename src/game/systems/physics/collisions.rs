use bevy::{prelude::*, math::bounding::{Aabb2d, BoundingCircle, IntersectsVolume}};

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Collision {
    Left,
    Right,
    Top,
    Bottom,
}

pub fn ball_collision(
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

pub fn ball_reflection(
    ball_velocity: &mut Vec2,
    collision: Collision,
) {
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