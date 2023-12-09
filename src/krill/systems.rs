use std::ops::Range;

use bevy::prelude::*;
use bevy_rapier2d::{
    dynamics::{CoefficientCombineRule, RigidBody, Velocity},
    geometry::{Collider, CollisionGroups, Friction, Group, Restitution},
};

use rand::prelude::*;

use crate::{
    assets::{AnimationIndices, AnimationTimer, ImageAssets},
    map::{Obstacal, BOTTOM_BORDER, LEFT_BORDER, RIGHT_BORDER, TOP_BORDER},
    player::Player,
    DebugEvent,
};

const ERROR_FROM_ZERO: f32 = 0.05;

const SPAWN_X_RANGE: Range<f32> = LEFT_BORDER..RIGHT_BORDER;
const SPAWN_Y_RANGE: Range<f32> = BOTTOM_BORDER..TOP_BORDER;

pub const BOID_MAX_FORCE: f32 = 0.3;
pub const BOID_ALIGN_MAG: f32 = 7.6;
pub const BOID_SEPERATION_MAG: f32 = 1.5;
pub const BOID_CHOESION_MAG: f32 = 1.2;
pub const BOID_PERCEPTION_RADIUS: f32 = 7.;

const KRILL: &str = "Krill";
const KRILL_ENTITYS_STARTING_AMT: u16 = 600;
pub const KRILL_RADIUS: f32 = 2.5;
pub const KRILL_MAX_SPEED: f32 = 50.;
pub const KRILL_COLLISION_GROUP: Group = Group::GROUP_1;
const KRILL_AVOIDANCE_MAG: f32 = 50.;
const KRILL_RIGID_BODY: RigidBody = RigidBody::Dynamic;
const KRILL_RESTITUTION_COE: f32 = 1.;
const KRILL_FRICTION_COE: f32 = 0.;

#[derive(Bundle)]
pub struct BoidBundle {
    boid: Boid,
    acceleration: Acceleration,
    velocity: Velocity,
    align: Align,
    seperation: Seperation,
    cohesion: Cohesion,
}

#[derive(Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Boid;

#[derive(Clone, PartialEq, Debug, Default, Component, Reflect)]
#[reflect(Component)]
pub struct Acceleration {
    vec: Vec2,
}

// #[derive(Clone, PartialEq, Debug, Default, Component)]
// pub struct Nieghbors {
//     vec: Vec<,
// }

#[derive(Clone, PartialEq, Debug, Default, Component)]
pub struct Align {
    vec: Vec2,
}

#[derive(Clone, PartialEq, Debug, Default, Component)]
pub struct Seperation {
    vec: Vec2,
}

#[derive(Clone, PartialEq, Debug, Default, Component)]
pub struct Cohesion {
    vec: Vec2,
}

#[derive(Resource, Reflect)]
#[reflect(Resource)]
pub struct AlignCoe {
    pub mag: f32,
}

impl Default for AlignCoe {
    fn default() -> Self {
        Self {
            mag: BOID_ALIGN_MAG,
        }
    }
}

#[derive(Resource, Reflect)]
#[reflect(Resource)]
pub struct SeperationCoe {
    pub mag: f32,
}

impl Default for SeperationCoe {
    fn default() -> Self {
        Self {
            mag: BOID_SEPERATION_MAG,
        }
    }
}

#[derive(Resource, Reflect)]
#[reflect(Resource)]
pub struct CohesionCoe {
    pub mag: f32,
}

impl Default for CohesionCoe {
    fn default() -> Self {
        Self {
            mag: BOID_CHOESION_MAG,
        }
    }
}

#[derive(Bundle)]
pub struct KrillBundle {
    krill: Krill,
    name: Name,
    sprite: SpriteSheetBundle,
    animation_indices: AnimationIndices,
    animation_timer: AnimationTimer,
    collider: Collider,
    ridgid_body: RigidBody,
    restitution: Restitution,
    friction: Friction,
    collision_group: CollisionGroups,
    boid: BoidBundle,
}
#[derive(Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Krill;

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
pub enum KrillState {
    #[default]
    Moving,
    Idle,
    // Dead,
}

