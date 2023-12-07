use bevy::prelude::*;
use bevy_rapier2d::prelude::{
    NoUserData, RapierConfiguration, RapierDebugRenderPlugin, RapierPhysicsPlugin, Vect,
};

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
            .insert_resource(RapierConfiguration {
                gravity: Vect::ZERO,
                ..Default::default()
            })
            .add_plugins(RapierDebugRenderPlugin::default())
            .add_systems(Update, velocity);
    }
}

#[derive(Component)]
pub struct Velocity {
    linvel: Vec3,
}

pub fn velocity(time: Res<Time>, mut query: Query<(&Velocity, &mut Transform)>) {
    for (velocity, mut transform) in query.iter_mut() {
        transform.translation += velocity.linvel * time.delta_seconds();
    }
}
