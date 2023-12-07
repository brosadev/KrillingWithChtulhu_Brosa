// Bevy code commonly triggers these lints and they may be important signals
// about code quality. They are sometimes hard to avoid though, and the CI
// workflow treats them as errors, so this allows them throughout the project.
// Feel free to delete this line.
#![allow(clippy::too_many_arguments, clippy::type_complexity)]
#![allow(unused)]

// use assets::AssetsPlugin;
use bevy::{math::Vec2, prelude::*};
use bevy_asset_loader::prelude::*;

use crate::GameState;

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

#[derive(Component)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
}

/*
#[derive(AssetCollection, Resource)]
struct AudioAssets {
    #[asset(path = "audio/background.ogg")]
    background: Handle<AudioSource>,
    #[asset(path = "audio/plop.ogg")]
    plop: Handle<AudioSource>
}
*/

#[derive(AssetCollection, Resource)]
pub struct ImageAssets {
    // if the sheet would have padding, we could set that with `padding_x` and `padding_y`.
    // if there's space between the top left corner of the sheet and the first sprite, we could configure that with `offset_x` and `offset_y`
    // whale uses a texture atlas as it will be moving while swimming
    #[asset(texture_atlas(tile_size_x = 32., tile_size_y = 32., columns = 5, rows = 3))]
    #[asset(path = "whale.png")]
    pub whale: Handle<TextureAtlas>,
    #[asset(path = "kenney_fish-pack/PNG/default_size/fishTile_093.png")]
    pub puffer_fish: Handle<Image>,
    #[asset(path = "kenney_fish-pack/PNG/default_size/fishTile_079.png")]
    pub red_fish: Handle<Image>,
    #[asset(path = "kenney_fish-pack/PNG/default_size/fishTile_077.png")]
    pub blue_fish: Handle<Image>,
    #[asset(path = "kenney_fish-pack/PNG/default_size/fishTile_073.png")]
    pub green_fish: Handle<Image>,
    #[asset(texture_atlas(tile_size_x = 32., tile_size_y = 32., columns = 2, rows = 1))]
    #[asset(path = "krill.png")]
    pub krill: Handle<TextureAtlas>,
}

pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_collection_to_loading_state::<_, ImageAssets>(GameState::Loading)
            //.add_systems(OnEnter(GameState::Active), draw_fish)
            .add_systems(
                Update,
                animate_sprite_system.run_if(in_state(GameState::Active)),
            );
    }
}

fn animate_sprite_system(
    time: Res<Time>,
    mut query: Query<(
        &AnimationIndices,
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
    )>,
) {
    for (indices, mut timer, mut sprite) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            sprite.index = if sprite.index == indices.last {
                indices.first
            } else {
                sprite.index + 1
            };
        }
    }
}

fn draw_fish(mut commands: Commands, image_assets: Res<ImageAssets>) {
    // draw single texture from sprite sheet starting at index 0
    let sprite_size = Vec2::new(10., 10.);
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(sprite_size),
                ..Default::default()
            },
            transform: Transform {
                translation: Vec3::new(-50., 0., 1.),
                ..Default::default()
            },
            //sprite: TextureAtlasSprite::new(0),
            texture: image_assets.puffer_fish.clone(),
            ..Default::default()
        },
        Name::new("puffer_fish"),
    ));
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(sprite_size), ..Default::default()
            },
            transform: Transform {
                translation: Vec3::new(50., 0., 1.),
                ..Default::default()
            },
            //sprite: TextureAtlasSprite::new(0),
            texture: image_assets.red_fish.clone(),
            ..Default::default()
        }, Name::new("red_fish")))
        // .insert(AnimationTimer(Timer::from_seconds(
        //     0.1,
        //     TimerMode::Repeating,
        // )))
    ;
}
