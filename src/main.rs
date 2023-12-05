// Bevy code commonly triggers these lints and they may be important signals
// about code quality. They are sometimes hard to avoid though, and the CI
// workflow treats them as errors, so this allows them throughout the project.
// Feel free to delete this line.
#![allow(clippy::too_many_arguments, clippy::type_complexity)]
// just for wasm release build
#![allow(non_snake_case)]

mod assets;
mod krill;
mod map;
mod physics;
mod player;
mod display;

use assets::AssetsPlugin;
use bevy::{prelude::*, render::camera::ScalingMode};
use bevy_asset_loader::loading_state::{LoadingState, LoadingStateAppExt};
use display::DisplayPlugin;
use krill::KrillPlugin;
use map::MapPlugin;
use physics::PhysicsPlugin;
use player::PlayerPlugin;

#[cfg(feature = "debug")]
use bevy_inspector_egui::quick::WorldInspectorPlugin;

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
    let mut app = App::new();
    app.add_state::<GameState>()
        .add_loading_state(
            LoadingState::new(GameState::Loading).continue_to_state(GameState::Active),
        )
        .add_plugins(DefaultPlugins)
        // Main Plugins
				.add_plugins(DisplayPlugin)
        .add_plugins(PhysicsPlugin)
        .add_plugins(AssetsPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(KrillPlugin)
        .add_plugins(MapPlugin)
        .add_event::<DebugEvent>()
        .add_systems(Startup, setup)
        .add_systems(Update, debug);

    // Development Plugins
    #[cfg(feature = "debug")]
    app.add_plugins(WorldInspectorPlugin::new());

    app.run();
}

fn setup(mut commands: Commands) {
    let mut camera = Camera2dBundle::default();
    camera.projection.scaling_mode = ScalingMode::AutoMin {
        min_width: 256.,
        min_height: 144.,
    };
    commands.spawn(camera);
}

pub fn debug(keyboard_input: Res<Input<KeyCode>>, mut debug_event_writer: EventWriter<DebugEvent>) {
    if keyboard_input.just_pressed(KeyCode::Tab) {
        debug_event_writer.send(DebugEvent);
    }
}
