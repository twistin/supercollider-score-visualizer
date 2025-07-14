// src/visual/shader_manager.rs
// Sistema de gestión de shaders y integración con mapeos profesionales de audio a visual
// Combina las funciones profesionales de conversión con el pipeline de renderizado

use nannou::prelude::*;
use crate::visual::audio_visual_mapping::{VisualNote, VisualStyle, TextureStyle};
use crate::visual::audio_visual_mapping_pro::{
    ProAudioVisualMapper, EventKind, VisualShape, ColorPalette, ProMappingConfig
};

/// Punto de rastro para efectos de desvanecimiento
#[derive(Debug, Clone)]
pub struct TrailPoint {
    pub position: Vec2,
    pub color: Srgb<u8>,
    pub size: f32,
    pub age: f32,
    pub max_age: f32,
    pub birth_amplitude: f32,
}

/// Estilos artísticos disponibles
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ArtisticStyle {
    /// Estilo híbrido que combina elementos de diferentes estilos
    Hybrid,
    /// Estilo basado en Kandinsky - formas orgánicas y colores emocionales
    Kandinsky,
    /// Estilo basado en Xenakis - patrones matemáticos y estructuras estocásticas
    Xenakis,
    /// Estilo minimalista - formas simples y colores limitados
    Minimalist,
    /// Estilo expresionista - formas dramáticas y colores intensos
    Expressionist,
}

/// Configuración para el sistema de shaders
#[derive(Debug, Clone)]
pub struct ShaderConfig {
    /// Calidad de renderizado (afecta número de partículas, resolución, etc.)
    pub quality: RenderQuality,
    /// Activar efectos de post-procesado
    pub enable_post_processing: bool,
    /// Configuración de anti-aliasing
    pub anti_aliasing: AntiAliasingMode,
    /// Factor de escala para efectos de glow
    pub glow_intensity: f32,
    /// Activar motion blur
    pub motion_blur: bool,
    /// Intensidad del motion blur
    pub blur_strength: f32,
    /// Efectos artísticos inspirados en Xenakis/Kandinsky
    pub artistic_effects: ArtisticEffects,
}

/// Calidad de renderizado
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RenderQuality {
    Low,    // Para dispositivos con recursos limitados
    Medium, // Configuración balanceada
    High,   // Máxima calidad visual
    Ultra,  // Para demostraciones y presentaciones
}

/// Configuración de anti-aliasing
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AntiAliasingMode {
    None,
    FXAA,
    MSAA2x,
    MSAA4x,
    MSAA8x,
}

impl Default for ShaderConfig {
    fn default() -> Self {
        Self {
            quality: RenderQuality::Medium,
            enable_post_processing: true,
            anti_aliasing: AntiAliasingMode::FXAA,
            glow_intensity: 1.5,
            motion_blur: false,
            blur_strength: 0.8,
            artistic_effects: ArtisticEffects::default(),
        }
    }
}

/// Modos de mezcla artística para efectos avanzados
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BlendMode {
    Normal,
    Additive,   // Suma de colores - más brillante
    Multiply,   // Multiplicación - más oscuro
    Screen,     // Pantalla - más claro
    Overlay,    // Superposición - contraste
}

impl Default for BlendMode {
    fn default() -> Self {
        BlendMode::Normal
    }
}

/// Manager principal para shaders y renderizado profesional
pub struct ShaderManager {
    /// Configuración del manager
    config: ShaderConfig,
    /// Mapeador profesional de audio a visual
    pro_mapper: ProAudioVisualMapper,
    /// Cache de formas calculadas para optimización
    shape_cache: std::collections::HashMap<String, VisualShape>,
    /// Tiempo actual para cálculos de animación
    current_time: f32,
    /// Dimensiones de la ventana para cálculos de posición
    window_size: Vec2,
    /// Buffer de rastros para efectos de desvanecimiento
    trail_buffer: std::collections::HashMap<u64, Vec<TrailPoint>>,
    /// Historial de posiciones para motion blur artístico
    position_history: std::collections::HashMap<u64, Vec<Vec2>>,
}

impl ShaderManager {
    /// Crea un nuevo ShaderManager
    pub fn new(
        shader_config: ShaderConfig, 
        mapping_config: ProMappingConfig,
        color_palette: ColorPalette
    ) -> Self {
        Self {
            config: shader_config,
            pro_mapper: ProAudioVisualMapper::new(mapping_config, color_palette),
            shape_cache: std::collections::HashMap::new(),
            current_time: 0.0,
            window_size: vec2(1200.0, 800.0),
            trail_buffer: std::collections::HashMap::new(),
            position_history: std::collections::HashMap::new(),
        }
    }
    
    /// Crea un ShaderManager con configuración por defecto
    pub fn default() -> Self {
        Self::new(
            ShaderConfig::default(),
            ProMappingConfig::default(),
            ColorPalette::Classical,
        )
    }
    
    /// Actualiza el tiempo actual para animaciones
    pub fn update_time(&mut self, time: f32) {
        self.current_time = time;
        self.pro_mapper.set_start_time(time);
        
        // Actualizar rastros y limpiar expirados
        self.update_trail_buffer();
    }
    
    /// **Actualiza el buffer de rastros y elimina puntos expirados**
    fn update_trail_buffer(&mut self) {
        let delta_time = 1.0 / 60.0; // Asumiendo 60 FPS
        
        // Actualizar edad de todos los puntos de rastro
        for (_, trail_points) in self.trail_buffer.iter_mut() {
            for point in trail_points.iter_mut() {
                point.age += delta_time;
            }
            
            // Eliminar puntos expirados
            trail_points.retain(|point| point.age < point.max_age);
        }
        
        // Eliminar rastros completamente vacíos
        self.trail_buffer.retain(|_, points| !points.is_empty());
    }
    
    /// **Añade un punto de rastro para una nota**
    fn add_trail_point(&mut self, note_id: u64, position: Vec2, color: Srgb<u8>, size: f32, amplitude: f32) {
        let trail_point = TrailPoint {
            position,
            color,
            size,
            age: 0.0,
            max_age: self.config.artistic_effects.trail_duration,
            birth_amplitude: amplitude,
        };
        
        self.trail_buffer
            .entry(note_id)
            .or_insert_with(Vec::new)
            .push(trail_point);
        
        // Limitar número de puntos por rastro para rendimiento
        if let Some(points) = self.trail_buffer.get_mut(&note_id) {
            if points.len() > 50 {  // Máximo 50 puntos por rastro
                points.remove(0);  // Remover el más antiguo
            }
        }
    }
    
    /// **Actualiza historial de posiciones para interpolación**
    fn update_position_history(&mut self, note_id: u64, position: Vec2) {
        let history = self.position_history
            .entry(note_id)
            .or_insert_with(Vec::new);
        
        history.push(position);
        
        // Mantener solo las últimas 10 posiciones
        if history.len() > 10 {
            history.remove(0);
        }
    }
    
    /// Actualiza el tamaño de la ventana
    pub fn update_window_size(&mut self, size: Vec2) {
        self.window_size = size;
    }
    
