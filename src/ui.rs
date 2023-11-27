use bevy::prelude::*;

use crate::consts::*;

#[derive(Component)]
struct TimeText;

fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    commands
        .spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                left: Val::Px(10.),
                top: Val::Px(10.),
                ..Default::default()
            },
            background_color: BackgroundColor(Color::NONE),
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn(TextBundle {
                    text: Text {
                        sections: vec![TextSection {
                            value: "Time: 0.0".to_string(),
                            style: TextStyle {
                                font: font.clone(),
                                font_size: 40.0,
                                color: Color::rgb(0.9, 0.9, 0.9),
                            },
                        }],
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(TimeText {});
        });
}

fn update_time_text(time: Res<Time>, mut query: Query<(&mut Text, &TimeText)>) {
    let secs = time.elapsed_seconds() - START_TIME_OFFSET;

    // don't do anything until song starts
    if secs < 0. {
        return;
    }

    for (mut text, _) in query.iter_mut() {
        text.sections[0].value = format!("Time: {:.2}", secs);
    }
}

pub struct UIPlugin;
impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_ui)
            .add_systems(Update, update_time_text);
    }
}
