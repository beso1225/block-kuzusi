use bevy::{prelude::*, math::bounding::{Aabb2d, BoundingCircle, IntersectsVolume}};

use crate::game::CORNER_THRESHOLD;

#[derive(Debug, PartialEq, Copy, Clone)]

pub enum Corner {
    UpperLeft,
    UpperRight,
    LowerLeft,
    LowerRight,
}
pub enum Collision {
    Left,
    Right,
    Top,
    Bottom,
    Corner(Vec2, Corner),
}

pub fn ball_collision(
    ball: BoundingCircle,
    bounding_box: Aabb2d,
    paddle_hit: bool
) -> Option<Collision>{
    if !ball.intersects(&bounding_box) {
        return None;
    }

    let closest = bounding_box.closest_point(ball.center);
    let offset = ball.center - closest;

    let abs_x = offset.x.abs();
    let abs_y = offset.y.abs();
    if (abs_x - abs_y).abs() <= CORNER_THRESHOLD {
        if offset.length_squared() <= 1e-6 {
            return Some(Collision::Top)
        } else {
            if paddle_hit {
                if offset.y <= 0.0 {
                    return Some(Collision::Bottom);
                }
            }
            // 4つの角のどれかを判定
            let corner = if offset.x >= 0.0 && offset.y >= 0.0 {
                Corner::UpperRight
            } else if offset.x < 0.0 && offset.y >= 0.0 {
                Corner::UpperLeft
            } else if offset.x >= 0.0 && offset.y < 0.0 {
                Corner::LowerRight
            } else {
                Corner::LowerLeft
            };
            return Some(Collision::Corner(offset.normalize(), corner));
        }
    }

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
    paddle_hit: bool,
) {
    if paddle_hit {
        // Optionally, add logic for modifying ball velocity based on where it hit the paddle
        // For example, you could change the angle of reflection based on the hit position
        match collision {
            Collision::Bottom => return,
            Collision::Corner(_normal, corner) => {
                match corner {
                    Corner::LowerRight | Corner::LowerLeft => return,
                    _ => {}
                }
            }
            _ => {}
        }
    }

    let mut reflect_x = false;
    let mut reflect_y = false;

    match collision {
        Collision::Left => reflect_x = ball_velocity.x > 0.0,
        Collision::Right => reflect_x = ball_velocity.x < 0.0,
        Collision::Top => reflect_y = ball_velocity.y < 0.0,
        Collision::Bottom => reflect_y = ball_velocity.y > 0.0,
        Collision::Corner(normal, _corner) => reflection_corner(ball_velocity, normal),
    }

    if reflect_x {
        ball_velocity.x = -ball_velocity.x;
    }
    if reflect_y {
        ball_velocity.y = -ball_velocity.y;
    }
}

fn reflection_corner(
    ball_velocity: &mut Vec2,
    normal: Vec2,
) {
    let normal = normal;
    let velocity_along_normal = ball_velocity.dot(normal);
    let reflection = *ball_velocity - 2.0 * velocity_along_normal * normal;
    *ball_velocity = reflection;
}