pub fn spawn_krill(mut commands: Commands, image_assets: Res<ImageAssets>) {
    let mut rand_gen = thread_rng();

    for _ in 0..KRILL_ENTITYS_STARTING_AMT {
        let random_x = rand_gen.gen_range(SPAWN_X_RANGE);
        let random_y = rand_gen.gen_range(SPAWN_Y_RANGE);
        let random_starting_vel =
            Vec2::new(rand_gen.gen_range(-1.0..1.0), rand_gen.gen_range(-1.0..1.0)).normalize();
        let random_starting_speed = rand_gen.gen_range(1.0..KRILL_MAX_SPEED);

        commands.spawn(KrillBundle {
            krill: Krill,
            name: Name::new(KRILL),
            sprite: SpriteSheetBundle {
                sprite: TextureAtlasSprite {
                    index: 0,
                    custom_size: Some(Vec2::new(KRILL_RADIUS * 2., KRILL_RADIUS * 2.)),
                    ..Default::default()
                },
                texture_atlas: image_assets.krill.clone(),
                transform: Transform::from_translation(Vec3::new(random_x, random_y, 1.)),
                ..Default::default()
            },
            animation_indices: AnimationIndices { first: 0, last: 1 },
            animation_timer: AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
            collider: Collider::ball(KRILL_RADIUS),
            collision_group: CollisionGroups {
                memberships: KRILL_COLLISION_GROUP,
                filters: Group::complement(KRILL_COLLISION_GROUP),
            },
            ridgid_body: KRILL_RIGID_BODY,
            restitution: Restitution {
                coefficient: KRILL_RESTITUTION_COE,
                combine_rule: CoefficientCombineRule::Max,
            },
            friction: Friction {
                coefficient: KRILL_FRICTION_COE,
                combine_rule: CoefficientCombineRule::Min,
            },
            boid: BoidBundle {
                boid: Boid,
                acceleration: Acceleration { vec: Vec2::ZERO },
                velocity: Velocity::linear(random_starting_vel * random_starting_speed),
                align: Align { vec: Vec2::ZERO },
                seperation: Seperation { vec: Vec2::ZERO },
                cohesion: Cohesion { vec: Vec2::ZERO },
            },
        });
    }
}

pub fn debug_krill(
    mut debug_event_reader: EventReader<DebugEvent>,
    krill_query: Query<(&Transform, &Velocity, &Acceleration), With<Krill>>,
) {
    for _event in debug_event_reader.read() {
        for (krill_transform, krill_vel, krill_acc) in krill_query.iter() {
            info!("{:?}", krill_transform);
            info!("{:?}", krill_vel);
            info!("{:?}", krill_acc);
        }
    }
}

pub fn krill_idle_movement(mut krill_query: Query<&mut Transform, With<Krill>>, time: Res<Time>) {
    for mut krill_transform in krill_query.iter_mut() {
        const IDLE_HIEGHT_SCALAR: f32 = 0.005;
        const IDLE_FREQ_SCALAR: f32 = 0.1;
        const IDLE_PERIOD_SCALAR: f32 = 20.;

        // info!("{:?}", (time.elapsed_seconds() / IDLE_FREQ_SCALAR).sin() * IDLE_HIEGHT_SCALAR;);

        krill_transform.translation.y += (time.elapsed_seconds()
            - (krill_transform.translation.x / IDLE_PERIOD_SCALAR) / IDLE_FREQ_SCALAR)
            .sin()
            * IDLE_HIEGHT_SCALAR;
    }
}

pub fn krill_rotate_to_face_vel_vec(
    mut krill_query: Query<(&mut Transform, &Velocity), (With<Krill>, Changed<Velocity>)>,
) {
    for (mut krill_transform, krill_velocity) in krill_query.iter_mut() {
        let angle = krill_velocity.linvel.y.atan2(krill_velocity.linvel.x);
        krill_transform.rotation = Quat::from_rotation_z(angle);
    }
}

pub fn krill_update_velocity(
    mut krill_query: Query<(&mut Velocity, &mut Acceleration), With<Krill>>,
    time: Res<Time>,
) {
    for (mut krill_velocity, mut krill_acceleration) in krill_query.iter_mut() {
        krill_velocity.linvel += krill_acceleration.vec * time.delta_seconds();
        krill_velocity.linvel.clamp_length_max(KRILL_MAX_SPEED);
        krill_acceleration.vec = Vec2::ZERO;
    }
}

