// src/visual/vertex_types.rs
// Estructuras de vértices para notas, partículas y clusters compatibles con WGSL

use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct NoteVertex {
    pub position: [f32; 3],
    pub color: [f32; 3],
    pub uv: [f32; 2],
    pub world_position: [f32; 3],
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct ParticleVertex {
    pub position: [f32; 3],
    pub color: [f32; 3],
    pub uv: [f32; 2],
    pub world_position: [f32; 3],
    pub velocity: [f32; 3],
    pub life: f32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct ClusterVertex {
    pub position: [f32; 3],
    pub color: [f32; 3],
    pub uv: [f32; 2],
    pub world_position: [f32; 3],
    pub density: f32,
    pub spread: f32,
}