    /// **Función principal de conversión de audio a VisualNote usando mapeos profesionales**
    /// 
    /// Esta función integra todas las funciones profesionales de mapeo:
    /// - freq_to_x_scroll(): posición X con scroll temporal
    /// - amp_to_opacity(): opacidad perceptual logarítmica
    /// - freq_to_color(): color basado en paletas artísticas
    /// - kind_to_shape(): forma según tipo de evento musical
    pub fn create_professional_visual_note(
        &mut self,
        freq: f32,
        amplitude: f32,
        duration: f32,
        event_kind: EventKind,
        birth_time: f32,
        instrument: &str,
    ) -> VisualNote {
        // **1. Calcular posición X usando scroll temporal profesional**
        let pos_x = self.pro_mapper.freq_to_x_scroll(freq, self.current_time, birth_time);
        
        // **2. Calcular posición Y usando frecuencia (escala logarítmica)**
        let pos_y = self.freq_to_y_logarithmic(freq);
        
        // **3. Calcular opacidad usando conversión perceptual**
        let opacity = self.pro_mapper.amp_to_opacity(amplitude);
        
        // **4. Calcular color usando paletas artísticas**
        let color = self.pro_mapper.freq_to_color(freq, amplitude);
        
        // **5. Determinar forma visual según tipo de evento**
        let visual_shape = self.pro_mapper.kind_to_shape(&event_kind, freq, amplitude, duration);
        
        // **6. Convertir VisualShape a VisualStyle y calcular tamaño**
        let (visual_style, base_size) = self.visual_shape_to_style(&visual_shape);
        
        // **7. Calcular parámetros de textura y animación**
        let texture_style = self.determine_texture_style(&event_kind, amplitude);
        let (scale_pulse, rotation_speed) = self.calculate_animation_params(&event_kind, freq, amplitude);
        
        // **8. Construir VisualNote con todos los parámetros profesionales**
        VisualNote {
            // Propiedades de audio fundamentales
            frequency: freq,
            amplitude,
            duration,
            instrument: instrument.to_string(),
            
            // Propiedades temporales
            timestamp: birth_time as f64,
            age: 0.0,
            lifetime_progress: 0.0,
            
            // Propiedades espaciales (usando mapeos profesionales)
            position: vec2(pos_x, pos_y),
            velocity: vec2(0.0, 0.0),
            target_position: vec2(pos_x, pos_y),
            
            // Propiedades visuales (usando conversiones profesionales)
            color,
            secondary_color: color, // Por ahora usar el mismo color
            size: base_size,
            size_variation: amplitude * 0.3,
            
            // Propiedades de forma y estilo
            visual_style,
            energy_level: self.amplitude_to_energy_level(amplitude),
            texture_style,
            
            // Propiedades de animación
            rotation: 0.0,
            rotation_speed,
            scale_pulse,
            opacity,
            
            // Metadatos
            note_id: self.generate_note_id(freq, amplitude, birth_time),
            layer: crate::visual::audio_visual_mapping::VisualLayer::Melody,
            priority: (amplitude * 255.0) as u8,
        }
    }
    
    /// **Posición Y logarítmica basada en frecuencia**
    /// 
    /// Utiliza escala logarítmica para posicionar notas en Y,
    /// siguiendo la percepción musical de las octavas.
    fn freq_to_y_logarithmic(&self, freq: f32) -> f32 {
        let config = self.pro_mapper.get_config();
        
        // Convertir a escala logarítmica (base 2 para octavas)
        let freq_log = freq.max(config.freq_min).log2();
        let freq_min_log = config.freq_min.log2();
        let freq_max_log = config.freq_max.log2();
        
        // Normalizar y mapear a coordenadas Y
        let normalized = (freq_log - freq_min_log) / (freq_max_log - freq_min_log);
        let y_range = config.window_height * 0.8; // Usar 80% de la altura
        let y_offset = config.window_height * 0.1; // 10% de margen superior/inferior
        
        -y_range * 0.5 + y_offset + (normalized * y_range)
    }
    
    /// **Conversión de VisualShape profesional a VisualStyle de renderizado**
    fn visual_shape_to_style(&self, shape: &VisualShape) -> (VisualStyle, f32) {
        match shape {
            VisualShape::Circle { radius } => {
                (VisualStyle::Circle, *radius * 2.0)
            },
            
            VisualShape::Rectangle { width, height } => {
                // Usar polígono de 4 lados para rectángulos
                (VisualStyle::Polygon(4), (*width + *height) * 0.5)
            },
            
            VisualShape::Ellipse { width, height } => {
                // Círculo escalado para simular elipse
                (VisualStyle::Circle, (*width + *height) * 0.5)
            },
            
            VisualShape::Curve { control_points } => {
                // Usar estilo wave para curvas
                let avg_distance = control_points.iter()
                    .zip(control_points.iter().skip(1))
                    .map(|(p1, p2)| p1.distance(*p2))
                    .sum::<f32>() / control_points.len().max(1) as f32;
                (VisualStyle::Wave, avg_distance)
            },
            
            VisualShape::Polygon { sides, radius } => {
                (VisualStyle::Polygon(*sides), *radius * 2.0)
            },
            
            VisualShape::Star { points, outer_radius, .. } => {
                (VisualStyle::Star(*points), *outer_radius * 2.0)
            },
            
            VisualShape::Blob { points } => {
                // Calcular tamaño promedio del blob
                let avg_radius = points.iter()
                    .map(|p| p.length())
                    .sum::<f32>() / points.len().max(1) as f32;
                (VisualStyle::Blob, avg_radius * 2.0)
            },
            
            VisualShape::Line { start, end, thickness } => {
                let length = start.distance(*end);
                (VisualStyle::Beam, length.max(*thickness))
            },
        }
    }
    
    /// **Determina estilo de textura basado en tipo de evento**
    fn determine_texture_style(&self, event_kind: &EventKind, amplitude: f32) -> TextureStyle {
        match event_kind {
            EventKind::Note => {
                // Notas suaves con gradiente
                if amplitude > 0.7 {
                    TextureStyle::Glow
                } else {
                    TextureStyle::Gradient
                }
            },
            
            EventKind::Chord => {
                // Acordes con brillo
                TextureStyle::Glow
            },
            
            EventKind::Percussion => {
                // Percusión sólida y contundente
                TextureStyle::Solid
            },
            
            EventKind::Sustained => {
                // Sonidos sostenidos con gradiente suave
                TextureStyle::Gradient
            },
            
            EventKind::Transient => {
                // Transitorios brillantes
                TextureStyle::Glow
            },
            
            EventKind::Noise => {
                // Ruido con textura sólida
                TextureStyle::Solid
            },
            
            EventKind::Control => {
                // Eventos de control sutiles
                TextureStyle::Gradient
            },
            
            EventKind::Custom(_) => {
                // Por defecto gradiente
                TextureStyle::Gradient
            },
        }
    }
    
    /// **Calcula parámetros de animación según características del evento**
    fn calculate_animation_params(&self, event_kind: &EventKind, freq: f32, amplitude: f32) -> (f32, f32) {
        let base_pulse = 1.0 + (amplitude * 0.3); // Pulso basado en amplitud
        let base_rotation = freq / 1000.0; // Rotación basada en frecuencia
        
        match event_kind {
            EventKind::Note => {
                // Notas con pulso suave
                (base_pulse, base_rotation * 0.5)
            },
            
            EventKind::Chord => {
                // Acordes con pulso lento y rotación mínima
                (base_pulse * 0.8, base_rotation * 0.2)
            },
            
            EventKind::Percussion => {
                // Percusión con pulso intenso sin rotación
                (base_pulse * 1.5, 0.0)
            },
            
            EventKind::Sustained => {
                // Sonidos sostenidos con pulso muy suave
                (1.0 + amplitude * 0.1, base_rotation * 0.3)
            },
            
            EventKind::Transient => {
                // Transitorios con animación rápida
                (base_pulse * 2.0, base_rotation * 2.0)
            },
            
            EventKind::Noise => {
                // Ruido con animación errática
                let chaos_factor = 1.0 + amplitude;
                (base_pulse * chaos_factor, base_rotation * chaos_factor)
            },
            
            EventKind::Control => {
                // Eventos de control con animación mínima
                (1.0, 0.0)
            },
            
            EventKind::Custom(_) => {
                // Por defecto
                (base_pulse, base_rotation)
            },
        }
    }
    
