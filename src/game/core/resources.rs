pub use bevy::prelude::*;

#[derive(Resource, Clone, Copy, PartialEq, Eq, Debug)]
pub enum GameState {
    Menu,
    Playing,
}

#[derive(Resource, Deref)]
pub struct CollisionSound(Handle<AudioSource>);

impl CollisionSound {
    pub fn new(handle: Handle<AudioSource>) -> Self {
        Self(handle)
    }
}

#[derive(Resource, Deref, DerefMut)]
pub struct Score(usize);

#[allow(dead_code)]
impl Score {
    pub fn new(value: usize) -> Self {
        Self(value)
    }

    pub fn set(&mut self, value: usize) {
        self.0 = value;
    }

    pub fn set_zero(&mut self) {
        self.0 = 0;
    }

    pub fn get(&self) -> usize {
        self.0
    }
}