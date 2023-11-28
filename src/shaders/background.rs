// use bevy::{
//     prelude::*,
//     reflect::TypePath,
//     render::render_resource::{AsBindGroup, ShaderRef},
// };

// use bevy::{
//     prelude::*,
//     reflect::TypeUuid,
//     render::render_resource::{AsBindGroup, ShaderRef},
//     sprite::Material2d,
// };

// // #[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
// // pub struct CustomMaterial {}

// // impl Material for CustomMaterial {
// //     fn fragment_shader() -> ShaderRef {
// //         "shaders/animate_shader.wgsl".into()
// //     }
// // }

// impl Material2d for CustomMaterial {
//     fn fragment_shader() -> ShaderRef {
//         "shaders/custom_material.wgsl".into()
//     }
// }

// // Background shader material
// #[derive(AsBindGroup, TypeUuid, Debug, Clone)]
// #[uuid = "f690fdae-d598-45ab-8225-97e2a3f056e0"]
// pub struct CustomMaterial {
//     #[uniform(0)]
//     color: Color,
//     #[texture(1)]
//     #[sampler(2)]
//     color_texture: Option<Handle<Image>>,
// }
