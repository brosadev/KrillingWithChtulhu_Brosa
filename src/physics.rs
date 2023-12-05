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
            .add_plugins(RapierDebugRenderPlugin::default());
    }
}
