// 📁 src/visual/uniforms.rs
// Define una estructura Uniforms para shaders compatibles con WGSL.
// Debe coincidir con la estructura WGSL usada en vertex.wgsl y particles.wgsl.

use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct Uniforms {
    pub mvp_matrix: [[f32; 4]; 4],
    pub time: f32,
    pub audio_level: f32,
    pub beat_intensity: f32,
    pub glow_strength: f32,
    pub particle_center: [f32; 2],
    pub particle_radius: f32,
    pub edge_softness: f32,
    pub mix_strength: f32,
    pub _padding: [f32; 3], // Para alineación de 16 bytes
}

impl Uniforms {
    pub fn new() -> Self {
        Self {
            mvp_matrix: [[1.0, 0.0, 0.0, 0.0],
                        [0.0, 1.0, 0.0, 0.0],
                        [0.0, 0.0, 1.0, 0.0],
                        [0.0, 0.0, 0.0, 1.0]],
            time: 0.0,
            audio_level: 0.0,
            beat_intensity: 0.0,
            glow_strength: 0.0,
            particle_center: [0.0, 0.0],
            particle_radius: 1.0,
            edge_softness: 0.1,
            mix_strength: 0.0,
            _padding: [0.0; 3],
        }
    }

    pub fn update_from_audio(&mut self, time: f32, audio_level: f32, beat_intensity: f32) {
        self.time = time;
        self.audio_level = audio_level;
        self.beat_intensity = beat_intensity;
    }
}