    /// **Renderiza una VisualNote usando shaders optimizados con efectos artísticos**
    pub fn render_visual_note(&self, draw: &Draw, note: &VisualNote) {
        // Aplicar transformaciones basadas en configuración de calidad
        let quality_factor = self.get_quality_factor();
        
        // Calcular efectos según configuración
        let effective_size = note.size * note.scale_pulse;
        let effective_opacity = note.opacity * note.lifetime_progress.max(0.3);
        
        // **1. Renderizar rastros primero (background)**
        self.render_trails(draw, note.note_id);
        
        // **2. Renderizar efectos artísticos de fondo**
        if self.config.artistic_effects.halo_effect {
            self.render_artistic_halo(draw, note, effective_size, effective_opacity, quality_factor);
        }
        
        // **3. Renderizar la nota principal con efectos**
        if self.config.enable_post_processing {
            self.render_with_artistic_effects(draw, note, effective_size, effective_opacity, quality_factor);
        } else {
            self.render_basic_artistic(draw, note, effective_size, effective_opacity);
        }
        
        // **4. Añadir punto de rastro para el próximo frame**
        // Note: Esto debe ser llamado externamente ya que necesitamos &mut self
    }
    
    /// **Renderizado básico sin efectos**
    fn render_basic(&self, draw: &Draw, note: &VisualNote, size: f32, opacity: f32) {
        let color = rgba(
            note.color.red as f32 / 255.0,
            note.color.green as f32 / 255.0,
            note.color.blue as f32 / 255.0,
            opacity,
        );
        
        let drawing = draw
            .translate(vec3(note.position.x, note.position.y, 0.0))
            .rotate(note.rotation)
            .scale(note.scale_pulse);
        
        // Renderizar forma básica
        match &note.visual_style {
            VisualStyle::Circle => {
                drawing.ellipse()
                    .w_h(size, size)
                    .color(color);
            },
            VisualStyle::Polygon(sides) => {
                // Simular polígono con puntos
                let points: Vec<Vec2> = (0..*sides)
                    .map(|i| {
                        let angle = (i as f32 / *sides as f32) * 2.0 * PI;
                        vec2(
                            (size * 0.5) * angle.cos(),
                            (size * 0.5) * angle.sin(),
                        )
                    })
                    .collect();
                
                drawing.polygon()
                    .points(points)
                    .color(color);
            },
            // Agregar más formas según necesidad
            _ => {
                // Fallback a círculo
                drawing.ellipse()
                    .w_h(size, size)
                    .color(color);
            }
        }
    }
    
    /// **Renderiza rastros con desvanecimiento artístico**
    fn render_trails(&self, draw: &Draw, note_id: u64) {
        if let Some(trail_points) = self.trail_buffer.get(&note_id) {
            for point in trail_points.iter() {
                // Calcular fade basado en edad
                let fade_factor = 1.0 - (point.age / point.max_age);
                let trail_opacity = (fade_factor * fade_factor).max(0.05); // Curva cuadrática
                
                // Tamaño que se reduce con el tiempo
                let trail_size = point.size * (0.3 + 0.7 * fade_factor);
                
                // Color que se desvanece
                let trail_color = rgba(
                    point.color.red as f32 / 255.0,
                    point.color.green as f32 / 255.0,
                    point.color.blue as f32 / 255.0,
                    trail_opacity * 0.6, // Rastros más sutiles
                );
                
                // Renderizar punto de rastro con efecto Kandinsky (orgánico)
                if self.config.artistic_effects.style == ArtisticStyle::Kandinsky {
                    self.render_kandinsky_trail_point(draw, point, trail_color, trail_size, fade_factor);
                } else {
                    // Rastro básico
                    draw.ellipse()
                        .xy(point.position)
                        .w_h(trail_size, trail_size)
                        .color(trail_color);
                }
            }
        }
    }
    
    /// **Renderiza punto de rastro estilo Kandinsky con distorsión orgánica**
    fn render_kandinsky_trail_point(&self, draw: &Draw, point: &TrailPoint, color: Rgba, size: f32, fade_factor: f32) {
        let distortion = self.config.artistic_effects.kandinsky_distortion * fade_factor;
        
        // Crear forma orgánica distorsionada
        let blob_points = self.generate_organic_blob(point.position, size, distortion);
        
        draw.polygon()
            .points(blob_points)
            .color(color);
    }
    
    /// **Genera forma orgánica para efectos Kandinsky**
    fn generate_organic_blob(&self, center: Vec2, radius: f32, distortion: f32) -> Vec<Vec2> {
        let point_count = 8;
        let mut points = Vec::new();
        
        for i in 0..point_count {
            let angle = (i as f32 / point_count as f32) * 2.0 * PI;
            
            // Distorsión orgánica basada en tiempo y posición
            let time_factor = self.current_time * 0.5;
            let noise = (angle * 3.0 + time_factor).sin() * distortion;
            let organic_radius = radius * (1.0 + noise);
            
            points.push(vec2(
                center.x + organic_radius * angle.cos(),
                center.y + organic_radius * angle.sin(),
            ));
        }
        
        points
    }
    
    /// **Renderiza halo artístico concéntrico**
    fn render_artistic_halo(&self, draw: &Draw, note: &VisualNote, size: f32, opacity: f32, quality: f32) {
        let effects = &self.config.artistic_effects;
        let ring_count = (effects.halo_rings as f32 * quality) as u32;
        
        for i in 0..ring_count {
            let ring_factor = (i + 1) as f32;
            let ring_radius = size * (1.0 + ring_factor * 0.4);
            let ring_opacity = (opacity * 0.15 / ring_factor).max(0.01);
            
            // Color del halo con variación artística
            let halo_color = self.calculate_artistic_halo_color(note, ring_factor, ring_opacity);
            
            match effects.style {
                ArtisticStyle::Xenakis => {
                    // Halo estocástico disperso
                    self.render_xenakis_halo_ring(draw, note.position, ring_radius, halo_color);
                }
                ArtisticStyle::Kandinsky => {
                    // Halo orgánico con distorsión
                    self.render_kandinsky_halo_ring(draw, note.position, ring_radius, halo_color);
                }
                _ => {
                    // Halo circular básico
                    draw.ellipse()
                        .xy(note.position)
                        .w_h(ring_radius, ring_radius)
                        .color(halo_color);
                }
            }
        }
    }
    
    /// **Calcula color artístico para halo**
    fn calculate_artistic_halo_color(&self, note: &VisualNote, ring_factor: f32, base_opacity: f32) -> Rgba {
        let base_color = note.color;
        let time_shift = self.current_time * 0.3;
        
        match self.config.artistic_effects.style {
            ArtisticStyle::Kandinsky => {
                // Colores emocionales cálidos
                let warmth = (time_shift + ring_factor * 0.5).sin() * 0.3;
                rgba(
                    (base_color.red as f32 / 255.0 + warmth).min(1.0),
                    (base_color.green as f32 / 255.0).max(0.2),
                    (base_color.blue as f32 / 255.0 - warmth * 0.5).max(0.1),
                    base_opacity,
                )
            }
            ArtisticStyle::Xenakis => {
                // Colores fríos y matemáticos
                let coldness = (time_shift * 2.0 + ring_factor).cos() * 0.2;
                rgba(
                    (base_color.red as f32 / 255.0 - coldness).max(0.1),
                    (base_color.green as f32 / 255.0 + coldness * 0.5).min(1.0),
                    (base_color.blue as f32 / 255.0 + coldness).min(1.0),
                    base_opacity,
                )
            }
            _ => {
                // Color base con ligera variación
                rgba(
                    base_color.red as f32 / 255.0,
                    base_color.green as f32 / 255.0,
                    base_color.blue as f32 / 255.0,
                    base_opacity,
                )
            }
        }
    }
    
