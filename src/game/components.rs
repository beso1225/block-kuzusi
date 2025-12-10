use bevy::prelude::*;
use crate::game::prelude::*;

#[derive(Component)]
pub struct StartUi;

#[derive(Component)]
pub struct Paddle;

#[derive(Component)]
pub struct Ball;

#[derive(Component, Deref, DerefMut)]
pub struct Velocity(Vec2);

impl Velocity {
    pub fn new(v: Vec2) -> Self {
        Self(v)
    }
}

#[derive(Component)]
pub struct Brick;

#[derive(Component)]
pub struct BottomWall;

#[derive(Component, Default)]
pub struct Collider;

#[derive(Component)]
#[require(Sprite, Transform, Collider)]
pub struct Wall;

pub enum WallLocation {
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
    pub fn new(location: WallLocation) -> (Wall, Sprite, Transform) {
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

#[derive(Component)]
pub struct ScoreboardUi;