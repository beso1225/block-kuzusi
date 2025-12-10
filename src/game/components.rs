use bevy::prelude::*;

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

#[derive(Component)]
pub struct ScoreboardUi;