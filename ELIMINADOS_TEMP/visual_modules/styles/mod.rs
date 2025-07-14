// 🎨 Estilos visuales
// Definiciones de colores, fuentes y estilos (futuro)

use nannou::prelude::*;

pub struct VisualTheme {
    pub background_color: Srgb<u8>,
    pub grid_color: Srgba<f32>,
    pub text_color: Srgba<f32>,
    pub accent_color: Srgba<f32>,
}

impl VisualTheme {
    pub fn dark_theme() -> Self {
        Self {
            background_color: srgb8(15, 15, 20),
            grid_color: srgba(0.3, 0.5, 0.8, 0.8),
            text_color: srgba(0.9, 0.9, 1.0, 1.0),
            accent_color: srgba(0.4, 0.6, 0.9, 0.9),
        }
    }
}

impl Default for VisualTheme {
    fn default() -> Self {
        Self::dark_theme()
    }
}