    /// **Renderiza anillo de halo estilo Xenakis (estocástico)**
    fn render_xenakis_halo_ring(&self, draw: &Draw, center: Vec2, radius: f32, color: Rgba) {
        let complexity = self.config.artistic_effects.xenakis_complexity;
        let point_count = (20.0 * complexity) as u32;
        
        for i in 0..point_count {
            let angle = (i as f32 / point_count as f32) * 2.0 * PI;
            
            // Dispersión estocástica
            let noise_factor = (angle * 5.0 + self.current_time * 2.0).sin() * complexity;
            let scattered_radius = radius * (1.0 + noise_factor * 0.3);
            
            let point_pos = vec2(
                center.x + scattered_radius * angle.cos(),
                center.y + scattered_radius * angle.sin(),
            );
            
            draw.ellipse()
                .xy(point_pos)
                .w_h(2.0, 2.0)
                .color(color);
        }
    }
    
    /// **Renderiza anillo de halo estilo Kandinsky (orgánico)**
    fn render_kandinsky_halo_ring(&self, draw: &Draw, center: Vec2, radius: f32, color: Rgba) {
        let distortion = self.config.artistic_effects.kandinsky_distortion;
        let organic_points = self.generate_organic_blob(center, radius, distortion);
        
        // Renderizar como línea orgánica
        for i in 0..organic_points.len() {
            let current = organic_points[i];
            let next = organic_points[(i + 1) % organic_points.len()];
            
            draw.line()
                .start(current)
                .end(next)
                .weight(1.5)
                .color(color);
        }
    }
    
    /// **Renderizado básico con efectos artísticos**
    fn render_basic_artistic(&self, draw: &Draw, note: &VisualNote, size: f32, opacity: f32) {
        let artistic_color = self.calculate_artistic_color(note, opacity);
        let artistic_size = self.calculate_artistic_size(note, size);
        
        // Animación de pulso si está habilitada
        let pulse_size = if self.config.artistic_effects.pulse_animation {
            let pulse_factor = (self.current_time * 3.0 + note.frequency * 0.01).sin() * 0.5 + 0.5;
            artistic_size * (1.0 + pulse_factor * self.config.artistic_effects.pulse_intensity * 0.3)
        } else {
            artistic_size
        };
        
        let drawing = draw
            .translate(vec3(note.position.x, note.position.y, 0.0))
            .rotate(note.rotation)
            .scale(note.scale_pulse);
        
        // Renderizar según estilo artístico
        match self.config.artistic_effects.style {
            ArtisticStyle::Xenakis => {
                self.render_xenakis_shape(drawing, note, pulse_size, artistic_color);
            }
            ArtisticStyle::Kandinsky => {
                self.render_kandinsky_shape(drawing, note, pulse_size, artistic_color);
            }
            _ => {
                // Renderizado básico mejorado
                self.render_enhanced_basic_shape(drawing, note, pulse_size, artistic_color);
            }
        }
    }
    
    /// **Calcula color artístico dinámico**
    fn calculate_artistic_color(&self, note: &VisualNote, base_opacity: f32) -> Rgba {
        let base_color = note.color;
        let time_factor = self.current_time * 0.5;
        let freq_factor = note.frequency * 0.001;
        
        match self.config.artistic_effects.style {
            ArtisticStyle::Kandinsky => {
                // Colores emocionales y cálidos
                let emotional_shift = (time_factor + freq_factor).sin() * 0.2;
                rgba(
                    (base_color.red as f32 / 255.0 + emotional_shift).min(1.0),
                    (base_color.green as f32 / 255.0 + emotional_shift * 0.5).min(1.0),
                    (base_color.blue as f32 / 255.0 - emotional_shift * 0.3).max(0.0),
                    base_opacity,
                )
            }
            ArtisticStyle::Xenakis => {
                // Colores matemáticos y fríos
                let mathematical_shift = (time_factor * 2.0 + freq_factor * 3.0).cos() * 0.15;
                rgba(
                    (base_color.red as f32 / 255.0 - mathematical_shift).max(0.0),
                    (base_color.green as f32 / 255.0 + mathematical_shift).min(1.0),
                    (base_color.blue as f32 / 255.0 + mathematical_shift * 0.8).min(1.0),
                    base_opacity,
                )
            }
            _ => {
                rgba(
                    base_color.red as f32 / 255.0,
                    base_color.green as f32 / 255.0,
                    base_color.blue as f32 / 255.0,
                    base_opacity,
                )
            }
        }
    }
    
    /// **Calcula tamaño artístico dinámico**
    fn calculate_artistic_size(&self, note: &VisualNote, base_size: f32) -> f32 {
        match self.config.artistic_effects.style {
            ArtisticStyle::Kandinsky => {
                // Tamaño orgánico variable
                let organic_variation = (self.current_time * 1.5 + note.frequency * 0.01).sin() * 0.3;
                base_size * (1.0 + organic_variation * self.config.artistic_effects.kandinsky_distortion)
            }
            ArtisticStyle::Xenakis => {
                // Tamaño estocástico
                let stochastic_variation = (self.current_time * 3.0 + note.frequency * 0.02).sin() * 0.4;
                base_size * (1.0 + stochastic_variation * self.config.artistic_effects.xenakis_complexity)
            }
            _ => base_size,
        }
    }
    
    /// **Renderiza forma estilo Xenakis (estocástica)**
    fn render_xenakis_shape(&self, drawing: nannou::draw::Draw, note: &VisualNote, size: f32, color: Rgba) {
        let complexity = self.config.artistic_effects.xenakis_complexity;
        let point_count = (8.0 + complexity * 12.0) as u32;
        
        // Generar nube de puntos estocástica
        for i in 0..point_count {
            let angle = (i as f32 / point_count as f32) * 2.0 * PI;
            let noise = (angle * 7.0 + self.current_time * 2.0 + note.frequency * 0.001).sin();
            let scattered_radius = size * 0.5 * (1.0 + noise * complexity);
            
            let point_pos = vec2(
                scattered_radius * angle.cos(),
                scattered_radius * angle.sin(),
            );
            
            drawing.ellipse()
                .xy(point_pos)
                .w_h(2.0 + complexity * 3.0, 2.0 + complexity * 3.0)
                .color(color);
        }
    }
    
    /// **Renderiza forma estilo Kandinsky (orgánica)**
    fn render_kandinsky_shape(&self, drawing: nannou::draw::Draw, note: &VisualNote, size: f32, color: Rgba) {
        let distortion = self.config.artistic_effects.kandinsky_distortion;
        
        // Crear forma orgánica compleja
        let organic_points = self.generate_complex_organic_shape(size, distortion, note.frequency);
        
        drawing.polygon()
            .points(organic_points)
            .color(color);
    }
    
