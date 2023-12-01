use std::fs::read_dir;

use bevy::prelude::*;

use crate::{consts::AppState, types::load_config};

/// Keep textures and materials for arrows
#[derive(Resource)]
struct ButtonMaterials {
    font: Handle<Font>,
}

const NORMAL_COLOR: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_COLOR: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_COLOR: Color = Color::rgb(0.35, 0.75, 0.35);
const FONT_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);

impl FromWorld for ButtonMaterials {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        let font = asset_server.load("fonts/FiraSans-Bold.ttf");

        // let mut materials = world.get_resource_mut::<Assets<ColorMaterial>>().unwrap();
        ButtonMaterials { font }
    }
}

#[derive(Component)]
struct MenuUI;

fn setup_menu(mut commands: Commands, button_materials: Res<ButtonMaterials>) {
    let mut buttons: Vec<MenuButton> = get_songs()
        .iter()
        .map(|name| MenuButton::PlaySong(name.clone()))
        .collect();
    buttons.push(MenuButton::MakeMap);

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::FlexStart,
                    justify_content: JustifyContent::FlexStart,
                    ..default()
                },
                ..default()
            },
            MenuUI,
        ))
        .with_children(|parent| {
            for button in buttons {
                let name = button.name();
                parent
                    .spawn((
                        ButtonBundle {
                            style: Style {
                                width: Val::Px(350.0),
                                height: Val::Px(65.0),
                                margin: UiRect::all(Val::Auto),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            background_color: BackgroundColor(NORMAL_COLOR),
                            ..default()
                        },
                        button,
                    ))
                    .with_children(|parent| {
                        parent.spawn(TextBundle {
                            text: Text {
                                sections: vec![TextSection::new(
                                    name,
                                    TextStyle {
                                        font_size: 20.0,
                                        color: FONT_COLOR,
                                        font: button_materials.font.clone(),
                                        ..default()
                                    },
                                )],
                                ..default()
                            },
                            ..default()
                        });
                    });
            }
        });
}

fn despawn_menu(mut commands: Commands, query: Query<(Entity, &MenuUI)>) {
    for (entity, _) in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn update_button_color(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut background_color) in interaction_query.iter_mut() {
        let color = match *interaction {
            Interaction::Pressed => PRESSED_COLOR,
            Interaction::Hovered => HOVERED_COLOR,
            Interaction::None => NORMAL_COLOR,
        };

        *background_color = BackgroundColor(color);
    }
}

pub fn get_songs() -> Vec<String> {
    let paths = read_dir("assets/songs").unwrap();

    let mut vec = vec![];
    for dir_entry in paths {
        let path = dir_entry.unwrap().path();

        let file_extension = path.as_path().extension().unwrap();
        if file_extension == "toml" {
            let name_without_extension = path
                .as_path()
                .file_stem()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string();
            vec.push(name_without_extension);
        }
    }
    vec
}

#[derive(Component, Debug)]
pub enum MenuButton {
    MakeMap,
    PlaySong(String),
}

impl MenuButton {
    fn name(&self) -> String {
        match self {
            MenuButton::MakeMap => "Make Map".to_string(),
            MenuButton::PlaySong(name) => format!("Play song: {}", name),
        }
    }
}

pub fn button_press_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    interaction_query: Query<(&Interaction, &MenuButton), (Changed<Interaction>, With<Button>)>,
    mut app_state: ResMut<NextState<AppState>>,
) {
    for (interaction, button) in interaction_query.iter() {
        if *interaction == Interaction::Pressed {
            match button {
                MenuButton::MakeMap => {
                    app_state.set(AppState::MakeMap);
                    return;
                }
                MenuButton::PlaySong(song) => {
                    let config = load_config(format!("{}.toml", song).as_str(), &asset_server);
                    commands.insert_resource(config);
                    app_state.set(AppState::Game);
                    return;
                }
            }
        };
    }
}

pub struct MenuPlugin;
impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ButtonMaterials>()
            .add_systems(OnEnter(AppState::Menu), setup_menu)
            .add_systems(
                Update,
                (update_button_color, button_press_system).run_if(in_state(AppState::Menu)),
            )
            .add_systems(OnExit(AppState::Menu), despawn_menu);
    }
}
