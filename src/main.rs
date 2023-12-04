// Bevy code commonly triggers these lints and they may be important signals
// about code quality. They are sometimes hard to avoid though, and the CI
// workflow treats them as errors, so this allows them throughout the project.
// Feel free to delete this line.
#![allow(clippy::too_many_arguments, clippy::type_complexity)]

mod assets;
mod krill;
mod player;

// use assets::AssetsPlugin;
use bevy::{prelude::*, render::camera::ScalingMode};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use player::{player_movement, spawn_player};

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
pub enum GameState {
    #[default]
    Loading,
    Active,
}

/// This example demonstrates how to load a texture atlas from a sprite sheet
///
/// Requires the feature '2d'
fn main() {
    App::new()
        .add_state::<GameState>()
        .add_plugins(DefaultPlugins)
        // Development Plugins
        .add_plugins(WorldInspectorPlugin::new())
        // Main Plugins
        .add_plugins(assets::AssetsPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, player_movement)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut camera = Camera2dBundle::default();
    camera.projection.scaling_mode = ScalingMode::AutoMin {
        min_width: 256.,
        min_height: 144.,
    };
    commands.spawn(camera);
    spawn_player(commands, asset_server);
}