    /// **Genera forma orgánica compleja para Kandinsky**
    fn generate_complex_organic_shape(&self, size: f32, distortion: f32, frequency: f32) -> Vec<Vec2> {
        let point_count = 12;
        let mut points = Vec::new();
        
        for i in 0..point_count {
            let angle = (i as f32 / point_count as f32) * 2.0 * PI;
            
            // Múltiples capas de distorsión orgánica
            let time_factor = self.current_time * 0.8;
            let freq_factor = frequency * 0.001;
            
            let primary_distortion = (angle * 3.0 + time_factor).sin() * distortion;
            let secondary_distortion = (angle * 7.0 + time_factor * 1.5 + freq_factor).cos() * distortion * 0.3;
            let tertiary_distortion = (angle * 11.0 + time_factor * 0.7).sin() * distortion * 0.15;
            
            let total_distortion = primary_distortion + secondary_distortion + tertiary_distortion;
            let organic_radius = size * 0.5 * (1.0 + total_distortion);
            
            points.push(vec2(
                organic_radius * angle.cos(),
                organic_radius * angle.sin(),
            ));
        }
        
        points
    }
    
    /// **Renderiza forma básica mejorada**
    fn render_enhanced_basic_shape(&self, drawing: nannou::draw::Draw, note: &VisualNote, size: f32, color: Rgba) {
        match &note.visual_style {
            VisualStyle::Circle => {
                drawing.ellipse()
                    .w_h(size, size)
                    .color(color);
            },
            VisualStyle::Polygon(sides) => {
                let points: Vec<Vec2> = (0..*sides)
                    .map(|i| {
                        let angle = (i as f32 / *sides as f32) * 2.0 * PI;
                        vec2(
                            (size * 0.5) * angle.cos(),
                            (size * 0.5) * angle.sin(),
                        )
                    })
                    .collect();
                
                drawing.polygon()
                    .points(points)
                    .color(color);
            },
            _ => {
                drawing.ellipse()
                    .w_h(size, size)
                    .color(color);
            }
        }
    }
    
    /// **Renderizado con efectos artísticos avanzados**
    fn render_with_artistic_effects(&self, draw: &Draw, note: &VisualNote, size: f32, opacity: f32, quality: f32) {
        let effects = &self.config.artistic_effects;
        
        // 1. Efecto de resplandor (glow) si está habilitado
        if effects.halo_effect {
            self.render_artistic_glow(draw, note, size, opacity, quality);
        }
        
        // 2. Renderizado principal con distorsión geométrica
        let distorted_size = if effects.kandinsky_distortion > 0.0 {
            self.apply_geometry_distortion(size, note)
        } else {
            size
        };
        
        // 3. Aplicar modo de mezcla artístico
        let blended_color = self.apply_artistic_blend_mode(note, opacity);
        
        // 4. Renderizar forma principal
        self.render_main_artistic_shape(draw, note, distorted_size, blended_color, quality);
        
        // 5. Efectos de partículas si están habilitados
        if effects.trail_effect {
            self.render_particle_effects(draw, note, size, opacity);
        }
    }
    
    /// **Renderiza efecto de resplandor artístico**
    fn render_artistic_glow(&self, draw: &Draw, note: &VisualNote, size: f32, opacity: f32, quality: f32) {
        let glow_layers = (3.0 * quality) as u32;
        
        for i in 0..glow_layers {
            let layer_factor = (i + 1) as f32;
            let glow_size = size * (1.0 + layer_factor * 0.6);
            let glow_opacity = (opacity * 0.08 / layer_factor).max(0.005);
            
            // Color del resplandor con tinte artístico
            let glow_color = self.calculate_glow_color(note, glow_opacity, layer_factor);
            
            // Renderizar capa de resplandor
            draw.ellipse()
                .xy(note.position)
                .w_h(glow_size, glow_size)
                .color(glow_color);
        }
    }
    
    /// **Calcula color del resplandor con efectos artísticos**
    fn calculate_glow_color(&self, note: &VisualNote, base_opacity: f32, layer_factor: f32) -> Rgba {
        let base_color = note.color;
        let time_shift = self.current_time * 0.4 + layer_factor * 0.2;
        
        match self.config.artistic_effects.style {
            ArtisticStyle::Kandinsky => {
                // Resplandor cálido y emocional
                let warmth = (time_shift + note.frequency * 0.001).sin() * 0.4;
                rgba(
                    (base_color.red as f32 / 255.0 + warmth * 0.3).min(1.0),
                    (base_color.green as f32 / 255.0 + warmth * 0.1).min(1.0),
                    (base_color.blue as f32 / 255.0 - warmth * 0.2).max(0.0),
                    base_opacity,
                )
            }
            ArtisticStyle::Xenakis => {
                // Resplandor frío y matemático
                let intensity = (time_shift * 2.0 + note.frequency * 0.002).cos() * 0.3;
                rgba(
                    (base_color.red as f32 / 255.0 + intensity * 0.1).min(1.0),
                    (base_color.green as f32 / 255.0 + intensity * 0.4).min(1.0),
                    (base_color.blue as f32 / 255.0 + intensity * 0.6).min(1.0),
                    base_opacity,
                )
            }
            _ => {
                // Resplandor neutro
                rgba(
                    base_color.red as f32 / 255.0,
                    base_color.green as f32 / 255.0,
                    base_color.blue as f32 / 255.0,
                    base_opacity * 0.8,
                )
            }
        }
    }
    
    /// **Aplica distorsión geométrica artística**
    fn apply_geometry_distortion(&self, base_size: f32, note: &VisualNote) -> f32 {
        let distortion_amount = self.config.artistic_effects.kandinsky_distortion;
        let time_factor = self.current_time * 1.2;
        let freq_factor = note.frequency * 0.001;
        
        match self.config.artistic_effects.style {
            ArtisticStyle::Kandinsky => {
                // Distorsión orgánica suave
                let organic_wave = (time_factor + freq_factor * 5.0).sin() * 0.3;
                base_size * (1.0 + organic_wave * distortion_amount)
            }
            ArtisticStyle::Xenakis => {
                // Distorsión estocástica abrupta
                let stochastic_noise = (time_factor * 3.0 + freq_factor * 7.0).sin() * 
                                      (time_factor * 1.7 + freq_factor * 11.0).cos();
                base_size * (1.0 + stochastic_noise * distortion_amount * 0.5)
            }
            _ => base_size,
        }
    }
    
    /// **Aplica modo de mezcla artístico**
    fn apply_artistic_blend_mode(&self, note: &VisualNote, base_opacity: f32) -> Rgba {
        let base_color = note.color;
        
        match BlendMode::Normal {
            BlendMode::Additive => {
                // Mezcla aditiva: colores más brillantes
                rgba(
                    (base_color.red as f32 / 255.0 * 1.3).min(1.0),
                    (base_color.green as f32 / 255.0 * 1.3).min(1.0),
                    (base_color.blue as f32 / 255.0 * 1.3).min(1.0),
                    base_opacity * 0.8,
                )
            }
            BlendMode::Multiply => {
                // Mezcla multiplicativa: colores más oscuros
                rgba(
                    base_color.red as f32 / 255.0 * 0.7,
                    base_color.green as f32 / 255.0 * 0.7,
                    base_color.blue as f32 / 255.0 * 0.7,
                    base_opacity * 1.2,
                )
            }
            BlendMode::Screen => {
                // Mezcla de pantalla: colores más claros
                rgba(
                    1.0 - (1.0 - base_color.red as f32 / 255.0) * 0.6,
                    1.0 - (1.0 - base_color.green as f32 / 255.0) * 0.6,
                    1.0 - (1.0 - base_color.blue as f32 / 255.0) * 0.6,
                    base_opacity,
                )
            }
            BlendMode::Overlay => {
                // Mezcla overlay: contraste aumentado
                let time_factor = self.current_time * 0.5;
                let contrast = (time_factor + note.frequency * 0.001).sin() * 0.3 + 1.0;
                rgba(
                    (base_color.red as f32 / 255.0 * contrast).min(1.0),
                    (base_color.green as f32 / 255.0 * contrast).min(1.0),
                    (base_color.blue as f32 / 255.0 * contrast).min(1.0),
                    base_opacity,
                )
            }
            BlendMode::Normal => {
                rgba(
                    base_color.red as f32 / 255.0,
                    base_color.green as f32 / 255.0,
                    base_color.blue as f32 / 255.0,
                    base_opacity,
                )
            }
        }
    }
    
