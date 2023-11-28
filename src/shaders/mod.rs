mod background;

use bevy::{
    prelude::*,
    reflect::TypeUuid,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::{Material2d, Material2dPlugin, MaterialMesh2dBundle},
};

// #[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
// pub struct CustomMaterial {}

// impl Material for CustomMaterial {
//     fn fragment_shader() -> ShaderRef {
//         "shaders/animate_shader.wgsl".into()
//     }
// }

impl Material2d for CustomMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/myshader.wgsl".into()

        //// watt.. all of the below don't work
        // "shaders/myshader_2d.wgsl".into() // Shadertoy mode??
        // "shaders/uv.wgsl".into()
        // "shaders/aura.wgsl".into()
        // "shaders/custom_material.wgsl".into()
        // "shaders/animate_shader.wgsl".into()
        // "shaders/circle.wgsl".into()
        // "shaders/sky_shader.wgsl".into()
    }
}

// Background shader material
#[derive(Asset, AsBindGroup, TypeUuid, Debug, Clone, TypePath)]
#[uuid = "f690fdae-d598-45ab-8225-97e2a3f056e0"]
pub struct CustomMaterial {
    #[uniform(0)]
    color: Color,
    #[texture(1)]
    #[sampler(2)]
    color_texture: Option<Handle<Image>>,
}

pub struct ShadersPlugin;
impl Plugin for ShadersPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(Material2dPlugin::<CustomMaterial>::default())
            .add_systems(Startup, setup);
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CustomMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // cube
    // commands.spawn(MaterialMeshBundle {
    //     mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
    //     transform: Transform::from_xyz(0.0, 0.5, 0.0),
    //     material: materials.add(CustomMaterial {}),
    //     ..default()
    // });
    commands.spawn(MaterialMesh2dBundle {
        // mesh: meshes.add(shape::Plane { size: 3.0 }.into()).into(),
        mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
        transform: Transform::default().with_scale(Vec3::splat(100.)),
        // material: materials.add(ColorMaterial::from(Color::TURQUOISE)),
        material: materials.add(CustomMaterial {
            color: Color::BLUE,
            color_texture: Some(asset_server.load("images/ship_C.png")),
        }),
        ..default()
    });
}

// Background
