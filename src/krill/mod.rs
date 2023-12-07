use bevy::prelude::*;

use crate::GameState;

use self::systems::*;

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
                    (
                        boid_align,
                        boid_seperation,
                        boid_cohesion,
                        boid_flock,
                        krill_update_velocity,
                        krill_rotate_to_face_vel_vec,
                    )
                        .run_if(in_state(KrillState::Moving)),
                    // krill_movement.run_if(in_state(KrillState::Moving)),
                ),
            );
    }
}