    /// **Renderiza forma principal artística**
    fn render_main_artistic_shape(&self, draw: &Draw, note: &VisualNote, size: f32, color: Rgba, quality: f32) {
        let drawing = draw
            .translate(vec3(note.position.x, note.position.y, 0.0))
            .rotate(note.rotation)
            .scale(note.scale_pulse);
        
        // Aplicar efectos según estilo
        match self.config.artistic_effects.style {
            ArtisticStyle::Xenakis => {
                self.render_xenakis_advanced_shape(drawing, note, size, color, quality);
            }
            ArtisticStyle::Kandinsky => {
                self.render_kandinsky_advanced_shape(drawing, note, size, color, quality);
            }
            ArtisticStyle::Minimalist => {
                self.render_minimalist_shape(drawing, note, size, color);
            }
            ArtisticStyle::Expressionist => {
                self.render_expressionist_shape(drawing, note, size, color);
            }
            ArtisticStyle::Hybrid => {
                self.render_hybrid_shape(drawing, note, size, color, quality);
            }
        }
    }
    
    /// **Renderiza forma avanzada estilo Xenakis**
    fn render_xenakis_advanced_shape(&self, drawing: nannou::draw::Draw, _note: &VisualNote, size: f32, color: Rgba, quality: f32) {
        let complexity = self.config.artistic_effects.xenakis_complexity * quality;
        
        // Crear constelación de puntos estocásticos
        let point_count = (15.0 + complexity * 25.0) as u32;
        let cluster_density = 0.8 + complexity * 0.2;
        
        for i in 0..point_count {
            // Distribución estocástica no uniforme
            let t = i as f32 / point_count as f32;
            let angle = t * 2.0 * PI + (t * 17.0 + self.current_time).sin() * complexity;
            
            // Densidad variable (clusters)
            let density_wave = (t * 5.0 + self.current_time * 0.7).sin() * 0.5 + 0.5;
            let radius_factor = density_wave * cluster_density + (1.0 - cluster_density);
            let radius = size * 0.5 * radius_factor;
            
            // Posición con ruido estocástico
            let noise_x = (angle * 7.0 + self.current_time * 1.3).sin() * complexity * size * 0.1;
            let noise_y = (angle * 11.0 + self.current_time * 0.9).cos() * complexity * size * 0.1;
            
            let point_pos = vec2(
                radius * angle.cos() + noise_x,
                radius * angle.sin() + noise_y,
            );
            
            // Tamaño variable de puntos
            let point_size = (2.0 + complexity * 4.0) * (0.5 + density_wave * 0.5);
            
            drawing.ellipse()
                .xy(point_pos)
                .w_h(point_size, point_size)
                .color(color);
        }
    }
    
    /// **Renderiza forma avanzada estilo Kandinsky**
    fn render_kandinsky_advanced_shape(&self, drawing: nannou::draw::Draw, note: &VisualNote, size: f32, color: Rgba, quality: f32) {
        let distortion = self.config.artistic_effects.kandinsky_distortion * quality;
        
        // Crear múltiples capas orgánicas
        let layer_count = (2.0 + quality * 2.0) as u32;
        
        for layer in 0..layer_count {
            let layer_factor = (layer + 1) as f32;
            let layer_size = size * (1.0 - layer_factor * 0.15);
            let layer_distortion = distortion * (1.0 + layer_factor * 0.3);
            
            // Generar forma orgánica para esta capa
            let organic_points = self.generate_kandinsky_layer(layer_size, layer_distortion, note.frequency, layer_factor);
            
            // Color con variación por capa
            let layer_color = self.calculate_kandinsky_layer_color(color, layer_factor);
            
            drawing.polygon()
                .points(organic_points)
                .color(layer_color);
        }
    }
    
    /// **Genera capa orgánica para Kandinsky**
    fn generate_kandinsky_layer(&self, size: f32, distortion: f32, frequency: f32, layer_factor: f32) -> Vec<Vec2> {
        let point_count = 16;
        let mut points = Vec::new();
        
        for i in 0..point_count {
            let angle = (i as f32 / point_count as f32) * 2.0 * PI;
            let time_factor = self.current_time * (0.5 + layer_factor * 0.3);
            let freq_factor = frequency * 0.001 * layer_factor;
            
            // Múltiples ondas orgánicas
            let wave1 = (angle * 3.0 + time_factor).sin() * distortion;
            let wave2 = (angle * 7.0 + time_factor * 1.4 + freq_factor).cos() * distortion * 0.4;
            let wave3 = (angle * 13.0 + time_factor * 0.8).sin() * distortion * 0.2;
            
            let total_distortion = wave1 + wave2 + wave3;
            let organic_radius = size * 0.5 * (1.0 + total_distortion);
            
            points.push(vec2(
                organic_radius * angle.cos(),
                organic_radius * angle.sin(),
            ));
        }
        
        points
    }
    
    /// **Calcula color de capa para Kandinsky**
    fn calculate_kandinsky_layer_color(&self, base_color: Rgba, layer_factor: f32) -> Rgba {
        let transparency = 0.7 / layer_factor;
        let emotional_shift = (self.current_time * 0.6 + layer_factor).sin() * 0.1;
        
        rgba(
            (base_color.red + emotional_shift).min(1.0).max(0.0),
            (base_color.green + emotional_shift * 0.5).min(1.0).max(0.0),
            (base_color.blue - emotional_shift * 0.3).min(1.0).max(0.0),
            base_color.alpha * transparency,
        )
    }
    
    /// **Renderiza forma minimalista**
    fn render_minimalist_shape(&self, drawing: nannou::draw::Draw, note: &VisualNote, size: f32, color: Rgba) {
        // Formas geométricas puras y simples
        match &note.visual_style {
            VisualStyle::Circle => {
                drawing.ellipse()
                    .w_h(size, size)
                    .color(color)
                    .stroke(rgba(1.0, 1.0, 1.0, 0.3))
                    .stroke_weight(1.0);
            }
            VisualStyle::Polygon(sides) => {
                let points: Vec<Vec2> = (0..*sides)
                    .map(|i| {
                        let angle = (i as f32 / *sides as f32) * 2.0 * PI;
                        vec2(
                            (size * 0.5) * angle.cos(),
                            (size * 0.5) * angle.sin(),
                        )
                    })
                    .collect();
                
                drawing.polygon()
                    .points(points)
                    .color(color);
            }
            _ => {
                drawing.rect()
                    .w_h(size * 0.8, size * 0.8)
                    .color(color)
                    .stroke(rgba(1.0, 1.0, 1.0, 0.3))
                    .stroke_weight(1.0);
            }
        }
    }
    
