#import bevy_pbr::forward_io::VertexOutput

struct OutlineMaterial {
    color: vec4<f32>,
};

@group(2) @binding(0) var<uniform> material: OutlineMaterial;

@fragment
fn fragment(
    mesh: VertexOutput,
) -> @location(0) vec4<f32> {
    return material.color;
}