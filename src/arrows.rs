use crate::types::*;
use crate::{consts::*, types::SongConfig};
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
    }
}

pub struct ArrowsPlugin;
impl Plugin for ArrowsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ArrowMaterialResource>()
            .add_systems(Startup, setup_target_arrows)
            .add_systems(Update, spawn_arrows)
            .add_systems(Update, move_arrows);
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