    /// **Renderiza forma expresionista**
    fn render_expressionist_shape(&self, drawing: nannou::draw::Draw, note: &VisualNote, size: f32, color: Rgba) {
        // Formas agresivas y expresivas
        let intensity = note.amplitude * 2.0;
        let distortion = (self.current_time * 2.0 + note.frequency * 0.01).sin() * intensity * 0.3;
        
        // Crear forma expresiva distorsionada
        let point_count = 8;
        let mut points = Vec::new();
        
        for i in 0..point_count {
            let angle = (i as f32 / point_count as f32) * 2.0 * PI;
            let aggressive_distortion = (angle * 3.0 + distortion).sin() * intensity * size * 0.2;
            let radius = size * 0.5 + aggressive_distortion;
            
            points.push(vec2(
                radius * angle.cos(),
                radius * angle.sin(),
            ));
        }
        
        drawing.polygon()
            .points(points)
            .color(color);
        
        // Añadir líneas expresivas
        if intensity > 0.5 {
            for i in 0..4 {
                let line_angle = (i as f32 / 4.0) * 2.0 * PI + distortion;
                let line_length = size * intensity;
                
                drawing.line()
                    .start(vec2(0.0, 0.0))
                    .end(vec2(
                        line_length * line_angle.cos(),
                        line_length * line_angle.sin(),
                    ))
                    .weight(2.0 * intensity)
                    .color(rgba(color.red, color.green, color.blue, color.alpha * 0.6));
            }
        }
    }
    
    /// **Renderiza forma híbrida (combina estilos)**
    fn render_hybrid_shape(&self, drawing: nannou::draw::Draw, note: &VisualNote, size: f32, color: Rgba, quality: f32) {
        let drawing_clone = drawing.clone();
        let time_factor = self.current_time * 0.3;
        
        // Alternar entre estilos basado en tiempo y frecuencia
        let style_mix = (time_factor + note.frequency * 0.001).sin() * 0.5 + 0.5;
        
        if style_mix < 0.3 {
            // Predominio Xenakis
            self.render_xenakis_advanced_shape(drawing, note, size, color, quality * 0.7);
        } else if style_mix < 0.7 {
            // Predominio Kandinsky
            self.render_kandinsky_advanced_shape(drawing, note, size, color, quality * 0.7);
        } else {
            // Mezcla expresionista-minimalista
            let hybrid_color = rgba(color.red, color.green, color.blue, color.alpha * 0.7);
            self.render_expressionist_shape(drawing_clone, note, size * 0.8, hybrid_color);
            self.render_minimalist_shape(drawing, note, size * 1.2,
                rgba(color.red, color.green, color.blue, color.alpha * 0.3));
        }
    }
    
    /// **Renderiza efectos de partículas**
    fn render_particle_effects(&self, draw: &Draw, note: &VisualNote, size: f32, opacity: f32) {
        if note.amplitude < 0.3 {
            return; // Solo para notas con suficiente intensidad
        }
        
        let particle_count = (note.amplitude * 10.0) as u32;
        let particle_spread = size * 2.0;
        
        for i in 0..particle_count {
            let t = i as f32 / particle_count as f32;
            let angle = t * 2.0 * PI + self.current_time * 2.0;
            
            // Posición de partícula con movimiento
            let particle_radius = particle_spread * (0.5 + t * 0.5);
            let drift = (self.current_time * 0.8 + t * 5.0).sin() * particle_spread * 0.2;
            
            let particle_pos = vec2(
                note.position.x + particle_radius * angle.cos() + drift,
                note.position.y + particle_radius * angle.sin() + drift,
            );
            
            // Color y tamaño de partícula
            let particle_opacity = opacity * (0.1 + t * 0.4) * note.amplitude;
            let particle_size = (1.0 + t * 3.0) * note.amplitude;
            
            let particle_color = rgba(
                note.color.red as f32 / 255.0,
                note.color.green as f32 / 255.0,
                note.color.blue as f32 / 255.0,
                particle_opacity,
            );
            
            draw.ellipse()
                .xy(particle_pos)
                .w_h(particle_size, particle_size)
                .color(particle_color);
        }
    }

    /// **Obtiene factor de calidad para optimización**
    fn get_quality_factor(&self) -> f32 {
        match self.config.quality {
            RenderQuality::Low => 0.5,
            RenderQuality::Medium => 0.75,
            RenderQuality::High => 1.0,
            RenderQuality::Ultra => 1.5,
        }
    }
    
    /// **Actualiza configuración del shader manager**
    pub fn update_config(&mut self, config: ShaderConfig) {
        self.config = config;
    }
    
    /// **Cambia la paleta de colores del mapeador profesional**
    pub fn set_color_palette(&mut self, palette: ColorPalette) {
        self.pro_mapper.set_color_palette(palette);
    }
    
    /// **Cambia el estilo artístico en tiempo real**
    pub fn set_artistic_style(&mut self, style: ArtisticStyle) {
        self.config.artistic_effects.style = style;
        println!("🎨 Estilo artístico cambiado a: {:?}", style);
    }
    
    /// **Activa/desactiva efectos artísticos específicos**
    pub fn toggle_artistic_effect(&mut self, effect: &str) {
        match effect.to_lowercase().as_str() {
            "pulse" => {
                self.config.artistic_effects.pulse_animation = !self.config.artistic_effects.pulse_animation;
                println!("💓 Animación de pulso: {}", if self.config.artistic_effects.pulse_animation { "ON" } else { "OFF" });
            }
            "halo" => {
                self.config.artistic_effects.halo_effect = !self.config.artistic_effects.halo_effect;
                println!("⭕ Efecto de halo: {}", if self.config.artistic_effects.halo_effect { "ON" } else { "OFF" });
            }
            "trail" => {
                self.config.artistic_effects.trail_effect = !self.config.artistic_effects.trail_effect;
                println!("🌟 Efecto de rastro: {}", if self.config.artistic_effects.trail_effect { "ON" } else { "OFF" });
            }
            "blend" => {
                self.config.artistic_effects.artistic_blending = !self.config.artistic_effects.artistic_blending;
                println!("🎭 Blend artístico: {}", if self.config.artistic_effects.artistic_blending { "ON" } else { "OFF" });
            }
            _ => println!("❌ Efecto desconocido: {}", effect),
        }
    }
    
    /// **Ajusta parámetros artísticos en tiempo real**
    pub fn adjust_artistic_parameter(&mut self, param: &str, delta: f32) {
        match param.to_lowercase().as_str() {
            "pulse_intensity" => {
                self.config.artistic_effects.pulse_intensity = (self.config.artistic_effects.pulse_intensity + delta).max(0.0).min(3.0);
                println!("💪 Intensidad de pulso: {:.2}", self.config.artistic_effects.pulse_intensity);
            }
            "kandinsky_distortion" => {
                self.config.artistic_effects.kandinsky_distortion = (self.config.artistic_effects.kandinsky_distortion + delta).max(0.0).min(1.0);
                println!("🌊 Distorsión Kandinsky: {:.2}", self.config.artistic_effects.kandinsky_distortion);
            }
            "xenakis_complexity" => {
                self.config.artistic_effects.xenakis_complexity = (self.config.artistic_effects.xenakis_complexity + delta).max(0.0).min(1.0);
                println!("🔢 Complejidad Xenakis: {:.2}", self.config.artistic_effects.xenakis_complexity);
            }
            "trail_duration" => {
                self.config.artistic_effects.trail_duration = (self.config.artistic_effects.trail_duration + delta).max(0.5).min(10.0);
                println!("⏱️ Duración de rastro: {:.2}s", self.config.artistic_effects.trail_duration);
            }
            _ => println!("❌ Parámetro desconocido: {}", param),
        }
    }
    
    /// **Método público para añadir rastros (wrapper para &mut self)**
    pub fn add_note_trail(&mut self, note: &VisualNote) {
        self.add_trail_point(note.note_id, note.position, note.color, note.size, note.amplitude);
        self.update_position_history(note.note_id, note.position);
    }
    
    /// **Limpia rastros de una nota específica**
    pub fn clear_note_trails(&mut self, note_id: u64) {
        self.trail_buffer.remove(&note_id);
        self.position_history.remove(&note_id);
    }
    
    /// **Limpia todos los rastros**
    pub fn clear_all_trails(&mut self) {
        self.trail_buffer.clear();
        self.position_history.clear();
    }

