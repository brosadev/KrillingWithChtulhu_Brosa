use bev::prelude::*;

#[derive(Component)]
struct Krill;

pub fn spawn_krill(commands: &mut Commands, asset_server: Res<AssetServer>) {
    commands.spawn();
}
