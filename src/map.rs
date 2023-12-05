use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_rapier2d::prelude::*;

//this entire file, is pretty much all temporary until development on a professional tilemap system is configured

pub struct MapPlugin;
impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (floor, left_wall, right_wall, ceiling));
    }
}

pub fn floor(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        RigidBody::Fixed,
        Collider::cuboid(280.0, 10.0),
        MaterialMesh2dBundle {
            mesh: meshes
                .add(Mesh::from(shape::Quad::new(Vec2 { x: 280.0, y: 10.0 })))
                .into(),
            material: materials.add(ColorMaterial::from(Color::MIDNIGHT_BLUE)),
            transform: Transform::from_translation(Vec3::new(0.0, -70.0, -1.0)),
            ..default()
        },
    ));
}

pub fn ceiling(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        RigidBody::Fixed,
        Collider::cuboid(280.0, 10.0),
        MaterialMesh2dBundle {
            mesh: meshes
                .add(Mesh::from(shape::Quad::new(Vec2 { x: 280.0, y: 10.0 })))
                .into(),
            material: materials.add(ColorMaterial::from(Color::MIDNIGHT_BLUE)),
            transform: Transform::from_translation(Vec3::new(0.0, 70.0, -1.0)),
            ..default()
        },
    ));
}

pub fn left_wall(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        RigidBody::Fixed,
        Collider::cuboid(10.0, 130.0),
        MaterialMesh2dBundle {
            mesh: meshes
                .add(Mesh::from(shape::Quad::new(Vec2 { x: 10.0, y: 130.0 })))
                .into(),
            material: materials.add(ColorMaterial::from(Color::MIDNIGHT_BLUE)),
            transform: Transform::from_translation(Vec3::new(-130.0, 0.0, -1.0)),
            ..default()
        },
    ));
}

pub fn right_wall(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        RigidBody::Fixed,
        Collider::cuboid(10.0, 130.0),
        MaterialMesh2dBundle {
            mesh: meshes
                .add(Mesh::from(shape::Quad::new(Vec2 { x: 10.0, y: 130.0 })))
                .into(),
            material: materials.add(ColorMaterial::from(Color::MIDNIGHT_BLUE)),
            transform: Transform::from_translation(Vec3::new(130.0, 0.0, -1.0)),
            ..default()
        },
    ));
}
