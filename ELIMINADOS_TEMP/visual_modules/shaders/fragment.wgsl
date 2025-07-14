// 🌟 Fragment Shader Básico para Visualizador - MODIFICADO CON HOT-RELOAD
// Renderiza píxeles con efectos visuales dinámicos

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
    // Colores base
    var color = input.color;
    
    // ¡NUEVO! Efecto de ondas concéntricas
    let center = vec2<f32>(0.5, 0.5);
    let dist = length(input.uv - center);
    let wave_rings = sin(dist * 20.0 - uniforms.time * 4.0) * 0.3 + 0.7;
    
    // Efecto de pulsación con audio mejorado
    let pulse = sin(uniforms.time * 2.0 + input.world_position.x * 0.1) * 0.5 + 0.5;
    let audio_glow = uniforms.audio_level * pulse * wave_rings;
    
    // Intensidad del beat con efecto de flash
    let beat_flash = uniforms.beat_intensity * 0.5;
    
    // ¡NUEVO! Color dinámico basado en tiempo
    let time_color = vec3<f32>(
        sin(uniforms.time * 1.5) * 0.3 + 0.7,
        cos(uniforms.time * 2.0) * 0.3 + 0.7,
        sin(uniforms.time * 1.2 + 1.0) * 0.3 + 0.7
    );
    
    // Combinación final con nuevos efectos
    color = mix(color, time_color, 0.3) * (1.0 + audio_glow + beat_flash) * wave_rings;
    
    // Degradado UV para efectos adicionales
    let uv_factor = length(input.uv - vec2<f32>(0.5, 0.5));
    let radial_glow = 1.0 - smoothstep(0.0, 0.7, uv_factor);
    
    color = color * (0.8 + radial_glow * 0.2);
    
    return vec4<f32>(color, 1.0);
}
