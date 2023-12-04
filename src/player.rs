use bevy::prelude::*;

#[derive(Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Player;

#[derive(Bundle)]
pub struct PlayerBundle {
    player: Player,
    sprite: SpriteBundle,
}

// This function is a very basic movement system that does not incorporate collisions and physics that can be found in Rapier

//This function is likely to evolve as physics become decided

pub fn player_movement(
    input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &Sprite), With<Player>>,
    time: Res<Time>,
) {
    for (mut transform, _) in &mut query {
        if input.pressed(KeyCode::W) {
            transform.translation.y += 400.0 * time.delta_seconds();
        }
        if input.pressed(KeyCode::D) {
            transform.translation.x += 400.0 * time.delta_seconds();
        }
        if input.pressed(KeyCode::A) {
            transform.translation.x -= 400.0 * time.delta_seconds();
        }
        if input.pressed(KeyCode::S) {
            transform.translation.y -= 400.0 * time.delta_seconds();
        }
    }
}

//This is a very basic player spawn implementation

pub fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    let scale = 0.25;

    commands
        .spawn(SpriteBundle {
            texture: asset_server
                .load("../assets/kenney_fish-pack/PNG/default_size/fishTile_103.png"),
            transform: Transform {
                scale: Vec3::new(scale, scale, 1.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Player);
}
