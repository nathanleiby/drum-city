use bevy::prelude::*;

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
            parent
                .spawn(ButtonBundle {
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
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection::new(
                                "Play".to_string(),
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
        });
}

pub struct MenuPlugin;
impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ButtonMaterials>()
            .add_systems(Startup, setup_menu);
    }
}
