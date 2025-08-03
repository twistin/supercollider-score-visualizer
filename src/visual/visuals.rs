// src/visuals.rs

/// Representa un elemento visual simple con una posición y color RGBA.
pub struct VisualElement {
    pub x: f32,
    pub y: f32,
    pub color: [f32; 4],
}

/// Renderiza un `VisualElement` usando el motor de dibujo de Nannou.
/// Actualmente imprime por consola; puede extenderse para dibujo gráfico.
pub fn render_element(element: &VisualElement) {
    println!("Renderizando en ({}, {})", element.x, element.y);
}

use nannou::prelude::*;

/// Renderiza visualmente el elemento como una elipse en pantalla.
pub fn render_element_nannou(draw: &Draw, element: &VisualElement) {
    draw.ellipse()
        .x_y(element.x, element.y)
        .radius(10.0)
        .rgba(
            element.color[0],
            element.color[1],
            element.color[2],
            element.color[3],
        );
}
