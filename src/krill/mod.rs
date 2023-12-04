use bevy::prelude::*;

use crate::GameState;

use self::systems::{debug_krill, spawn_krill};

mod systems;

pub struct KrillPlugin;

impl Plugin for KrillPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Active), spawn_krill)
            .add_systems(Update, debug_krill);
    }
}