pub fn boid_align(
    mut boid_a_query: Query<(Entity, &Transform, &Velocity, &mut Align), With<Boid>>,
    boid_b_query: Query<(Entity, &Transform, &Velocity), With<Boid>>,
) {
    for (boid_entity_a, boid_transform_a, boid_velocity_a, mut boid_align_a) in
        boid_a_query.iter_mut()
    {
        boid_align_a.vec = Vec2::ZERO;
        let mut num_near_boids: u32 = 0;

        for (boid_entity_b, boid_transform_b, boid_velocity_b) in boid_b_query.iter() {
            if boid_entity_a == boid_entity_b {
                continue;
            }

            if boid_transform_a
                .translation
                .distance(boid_transform_b.translation)
                < BOID_PERCEPTION_RADIUS
            {
                boid_align_a.vec += boid_velocity_b.linvel;
                num_near_boids += 1;
            }
        }

        if num_near_boids > 0 {
            boid_align_a.vec /= num_near_boids as f32;
            boid_align_a.vec =
                boid_align_a.vec.normalize() * KRILL_MAX_SPEED - boid_velocity_a.linvel;
            boid_align_a.vec.clamp_length_max(BOID_MAX_FORCE);
        }
    }
}

pub fn boid_seperation(
    mut boid_a_query: Query<(Entity, &Transform, &Velocity, &mut Seperation), With<Boid>>,
    boid_b_query: Query<(Entity, &Transform), With<Boid>>,
) {
    for (boid_entity_a, boid_transform_a, boid_velocity_a, mut boid_sepreation_a) in
        boid_a_query.iter_mut()
    {
        boid_sepreation_a.vec = Vec2::ZERO;
        let mut num_near_boids: u32 = 0;

        for (boid_entity_b, boid_transform_b) in boid_b_query.iter() {
            if boid_entity_a == boid_entity_b {
                continue;
            }

            let distance_between_boids = boid_transform_a
                .translation
                .distance(boid_transform_b.translation);

            if distance_between_boids < BOID_PERCEPTION_RADIUS / 1.5
                && !(-ERROR_FROM_ZERO..=ERROR_FROM_ZERO).contains(&distance_between_boids)
            {
                let mut distance_between_boids_as_vec =
                    boid_transform_a.translation.xy() - boid_transform_b.translation.xy();
                distance_between_boids_as_vec /= (distance_between_boids / 2.).powf(2.0);
                boid_sepreation_a.vec += distance_between_boids_as_vec;
                num_near_boids += 1;
            }
        }

        if num_near_boids > 0 {
            boid_sepreation_a.vec /= num_near_boids as f32;
            boid_sepreation_a.vec =
                boid_sepreation_a.vec.normalize() * KRILL_MAX_SPEED * 1.5 - boid_velocity_a.linvel;
            boid_sepreation_a.vec.clamp_length_max(BOID_MAX_FORCE);
        }
    }
}

pub fn boid_cohesion(
    mut boid_a_query: Query<(Entity, &Transform, &Velocity, &mut Cohesion), With<Boid>>,
    boid_b_query: Query<(Entity, &Transform), With<Boid>>,
) {
    for (boid_entity_a, boid_transform_a, boid_velocity_a, mut boid_coehesion_a) in
        boid_a_query.iter_mut()
    {
        boid_coehesion_a.vec = Vec2::ZERO;
        let mut num_near_boids: u32 = 0;

        for (boid_entity_b, boid_transform_b) in boid_b_query.iter() {
            if boid_entity_a == boid_entity_b {
                continue;
            }

            if boid_transform_a
                .translation
                .distance(boid_transform_b.translation)
                < BOID_PERCEPTION_RADIUS * 1.1
            {
                boid_coehesion_a.vec += boid_transform_b.translation.xy();
                num_near_boids += 1;
            }
        }

        if num_near_boids > 0 {
            boid_coehesion_a.vec /= num_near_boids as f32;
            boid_coehesion_a.vec = (boid_coehesion_a.vec - boid_transform_a.translation.xy())
                .normalize()
                * KRILL_MAX_SPEED
                - boid_velocity_a.linvel;
            boid_coehesion_a.vec.clamp_length_max(BOID_MAX_FORCE);
        }
    }
}

