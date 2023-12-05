use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

const PLAYER_SPEED: f32 = 50.0;
const PLAYER_SCALE: f32 = 0.25;
const DAMPING: f32 = 3.0;

#[derive(Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Player;

#[derive(Bundle)]
pub struct PlayerBundle {
    player: Player,
    sprite: SpriteBundle,
}

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, player_movement);
    }
}

// This function is a very basic movement system that does not incorporate collisions and physics that can be found in Rapier

//This function is likely to evolve as physics become decided

pub fn player_movement(
    input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &Sprite), With<Player>>,
    time: Res<Time>,
) {
    for (mut transform, _) in &mut query {
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
pub fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Player,
        Collider::cuboid(2.0, 2.0),
        RigidBody::Dynamic,
        SpriteBundle {
            texture: asset_server
                .load("../assets/kenney_fish-pack/PNG/default_size/fishTile_103.png"),
            transform: Transform {
                scale: Vec3::new(PLAYER_SCALE, PLAYER_SCALE, 1.0),
                ..Default::default()
            },
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
