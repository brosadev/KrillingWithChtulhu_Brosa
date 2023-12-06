use bevy::prelude::*;

use self::systems::{setup, update_score, DisplayData, DisplayEvent};
use crate::GameState;

mod systems;

pub struct DisplayPlugin;

impl Plugin for DisplayPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(DisplayData { total_score: 0 })
            .add_systems(OnEnter(GameState::Active), setup)
            .add_event::<DisplayEvent>()
            .add_systems(Update, update_score);
    }
}
