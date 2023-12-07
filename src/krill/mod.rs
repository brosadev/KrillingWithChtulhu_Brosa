use bevy::prelude::*;

use crate::GameState;

use self::systems::*;

mod systems;

pub struct KrillPlugin;

impl Plugin for KrillPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<AlignCoe>()
            .register_type::<SeperationCoe>()
            .register_type::<CohesionCoe>()
            .register_type::<Acceleration>()
            .init_resource::<AlignCoe>()
            .init_resource::<SeperationCoe>()
            .init_resource::<CohesionCoe>()
            .add_state::<KrillState>()
            .add_systems(OnEnter(GameState::Active), spawn_krill)
            .add_systems(
                Update,
                (
                    debug_krill,
                    krill_idle_movement.run_if(in_state(KrillState::Idle)),
                    ((
                        ((boid_align, boid_seperation, boid_cohesion), boid_flock).chain(),
                        (krill_update_velocity, krill_rotate_to_face_vel_vec).chain(),
                    )
                        .chain())
                    .run_if(in_state(KrillState::Moving)),
                    // krill_movement.run_if(in_state(KrillState::Moving)),
                    krill_death.run_if(in_state(KrillState::Dead)),
                ),
            );
    }
}
