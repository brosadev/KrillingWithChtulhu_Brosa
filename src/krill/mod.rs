use bevy::prelude::*;

use crate::GameState;

use self::systems::{debug_krill, krill_idle_movement, spawn_krill, KrillState};

mod systems;

pub struct KrillPlugin;

impl Plugin for KrillPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<KrillState>()
            .add_systems(OnEnter(GameState::Active), spawn_krill)
            .add_systems(
                Update,
                (
                    debug_krill,
                    krill_idle_movement.run_if(in_state(KrillState::Idle)),
                    // krill_movement.run_if(in_state(KrillState::Moving)),
                ),
            );
    }
}
