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
use bevy_inspector_egui::{egui::Key, quick::WorldInspectorPlugin};
use krill::KrillPlugin;
use player::{player_movement, spawn_player};

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
pub enum GameState {
    #[default]
    Loading,
    Active,
}

#[derive(Event)]
pub struct DebugEvent;

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
        // Actor plugins??
        .add_plugins(KrillPlugin)
        .add_event::<DebugEvent>()
        .add_systems(Startup, setup)
        .add_systems(Update, (player_movement, debug))
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

pub fn debug(keyboard_input: Res<Input<KeyCode>>, mut debug_event_writer: EventWriter<DebugEvent>) {
    if keyboard_input.just_pressed(KeyCode::Tab) {
        debug_event_writer.send(DebugEvent);
    }
}