    /// **Obtiene estadísticas del manager**
    pub fn get_stats(&self) -> ShaderStats {
        ShaderStats {
            cache_size: self.shape_cache.len(),
            current_time: self.current_time,
            window_size: self.window_size,
            quality: self.config.quality,
            artistic_style: self.config.artistic_effects.style,
            active_trails: self.trail_buffer.len(),
            total_trail_points: self.trail_buffer.values().map(|v| v.len()).sum(),
            effects_enabled: ArtisticEffectsStatus {
                pulse_animation: self.config.artistic_effects.pulse_animation,
                halo_effect: self.config.artistic_effects.halo_effect,
                trail_effect: self.config.artistic_effects.trail_effect,
                artistic_blending: self.config.artistic_effects.artistic_blending,
            },
        }
    }
    
    /// **Convierte amplitud a nivel de energía visual**
    fn amplitude_to_energy_level(&self, amplitude: f32) -> crate::visual::audio_visual_mapping::EnergyLevel {
        use crate::visual::audio_visual_mapping::EnergyLevel;
        
        if amplitude < 0.2 {
            EnergyLevel::Low
        } else if amplitude < 0.6 {
            EnergyLevel::Medium
        } else if amplitude < 0.8 {
            EnergyLevel::High
        } else {
            EnergyLevel::Extreme
        }
    }
    
    /// **Genera un ID único para la nota**
    fn generate_note_id(&self, freq: f32, amplitude: f32, birth_time: f32) -> u64 {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        (freq as u32).hash(&mut hasher);
        ((amplitude * 1000.0) as u32).hash(&mut hasher);
        ((birth_time * 1000.0) as u32).hash(&mut hasher);
        hasher.finish()
    }
}

/// Estadísticas del ShaderManager
#[derive(Debug, Clone)]
pub struct ShaderStats {
    pub cache_size: usize,
    pub current_time: f32,
    pub window_size: Vec2,
    pub quality: RenderQuality,
    pub artistic_style: ArtisticStyle,
    pub active_trails: usize,
    pub total_trail_points: usize,
    pub effects_enabled: ArtisticEffectsStatus,
}

/// Estado de efectos artísticos
#[derive(Debug, Clone)]
pub struct ArtisticEffectsStatus {
    pub pulse_animation: bool,
    pub halo_effect: bool,
    pub trail_effect: bool,
    pub artistic_blending: bool,
}

/// Función de utilidad para crear EventKind desde string
pub fn string_to_event_kind(event_type: &str) -> EventKind {
    match event_type.to_lowercase().as_str() {
        "note" | "nota" => EventKind::Note,
        "chord" | "acorde" => EventKind::Chord,
        "percussion" | "perc" | "drum" => EventKind::Percussion,
        "sustained" | "pad" | "drone" => EventKind::Sustained,
        "transient" | "attack" => EventKind::Transient,
        "noise" | "ruido" => EventKind::Noise,
        "control" | "cc" => EventKind::Control,
        other => EventKind::Custom(other.to_string()),
    }
}

/// Efectos artísticos inspirados en compositores visuales como Xenakis y Kandinsky
#[derive(Debug, Clone)]
pub struct ArtisticEffects {
    /// Activar animación de pulso basada en amplitud
    pub pulse_animation: bool,
    /// Intensidad del pulso (0.0 - 2.0)
    pub pulse_intensity: f32,
    /// Activar efecto de halo concéntrico
    pub halo_effect: bool,
    /// Número de anillos en el halo
    pub halo_rings: u32,
    /// Activar rastros que se desvanecen
    pub trail_effect: bool,
    /// Duración del rastro en segundos
    pub trail_duration: f32,
    /// Activar blend modes artísticos
    pub artistic_blending: bool,
    /// Estilo artístico principal
    pub style: ArtisticStyle,
    /// Intensidad de distorsión Kandinsky (geometría orgánica)
    pub kandinsky_distortion: f32,
    /// Factor de complejidad Xenakis (dispersión estocástica)
    pub xenakis_complexity: f32,
}

impl Default for ArtisticEffects {
    fn default() -> Self {
        Self {
            pulse_animation: true,
            pulse_intensity: 1.2,
            halo_effect: true,
            halo_rings: 3,
            trail_effect: true,
            trail_duration: 2.0,
            artistic_blending: true,
            style: ArtisticStyle::Hybrid,
            kandinsky_distortion: 0.3,
            xenakis_complexity: 0.5,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_shader_manager_creation() {
        let manager = ShaderManager::default();
        let stats = manager.get_stats();
        
        assert_eq!(stats.cache_size, 0);
        assert_eq!(stats.quality, RenderQuality::Medium);
        assert_eq!(stats.artistic_style, ArtisticStyle::Hybrid);
        assert_eq!(stats.active_trails, 0);
        assert!(stats.effects_enabled.pulse_animation);
        assert!(stats.effects_enabled.halo_effect);
        assert!(stats.effects_enabled.trail_effect);
    }
    
    #[test]
    fn test_professional_visual_note_creation() {
        let mut manager = ShaderManager::default();
        manager.update_time(1.0);
        
        let note = manager.create_professional_visual_note(
            440.0,      // A4
            0.8,        // amplitud alta
            2.0,        // duración
            EventKind::Note,
            0.0,        // birth_time
            "piano",
        );
        
        assert!(note.frequency == 440.0);
        assert!(note.amplitude == 0.8);
        assert!(note.opacity > 0.0);
        assert!(note.note_id > 0);
    }
    
    #[test]
    fn test_artistic_effects_control() {
        let mut manager = ShaderManager::default();
        
        // Test cambio de estilo
        manager.set_artistic_style(ArtisticStyle::Xenakis);
        assert_eq!(manager.get_stats().artistic_style, ArtisticStyle::Xenakis);
        
        // Test toggle de efectos
        manager.toggle_artistic_effect("pulse");
        assert!(!manager.get_stats().effects_enabled.pulse_animation);
        
        manager.toggle_artistic_effect("halo");
        assert!(!manager.get_stats().effects_enabled.halo_effect);
        
        // Test ajuste de parámetros
        manager.adjust_artistic_parameter("pulse_intensity", 0.5);
        assert_eq!(manager.config.artistic_effects.pulse_intensity, 1.7); // 1.2 + 0.5
        
        manager.adjust_artistic_parameter("kandinsky_distortion", 0.2);
        assert_eq!(manager.config.artistic_effects.kandinsky_distortion, 0.5); // 0.3 + 0.2
    }
    
    #[test]
    fn test_trail_system() {
        let mut manager = ShaderManager::default();
        
        // Crear nota de prueba
        let note = manager.create_professional_visual_note(
            440.0, 0.8, 2.0, EventKind::Note, 0.0, "piano"
        );
        
        // Añadir rastro
        manager.add_note_trail(&note);
        
        let stats = manager.get_stats();
        assert_eq!(stats.active_trails, 1);
        assert_eq!(stats.total_trail_points, 1);
        
        // Limpiar rastros
        manager.clear_note_trails(note.note_id);
        assert_eq!(manager.get_stats().active_trails, 0);
    }
    
    #[test]
    fn test_event_kind_conversion() {
        assert_eq!(string_to_event_kind("note"), EventKind::Note);
        assert_eq!(string_to_event_kind("CHORD"), EventKind::Chord);
        assert_eq!(string_to_event_kind("percussion"), EventKind::Percussion);
        
        match string_to_event_kind("custom_event") {
            EventKind::Custom(name) => assert_eq!(name, "custom_event"),
            _ => panic!("Should be Custom variant"),
        }
    }
}
