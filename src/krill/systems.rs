use std::ops::Range;

use bevy::prelude::*;

use rand::prelude::*;

const KRILL_SIZE: f32 = 3.;

use crate::{
    assets::{AnimationIndices, AnimationTimer, ImageAssets},
    map::{BOTTOM_BORDER, LEFT_BORDER, RIGHT_BORDER, TOP_BORDER},
    DebugEvent,
};

#[derive(Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Krill;



#[derive(Bundle)]
pub struct KrillBundle {
    krill: Krill,
    sprite: SpriteSheetBundle,
}

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
pub enum KrillState {
    #[default]
    Idle,
    Dead,
}



pub fn spawn_krill(mut commands: Commands, image_assets: Res<ImageAssets>) {
    const STARTING_KRILL: u16 = 150;
    const SPAWN_X_RANGE: Range<f32> = LEFT_BORDER..RIGHT_BORDER;
    const SPAWN_Y_RANGE: Range<f32> = BOTTOM_BORDER..TOP_BORDER;
    let mut rand_gen = thread_rng();

    for _ in 0..STARTING_KRILL {
        let random_x = rand_gen.gen_range(SPAWN_X_RANGE);
        let random_y = rand_gen.gen_range(SPAWN_Y_RANGE);

        commands.spawn((
            KrillBundle {
                krill: Krill,
                sprite: SpriteSheetBundle {
                    sprite: TextureAtlasSprite {
                        index: 0,
                        custom_size: Some(Vec2::new(KRILL_SIZE, KRILL_SIZE)),
                        ..Default::default()
                    },
                    texture_atlas: image_assets.krill.clone(),
                    transform: Transform::from_translation(Vec3::new(random_x, random_y, 1.)),
                    ..Default::default()
                },
            },
            Name::new("Krill"),
            AnimationIndices { first: 0, last: 1 },
            AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        ));
    }
}

pub fn debug_krill(
    mut debug_event_reader: EventReader<DebugEvent>,
    krill_query: Query<&Transform, With<Krill>>,
) {
    for _event in debug_event_reader.read() {
        info!("{:?}", krill_query);
        let krill_transform = krill_query.get_single().unwrap();
        info!("{:?}", krill_transform);
        info!("{:?}", krill_transform.forward());
    }
}

pub fn krill_idle_movement(mut krill_query: Query<&mut Transform, With<Krill>>, time: Res<Time>) {
    for mut krill_transform in krill_query.iter_mut() {
        const IDLE_HIEGHT_SCALAR: f32 = 0.005;
        const IDLE_FREQ_SCALAR: f32 = 0.1;

        // info!("{:?}", (time.elapsed_seconds() / IDLE_FREQ_SCALAR).sin() * IDLE_HIEGHT_SCALAR;);

        krill_transform.translation.y += (time.elapsed_seconds()
            - (krill_transform.translation.x / 5.) / IDLE_FREQ_SCALAR)
            .sin()
            * IDLE_HIEGHT_SCALAR;
    }
}

const ROTATION_SPEED: f32 = 0.5;

pub fn krill_death(
    mut krill_query: Query<(&mut Transform, &mut Sprite), With<Krill>>,
    time: Res<Time>,
    mut commands: Commands,
) {
    let elapsed_seconds = time.elapsed_seconds();
    commands.spawn((
        

        for (mut krill_transform, mut krill_change) in krill_query.iter_mut() {
            if elapsed_seconds <= 1.0 {
                // Rotate smoothly until upside down
                let target_rotation =
                    Quat::from_rotation_x(-std::f32::consts::PI); // Upside down
                let rotation =
                    Quat::from_rotation_x(elapsed_seconds * ROTATION_SPEED.to_radians());
                let interpolated_rotation =
                    Quat::slerp(rotation, target_rotation, elapsed_seconds);

                krill_transform.rotation = interpolated_rotation;



                let light_blue = Color::rgb(0.2, 0.2, 1.0); // Adjust as needed

                // Store the original color
                let original_color = krill_change.color.clone();

                // Adjust the speed of color change by introducing a color change speed factor
                let color_change_speed_factor = 0.4; // Adjust as needed

                // Manually interpolate color components
                let factor = elapsed_seconds / (1.0 * color_change_speed_factor); // Assuming 1.0 seconds for the color transition
                krill_change.color.set_r((1.0 - factor) * original_color.r() + factor * light_blue.r());
                krill_change.color.set_g((1.0 - factor) * original_color.g() + factor * light_blue.g());
                krill_change.color.set_b((1.0 - factor) * original_color.b() + factor * light_blue.b());
            }

            

            
            
        },
    ));
}
