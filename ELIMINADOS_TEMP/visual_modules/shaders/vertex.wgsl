// 🎨 Vertex Shader Básico para Visualizador
// Maneja la transformación de vértices y pasa datos al fragment shader

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) color: vec3<f32>,
    @location(2) uv: vec2<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>,
    @location(1) uv: vec2<f32>,
    @location(2) world_position: vec3<f32>,
}

struct Uniforms {
    mvp_matrix: mat4x4<f32>,
    time: f32,
    audio_level: f32,
    beat_intensity: f32,
    _padding: f32,
}

@group(0) @binding(0)
var<uniform> uniforms: Uniforms;

@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
    var output: VertexOutput;
    
    // Transformación básica con matrix MVP
    let world_pos = vec4<f32>(input.position, 1.0);
    output.clip_position = uniforms.mvp_matrix * world_pos;
    
    // Pasa los datos al fragment shader
    output.color = input.color;
    output.uv = input.uv;
    output.world_position = input.position;
    
    return output;
}
