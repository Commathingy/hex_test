use bevy::{render::{render_resource::{ShaderRef, AsBindGroup, RenderPipelineDescriptor, SpecializedMeshPipelineError, PolygonMode}, color::Color, mesh::MeshVertexBufferLayout}, pbr::{Material, MaterialPipeline, MaterialPipelineKey}, asset::Asset, reflect::TypePath};




#[derive(AsBindGroup, Debug, Clone, Asset, TypePath)]
pub struct OutlineMaterial{
    #[uniform(0)]
    pub outline_colour: Color,
}

impl Material for OutlineMaterial{
    fn fragment_shader() -> ShaderRef{
        "shaders/outline_material.wgsl".into()
    }

    fn depth_bias(&self) -> f32 {
        1.0
    }

    fn specialize(
        _pipeline: &MaterialPipeline<Self>,
        descriptor: &mut RenderPipelineDescriptor,
        _layout: &MeshVertexBufferLayout,
        _key: MaterialPipelineKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        descriptor.primitive.polygon_mode = PolygonMode::Line;
        Ok(())
    }
}

