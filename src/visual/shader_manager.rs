// src/visual/shader_manager.rs

use nannou::draw::Draw;
use nannou::prelude::*;
use nannou::wgpu::Texture;
use crate::visual::visual_note::VisualNote;

/// Representa un manejador de shaders, opcionalmente cargados desde archivos WGSL o Nannou.
#[derive(Debug)]
pub struct ShaderManager {
    pub is_loaded: bool,
    pub quality: VisualQuality,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VisualQuality {
    Low,
    Medium,
    High,
}

impl ShaderManager {
    /// Indica si el ShaderManager es dummy (no tiene shaders cargados)
    pub fn is_dummy(&self) -> bool {
        !self.is_loaded
    }

    /// Renderiza una nota visual (dummy)
    pub fn render_visual_note(&self, draw: &Draw, note: &VisualNote) {
        // Renderizado simple por defecto
        draw.ellipse()
            .xy(note.position)
            .radius(note.size)
            .color(note.color);
    }
    /// Crea un ShaderManager con calidad alta por defecto
    pub fn new(_device: &wgpu::Device) -> Self {
        // Puedes ajustar la calidad predeterminada aquí
        Self::load(VisualQuality::High)
    }
    /// Devuelve una instancia dummy sin cargar shaders
    pub fn dummy() -> Self {
        ShaderManager {
            is_loaded: false,
            quality: VisualQuality::Low,
        }
    }

    /// Carga shaders según la calidad deseada
    pub fn load(quality: VisualQuality) -> Self {
        // Aquí se podría implementar carga de shaders reales si se desea
        ShaderManager {
            is_loaded: true,
            quality,
        }
    }

    /// Renderizado alternativo que se podría usar si se activa shading
    pub fn render_note_with_shader(
        &self,
        draw: &Draw,
        note: &VisualNote,
        _frame_texture: Option<&Texture>,
    ) {
        if !self.is_loaded {
            return;
        }

        // Renderizado simulado para shader: color y contorno especial
        draw.ellipse()
            .xy(note.position)
            .radius(note.size)
            .color(note.color)
            .stroke(rgba(1.0, 1.0, 1.0, 0.2))
            .stroke_weight(2.0);
    }
}
