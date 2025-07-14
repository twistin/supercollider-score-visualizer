// 🎆 Shader de Partículas para Efectos Visuales
// Renderiza partículas con efectos dinámicos de audio

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

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    let center = vec2<f32>(0.5, 0.5);
    let dist = length(input.uv - center);
    
    // Efecto de partícula circular
    let particle_radius = 0.4;
    let edge_softness = 0.1;
    let alpha = 1.0 - smoothstep(particle_radius - edge_softness, particle_radius, dist);
    
    // Color dinámico basado en audio
    let audio_color = vec3<f32>(
        uniforms.audio_level * 0.8 + 0.2,
        uniforms.beat_intensity * 0.6 + 0.4,
        sin(uniforms.time * 3.0) * 0.3 + 0.7
    );
    
    let final_color = mix(input.color, audio_color, uniforms.audio_level);
    
    // Brillo central
    let core_glow = 1.0 - smoothstep(0.0, 0.2, dist);
    let glow_color = final_color * (1.0 + core_glow * 2.0);
    
    return vec4<f32>(glow_color, alpha);
}