pub fn boid_flock(
    mut boid_query: Query<(&mut Acceleration, &Align, &Seperation, &Cohesion), With<Boid>>,
    align_coe: Res<AlignCoe>,
    sepration_coe: Res<SeperationCoe>,
    cohesion_coe: Res<CohesionCoe>,
) {
    for (mut boid_acceleration, boid_align, boid_seperation, boid_cohesion) in boid_query.iter_mut()
    {
        boid_acceleration.vec += (boid_align.vec * align_coe.mag)
            + (boid_seperation.vec * sepration_coe.mag)
            + (boid_cohesion.vec * cohesion_coe.mag);
    }
}

// const ROTATION_SPEED: f32 = 0.5;

// pub fn krill_death(
//     mut krill_query: Query<(&mut Transform, &mut Sprite), With<Krill>>,
//     time: Res<Time>,
//     mut commands: Commands,
// ) {
//     let elapsed_seconds = time.elapsed_seconds();
//     commands.spawn((
//         for (mut krill_transform, mut krill_change) in krill_query.iter_mut() {
//             if elapsed_seconds <= 1.0 {
//                 // Rotate smoothly until upside down

//                 let target_rotation = Quat::from_rotation_x(-std::f32::consts::PI); // Upside down
//                 let rotation = Quat::from_rotation_x(elapsed_seconds * ROTATION_SPEED.to_radians());
//                 let interpolated_rotation = Quat::slerp(rotation, target_rotation, elapsed_seconds);
//                 krill_transform.rotation = interpolated_rotation;

//                 let light_blue = Color::rgb(0.2, 0.2, 1.0); // Adjust as needed
//                                                             // Store the original color
//                 let original_color = krill_change.color;
//                 // Adjust the speed of color change by introducing a color change speed factor
//                 let color_change_speed_factor = 0.4; // Adjust as needed
//                                                      // Manually interpolate color components
//                 let factor = elapsed_seconds / (1.0 * color_change_speed_factor); // Assuming 1.0 seconds for the color transition
//                 krill_change
//                     .color
//                     .set_r((1.0 - factor) * original_color.r() + factor * light_blue.r());
//                 krill_change
//                     .color
//                     .set_g((1.0 - factor) * original_color.g() + factor * light_blue.g());
//                 krill_change
//                     .color
//                     .set_b((1.0 - factor) * original_color.b() + factor * light_blue.b());
//             }
//         },
//     ));
// }

pub fn krill_avoid_player(
    mut krill_query: Query<(&mut Acceleration, &Transform), With<Krill>>,
    player_query: Query<&Transform, With<Player>>,
) {
    let Ok(player_transform) = player_query.get_single() else {
        info!("error");
        return;
    };
    for (mut krill_acceleration, krill_transform) in krill_query.iter_mut() {
        let dist = krill_transform
            .translation
            .distance(player_transform.translation);
        if dist < BOID_PERCEPTION_RADIUS * 5. && dist > ERROR_FROM_ZERO {
            krill_acceleration.vec += ((krill_transform.translation.xy()
                - player_transform.translation.xy())
            .normalize()
                * KRILL_AVOIDANCE_MAG)
                / ((dist / 30.).powf(1.3));
        }
    }
}

pub fn krill_avoid_obstical(
    mut krill_query: Query<(&mut Acceleration, &Transform), With<Krill>>,
    obstacal_query: Query<&Obstacal, With<Obstacal>>,
) {
    for (mut krill_acceleration, krill_transform) in krill_query.iter_mut() {
        for obstacal in obstacal_query.iter() {
            let mut dist = match obstacal {
                Obstacal::Floor => krill_transform.translation.y - BOTTOM_BORDER,
                Obstacal::Ceiling => TOP_BORDER - krill_transform.translation.y,
                Obstacal::LeftWall => krill_transform.translation.x - LEFT_BORDER,
                Obstacal::RightWall => RIGHT_BORDER - krill_transform.translation.x,
            };

            dist = dist.max(ERROR_FROM_ZERO);

            if dist < BOID_PERCEPTION_RADIUS * 10. {
                let correction_vec = match obstacal {
                    Obstacal::Ceiling => -Vec2::Y,
                    Obstacal::RightWall => -Vec2::X,
                    Obstacal::Floor => Vec2::Y,
                    Obstacal::LeftWall => Vec2::X,
                };

                krill_acceleration.vec +=
                    (correction_vec * KRILL_AVOIDANCE_MAG * 10.) / ((dist).powf(0.7));
            }
        }
    }
}
