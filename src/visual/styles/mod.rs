// 🎨 Estilos visuales
// Definiciones de colores, fuentes y estilos (futuro)

use nannou::prelude::*;

pub struct VisualTheme {
    /// Color de fondo general de la interfaz
    pub background_color: Srgb<u8>,
    /// Color de las líneas de la cuadrícula
    pub grid_color: Srgba<f32>,
    /// Color del texto principal
    pub text_color: Srgba<f32>,
    /// Color de acento para elementos activos
    pub accent_color: Srgba<f32>,
    /// Grosor de las líneas de la cuadrícula
    pub grid_line_thickness: f32,
    /// Tamaño base de fuente
    pub font_size: u32,
    /// Radio de las esquinas en elementos visuales
    pub border_radius: f32,
}

impl VisualTheme {
    pub fn dark_theme() -> Self {
        Self {
            background_color: srgb8(15, 15, 20),
            grid_color: srgba(0.3, 0.5, 0.8, 0.8),
            text_color: srgba(0.9, 0.9, 1.0, 1.0),
            accent_color: srgba(0.4, 0.6, 0.9, 0.9),
            grid_line_thickness: 1.5,
            font_size: 14,
            border_radius: 4.0,
        }
    }

    pub fn light_theme() -> Self {
        Self {
            background_color: srgb8(245, 245, 250),
            grid_color: srgba(0.6, 0.6, 0.6, 0.7),
            text_color: srgba(0.1, 0.1, 0.2, 1.0),
            accent_color: srgba(0.2, 0.4, 0.8, 1.0),
            grid_line_thickness: 1.0,
            font_size: 14,
            border_radius: 4.0,
        }
    }
}

impl Default for VisualTheme {
    fn default() -> Self {
        Self::dark_theme()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_is_dark_theme() {
        let theme = VisualTheme::default();
        assert_eq!(theme.background_color, srgb8(15, 15, 20));
    }

    #[test]
    fn test_light_theme_values() {
        let theme = VisualTheme::light_theme();
        assert_eq!(theme.background_color, srgb8(245, 245, 250));
    }
}
