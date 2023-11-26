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
struct Arrow {}

#[derive(Resource)]
struct SpawnTimer(Timer);

/// Spawn arrows
fn spawn_arrows(
    mut commands: Commands,
    materials: Res<ArrowMaterialResource>,
    time: Res<Time>,
    mut spawn_timer: ResMut<SpawnTimer>,
) {
    // add elapsed time to timer.
    // if there's still time remaining in an interval, do nothing
    if !spawn_timer.0.tick(time.delta()).just_finished() {
        return;
    }

    let transform = Transform::from_translation(Vec3::new(-400., 0., 1.));
    commands
        .spawn(SpriteBundle {
            // texture: asset_server.load("branding/bevy_bird_dark.png"),
            texture: materials.red_image.clone(),
            sprite: Sprite {
                custom_size: Some(Vec2::new(140., 140.)),
                ..Default::default()
            },
            transform,
            ..Default::default()
        })
        .insert(Arrow {});
}

/// Moves arrows forward
fn move_arrows(time: Res<Time>, mut query: Query<(&mut Transform, &Arrow)>) {
    for (mut transform, _arrow) in query.iter_mut() {
        transform.translation.x += 200. * time.delta_seconds();
    }
}

pub struct ArrowsPlugin;
impl Plugin for ArrowsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ArrowMaterialResource>()
            .insert_resource(SpawnTimer(Timer::from_seconds(1.0, TimerMode::Repeating)))
            .add_systems(Update, spawn_arrows)
            .add_systems(Update, move_arrows);
    }
}
