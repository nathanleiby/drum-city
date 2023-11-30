use crate::{consts::*, types::SongConfig};
use crate::{score, types::*};
use bevy::prelude::*;

/// Keep textures and materials for arrows
#[derive(Resource)]
struct ArrowMaterialResource {
    red_image: Handle<Image>,
    blue_image: Handle<Image>,
    green_image: Handle<Image>,
    border_image: Handle<Image>,
}

// The approach here it to create a handle to the material, so that arrows share a reference vs each having their own copy.
impl FromWorld for ArrowMaterialResource {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        let red_handle = asset_server.load("images/arrow_red.png");
        let blue_handle = asset_server.load("images/arrow_blue.png");
        let green_handle = asset_server.load("images/arrow_green.png");
        let border_handle = asset_server.load("images/arrow_border.png");

        ArrowMaterialResource {
            red_image: red_handle,
            blue_image: blue_handle,
            green_image: green_handle,
            border_image: border_handle,
        }
    }
}

#[derive(Component)]
struct Arrow {
    speed: Speed,
    direction: Directions,
}

/// Spawn arrows
fn spawn_arrows(
    mut commands: Commands,
    mut song_config: ResMut<SongConfig>,
    materials: Res<ArrowMaterialResource>,
    time: Res<Time>,
) {
    let secs = time.elapsed_seconds_f64() - 3.;
    let secs_last = secs - time.delta_seconds_f64();

    // Count how many arrows got consumed
    let mut arrows_consumed = 0;

    for arrow in &song_config.arrows {
        if arrow.spawn_time > secs_last && arrow.spawn_time <= secs {
            let texture = match arrow.speed {
                Speed::Slow => materials.green_image.clone(),
                Speed::Medium => materials.blue_image.clone(),
                Speed::Fast => materials.red_image.clone(),
            };

            let mut transform =
                Transform::from_translation(Vec3::new(SPAWN_POSITION, arrow.direction.y(), 1.));
            transform.rotate(Quat::from_rotation_z(arrow.direction.rotation()));

            commands
                .spawn(SpriteBundle {
                    texture,
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(140., 140.)),
                        ..Default::default()
                    },
                    transform,
                    ..Default::default()
                })
                .insert(Arrow {
                    speed: arrow.speed,
                    direction: arrow.direction,
                });
            arrows_consumed += 1;
        } else {
            break;
        }
    }

    // remove arrows that were consumed
    for _ in 0..arrows_consumed {
        song_config.arrows.remove(0);
    }
}

/// Moves arrows forward
fn move_arrows(time: Res<Time>, mut query: Query<(&mut Transform, &Arrow)>) {
    for (mut transform, arrow) in query.iter_mut() {
        transform.translation.x += time.delta_seconds() * arrow.speed.value();

        // animate arrow falling after failing
        let distance_after_target = transform.translation.x - TARGET_POSITION - THRESHOLD;
        if distance_after_target > 0.02 {
            transform.translation.y -= time.delta_seconds() * distance_after_target * 2.;

            let scale = ((100. - distance_after_target / 3.) / 100.).max(0.2);
            transform.scale = Vec3::splat(scale);

            transform.rotate(Quat::from_rotation_z(
                -distance_after_target * arrow.speed.multiplier() / 460.,
            ));
        }
    }
}

pub struct ArrowsPlugin;
impl Plugin for ArrowsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ArrowMaterialResource>()
            .add_systems(OnEnter(AppState::Game), setup_target_arrows)
            .add_systems(Update, spawn_arrows.run_if(in_state(AppState::Game)))
            .add_systems(Update, move_arrows.run_if(in_state(AppState::Game)))
            .add_systems(Update, despawn_arrows.run_if(in_state(AppState::Game)))
            .add_systems(OnExit(AppState::Game), despawn_target_arrows);
    }
}
#[derive(Component)]
struct TargetArrow;

/// Setup target arrows
fn setup_target_arrows(mut commands: Commands, materials: Res<ArrowMaterialResource>) {
    for direction in &[
        Directions::Up,
        Directions::Down,
        Directions::Left,
        Directions::Right,
    ] {
        let mut transform =
            Transform::from_translation(Vec3::new(TARGET_POSITION, direction.y(), 1.));
        transform.rotate(Quat::from_rotation_z(direction.rotation()));

        commands
            .spawn(SpriteBundle {
                texture: materials.border_image.clone(),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(140., 140.)),
                    ..Default::default()
                },
                transform,
                ..Default::default()
            })
            .insert(TargetArrow {});
    }
}

fn despawn_arrows(
    mut commands: Commands,
    query: Query<(Entity, &Transform, &Arrow)>,
    keyboard_input: Res<Input<KeyCode>>,
    mut score: ResMut<score::Score>,
) {
    for (entity, transform, arrow) in query.iter() {
        let pos = transform.translation.x;
        if pos > TARGET_POSITION - THRESHOLD && pos < TARGET_POSITION + THRESHOLD {
            // pressed input with correct timing
            if arrow.direction.key_just_pressed(&keyboard_input) {
                commands.entity(entity).despawn();
                score.incr_correct(pos - TARGET_POSITION);
            }
        } else if pos > 2. * TARGET_POSITION {
            // left screen
            commands.entity(entity).despawn();
            score.incr_failed();
        }
    }
}

fn despawn_target_arrows(mut commands: Commands, query: Query<(Entity, &TargetArrow)>) {
    for (entity, _) in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
