use bevy::prelude::*;

use crate::{consts::*, score::Score};

#[derive(Component)]
struct UI;

#[derive(Component)]
struct TimeText;

fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    left: Val::Px(10.),
                    top: Val::Px(10.),
                    ..Default::default()
                },
                background_color: BackgroundColor(Color::NONE),
                ..Default::default()
            },
            UI,
        ))
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
    commands
        .spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                left: Val::Px(10.),
                bottom: Val::Px(10.),
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
                            value: "Score: 0. Corrects: 0. Fails: 0".to_string(),
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
                .insert(ScoreText {});
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

#[derive(Component)]
struct ScoreText;

// TODO: is ChangedRes still a thing? (maybe: https://bevy-cheatbook.github.io/programming/change-detection.html?highlight=changed#change-detection)
fn update_score_text(score: Res<Score>, mut query: Query<(&mut Text, &ScoreText)>) {
    for (mut text, _) in query.iter_mut() {
        text.sections[0].value = format!(
            "Score: {}. Corrects: {}. Fails: {}",
            score.get_score(),
            score.get_corrects(),
            score.get_fails()
        );
    }
}

fn despawn_ui(mut commands: Commands, query: Query<(Entity, &UI)>) {
    for (entity, _) in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub struct UIPlugin;
impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Game), setup_ui)
            .add_systems(Update, update_time_text.run_if(in_state(AppState::Game)))
            .add_systems(Update, update_score_text.run_if(in_state(AppState::Game)))
            .add_systems(OnExit(AppState::Game), despawn_ui);
    }
}
