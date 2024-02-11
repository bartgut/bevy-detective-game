use bevy::pbr::{MaterialPipeline, MaterialPipelineKey};
use bevy::prelude::*;
use bevy::render::mesh::MeshVertexBufferLayout;
use bevy::render::render_resource::{
    AsBindGroup, RenderPipelineDescriptor, ShaderRef, SpecializedMeshPipelineError,
};
use bevy::sprite::{Material2d, Material2dKey};

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct FogMaterial {
    #[uniform(0)]
    pub fog_size: Vec2,
    #[uniform(0)]
    pub fog_color: Color,
    #[uniform(0)]
    pub fog_speed: Vec2,
}

impl Material2d for FogMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/fog.wgsl".into()
    }
}
