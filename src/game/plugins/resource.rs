use bevy::prelude::*;
use crate::game::prelude::*;

pub struct ResourcePlugin;

impl Plugin for ResourcePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Score::new(0))
            .insert_resource(GameState::Menu);
    }
}

