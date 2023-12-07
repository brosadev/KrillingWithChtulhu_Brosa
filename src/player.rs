use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{assets::ImageAssets, GameState};
const PLAYER_SPEED: f32 = 50.0;
const PLAYER_SCALE: f32 = 0.50;
const DAMPING: f32 = 3.0;
const LASER_SPEED: f32 = 200.0;
const DESPAWN_DISTANCE: f32 = 1000.0;

#[derive(Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Player;

#[derive(Component)]
pub struct Laser;

#[derive(Component)]
struct Velocity {
    linvel: Vec3,
}

#[derive(Bundle)]
pub struct PlayerBundle {
    player: Player,
    sprite: SpriteBundle,
}

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Active), spawn_player)
            .add_systems(Update, (player_movement, spawn_laser, velocity, despawn));
    }
}

// This function is a very basic movement system that does not incorporate collisions and physics that can be found in Rapier

//This function is likely to evolve as physics become decided

pub fn player_movement(
    input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    for mut transform in &mut query {
        if input.pressed(KeyCode::W) || input.pressed(KeyCode::Up) {
            transform.translation.y += PLAYER_SPEED * time.delta_seconds();
        }
        if input.pressed(KeyCode::D) || input.pressed(KeyCode::Right) {
            transform.translation.x += PLAYER_SPEED * time.delta_seconds();
        }
        if input.pressed(KeyCode::A) || input.pressed(KeyCode::Left) {
            transform.translation.x -= PLAYER_SPEED * time.delta_seconds();
        }
        if input.pressed(KeyCode::S) || input.pressed(KeyCode::Down) {
            transform.translation.y -= PLAYER_SPEED * time.delta_seconds();
        }
    }
}

// This is a very basic player spawn implementation
pub fn spawn_player(mut commands: Commands, image_assets: Res<ImageAssets>) {
    commands.spawn((
        Player,
        Collider::cuboid(2.0, 2.0),
        RigidBody::Dynamic,
        SpriteSheetBundle {
            transform: Transform {
                scale: Vec3::new(PLAYER_SCALE, PLAYER_SCALE, 1.0),
                ..Default::default()
            },
            sprite: TextureAtlasSprite::new(0),
            texture_atlas: image_assets.whale.clone(),
            ..Default::default()
        },
        //This is the specific area, where you can adjust the bouncing off of walls
        // You can add and play with much more here in regards to physics
        Damping {
            linear_damping: DAMPING,
            angular_damping: DAMPING,
        },
    ));
}

pub fn spawn_laser(
    mut commands: Commands,
    input: Res<Input<KeyCode>>,
    query: Query<&Transform, With<Player>>,
) {
    if input.just_pressed(KeyCode::Space) {
        if let Ok(player_transform) = query.get_single() {
            commands
                .spawn(SpriteBundle {
                    sprite: Sprite {
                        color: Color::RED,
                        custom_size: Some(Vec2::new(5.0, 15.0)),
                        ..default()
                    },
                    transform: Transform {
                        translation: player_transform.translation + Vec3::Y * 20.0,
                        ..default()
                    },
                    ..default()
                })
                .insert(Laser)
                .insert(Velocity {
                    linvel: Vec3::new(0.0, LASER_SPEED, 0.0),
                });
        }
    }
}

fn despawn(mut commands: Commands, query: Query<(Entity, &GlobalTransform)>) {
    for (entity, transform) in query.iter() {
        let distance = transform.translation().distance(Vec3::ZERO);
        if distance > DESPAWN_DISTANCE {
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn velocity(time: Res<Time>, mut query: Query<(&Velocity, &mut Transform)>) {
    for (velocity, mut transform) in query.iter_mut() {
        transform.translation += velocity.linvel * time.delta_seconds();
    }
}
