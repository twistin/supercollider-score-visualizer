// ⚡ Sistema de batching para optimización de renderizado
// Agrupa elementos visuales similares para reducir draw calls

use nannou::prelude::*;
use crate::musical_events::{MusicalEvent, VisualParams};

/// Instancia de un punto para batching
#[derive(Debug, Clone)]
pub struct PointInstance {
    pub position: Vec2,
    pub radius: f32,
    pub color: Srgba<f32>,
    pub intensity: f32,
    pub timbre: f32,
}

/// Instancia de una línea para batching
#[derive(Debug, Clone)]
pub struct LineInstance {
    pub start: Vec2,
    pub end: Vec2,
    pub weight: f32,
    pub color: Srgba<f32>,
    pub progress: f32,
}

/// Instancia de una partícula para batching
#[derive(Debug, Clone)]
pub struct ParticleInstance {
    pub position: Vec2,
    pub velocity: Vec2,
    pub size: f32,
    pub color: Srgba<f32>,
    pub life: f32,
    pub max_life: f32,
}

/// Instancia de un cluster para batching
#[derive(Debug, Clone)]
pub struct ClusterInstance {
    pub center: Vec2,
    pub radius: f32,
    pub color: Srgba<f32>,
    pub density: f32,
    pub spread: f32,
}

/// Batch de elementos visuales agrupados por tipo
#[derive(Debug, Default)]
pub struct RenderBatch {
    pub points: Vec<PointInstance>,
    pub lines: Vec<LineInstance>,
    pub particles: Vec<ParticleInstance>,
    pub clusters: Vec<ClusterInstance>,
    pub frame_count: u64,
}

impl RenderBatch {
    /// Crea un nuevo batch vacío
    pub fn new() -> Self {
        Self {
            points: Vec::with_capacity(1000),    // Pre-allocar para 1000 puntos
            lines: Vec::with_capacity(500),      // Pre-allocar para 500 líneas
            particles: Vec::with_capacity(2000), // Pre-allocar para 2000 partículas
            clusters: Vec::with_capacity(100),   // Pre-allocar para 100 clusters
            frame_count: 0,
        }
    }

    /// Limpia el batch para el siguiente frame
    pub fn clear(&mut self) {
        self.points.clear();
        self.lines.clear();
        self.particles.clear();
        self.clusters.clear();
        self.frame_count += 1;
    }

    /// Añade un punto al batch
    pub fn add_point(&mut self, position: Vec2, radius: f32, color: Srgba<f32>, 
                    intensity: f32, timbre: f32) {
        self.points.push(PointInstance {
            position,
            radius,
            color,
            intensity,
            timbre,
        });
    }

    /// Añade una línea al batch
    pub fn add_line(&mut self, start: Vec2, end: Vec2, weight: f32, color: Srgba<f32>, 
                   progress: f32) {
        self.lines.push(LineInstance {
            start,
            end,
            weight,
            color,
            progress,
        });
    }

    /// Añade una partícula al batch
    pub fn add_particle(&mut self, position: Vec2, velocity: Vec2, size: f32, 
                       color: Srgba<f32>, life: f32, max_life: f32) {
        self.particles.push(ParticleInstance {
            position,
            velocity,
            size,
            color,
            life,
            max_life,
        });
    }

    /// Añade un cluster al batch
    pub fn add_cluster(&mut self, center: Vec2, radius: f32, color: Srgba<f32>, 
                      density: f32, spread: f32) {
        self.clusters.push(ClusterInstance {
            center,
            radius,
            color,
            density,
            spread,
        });
    }

    /// Retorna estadísticas del batch
    pub fn stats(&self) -> BatchStats {
        BatchStats {
            points: self.points.len(),
            lines: self.lines.len(),
            particles: self.particles.len(),
            clusters: self.clusters.len(),
            total_elements: self.points.len() + self.lines.len() + 
                          self.particles.len() + self.clusters.len(),
            frame_count: self.frame_count,
        }
    }
}

/// Estadísticas del batch para diagnóstico
#[derive(Debug, Clone)]
pub struct BatchStats {
    pub points: usize,
    pub lines: usize,
    pub particles: usize,
    pub clusters: usize,
    pub total_elements: usize,
    pub frame_count: u64,
}

/// Renderizador optimizado con batching
pub struct BatchedRenderer {
    pub time_window: f32,
    pub batch: RenderBatch,
    pub enable_optimizations: bool,
    pub max_elements_per_batch: usize,
    pub particle_system: ParticleSystem,
}

impl BatchedRenderer {
    /// Crea un nuevo renderizador con batching
    pub fn new() -> Self {
        Self {
            time_window: 10.0,
            batch: RenderBatch::new(),
            enable_optimizations: true,
            max_elements_per_batch: 5000,
            particle_system: ParticleSystem::new(),
        }
    }

    /// Procesa eventos y los añade al batch
    pub fn process_events(&mut self, events: &[MusicalEvent], current_time: f32, win: &Rect) {
        // Limpiar el batch del frame anterior
        self.batch.clear();
        
        // Procesar eventos y añadir al batch
        for event in events {
            self.process_event(event, current_time, win);
        }
        
        // Actualizar sistema de partículas
        self.particle_system.update(current_time);
        
        // Añadir partículas al batch
        self.add_particles_to_batch(win);
    }

    /// Procesa un evento individual y lo añade al batch
    fn process_event(&mut self, event: &MusicalEvent, current_time: f32, win: &Rect) {
        let event_time = event.start_time();
        let progress = event.progress(current_time);
        let params = event.visual_params();
        
        match event {
            MusicalEvent::Point { freq, amp, .. } => {
                self.process_point(*freq, *amp, event_time, current_time, &params, win);
            },
            MusicalEvent::Glissando { start_freq, end_freq, amp, duration, .. } => {
                self.process_glissando(*start_freq, *end_freq, *amp, *duration, 
                                     event_time, current_time, progress, &params, win);
            },
            MusicalEvent::Cluster { center_freq, bandwidth, amp, duration, .. } => {
                self.process_cluster(*center_freq, *bandwidth, *amp, *duration, 
                                   event_time, current_time, progress, &params, win);
            },
            MusicalEvent::Noise { center_freq, bandwidth, amp, duration, .. } => {
                self.process_noise(*center_freq, *bandwidth, *amp, *duration, 
                                 event_time, current_time, progress, &params, win);
            },
        }
    }

    /// Procesa un punto y lo añade al batch
    fn process_point(&mut self, freq: f32, amp: f32, event_time: f32, current_time: f32, 
                    params: &VisualParams, win: &Rect) {
        // Mapeo mejorado para puntos
        let time_start = current_time - self.time_window * 0.1;
        let time_end = current_time + self.time_window * 0.9;
        let x = map_range(event_time, time_start, time_end, 
                         win.left() + 60.0, win.right() - 60.0);
        let y = map_range(freq, 20.0, 2000.0, win.bottom() + 40.0, win.top() - 40.0);
        
        let base_radius = map_range(amp, 0.0, 1.0, 2.0, 15.0);
        let radius = base_radius * params.spatial_spread;
        
        let alpha = map_range(amp, 0.0, 1.0, 0.3, 0.9) * params.intensity;
        let color = self.create_color_from_params(srgb(0.0, 0.5, 1.0), params, alpha);
        
        self.batch.add_point(
            pt2(x, y),
            radius,
            color,
            params.intensity,
            params.timbre
        );
        
        // Generar partículas para eventos intensos
        if amp > 0.7 {
            self.particle_system.emit_burst(pt2(x, y), amp * 10.0, color);
        }
    }

    /// Procesa un glissando y lo añade al batch
    fn process_glissando(&mut self, start_freq: f32, end_freq: f32, amp: f32, duration: f32,
                        event_time: f32, current_time: f32, progress: f32, 
                        params: &VisualParams, win: &Rect) {
        // Mapeo mejorado para glissandos
        let time_start = current_time - self.time_window * 0.1;
        let time_end = current_time + self.time_window * 0.9;
        let x1 = map_range(event_time, time_start, time_end, 
                          win.left() + 60.0, win.right() - 60.0);
        let x2 = map_range(event_time + duration, time_start, time_end, 
                          win.left() + 60.0, win.right() - 60.0);
        let y1 = map_range(start_freq, 20.0, 2000.0, win.bottom() + 40.0, win.top() - 40.0);
        let y2 = map_range(end_freq, 20.0, 2000.0, win.bottom() + 40.0, win.top() - 40.0);
        
        let weight = map_range(amp, 0.0, 1.0, 1.0, 8.0) * params.spatial_spread;
        let alpha = map_range(amp, 0.0, 1.0, 0.4, 0.8) * params.intensity;
        let color = self.create_color_from_params(srgb(1.0, 0.5, 0.3), params, alpha);
        
        self.batch.add_line(
            pt2(x1, y1),
            pt2(x2, y2),
            weight,
            color,
            progress
        );
    }

    /// Procesa un cluster y lo añade al batch
    fn process_cluster(&mut self, center_freq: f32, bandwidth: f32, amp: f32, _duration: f32,
                      event_time: f32, current_time: f32, _progress: f32, 
                      params: &VisualParams, win: &Rect) {
        // Mapeo mejorado para clusters
        let time_start = current_time - self.time_window * 0.1;
        let time_end = current_time + self.time_window * 0.9;
        let x = map_range(event_time, time_start, time_end, 
                         win.left() + 60.0, win.right() - 60.0);
        let y = map_range(center_freq, 20.0, 2000.0, win.bottom() + 40.0, win.top() - 40.0);
        
        let radius = map_range(bandwidth, 0.0, 1000.0, 10.0, 100.0) * params.spatial_spread;
        let alpha = map_range(amp, 0.0, 1.0, 0.2, 0.6) * params.intensity;
        let color = self.create_color_from_params(srgb(0.5, 0.0, 0.5), params, alpha);
        
        self.batch.add_cluster(
            pt2(x, y),
            radius,
            color,
            params.intensity,
            params.spatial_spread
        );
    }

    /// Procesa ruido y genera partículas
    fn process_noise(&mut self, center_freq: f32, bandwidth: f32, amp: f32, _duration: f32,
                    event_time: f32, current_time: f32, _progress: f32, 
                    params: &VisualParams, win: &Rect) {
        // Mapeo mejorado para ruido
        let time_start = current_time - self.time_window * 0.1;
        let time_end = current_time + self.time_window * 0.9;
        let x = map_range(event_time, time_start, time_end, 
                         win.left() + 60.0, win.right() - 60.0);
        let y = map_range(center_freq, 20.0, 2000.0, win.bottom() + 40.0, win.top() - 40.0);
        
        let particle_count = (bandwidth / 50.0 * amp * 20.0) as usize;
        let color = self.create_color_from_params(srgb(1.0, 1.0, 0.0), params, 0.4);
        
        // Generar partículas para simular ruido
        for _ in 0..particle_count {
            let offset_x = random_range(-bandwidth / 10.0, bandwidth / 10.0);
            let offset_y = random_range(-bandwidth / 10.0, bandwidth / 10.0);
            let velocity = pt2(random_range(-50.0, 50.0), random_range(-50.0, 50.0));
            
            self.particle_system.emit_particle(
                pt2(x + offset_x, y + offset_y),
                velocity,
                color,
                1.0 + random_range(0.0, 2.0)
            );
        }
    }

    /// Añade partículas del sistema al batch
    fn add_particles_to_batch(&mut self, win: &Rect) {
        for particle in &self.particle_system.particles {
            if particle.life > 0.0 && win.contains(particle.position) {
                self.batch.add_particle(
                    particle.position,
                    particle.velocity,
                    particle.size,
                    particle.color,
                    particle.life,
                    particle.max_life
                );
            }
        }
    }

    /// Crea un color basado en parámetros visuales
    fn create_color_from_params(&self, base_color: Srgb<f32>, params: &VisualParams, alpha: f32) -> Srgba<f32> {
        let hue_shift = params.color_hue * 0.3;
        let saturation = 0.7 + (params.intensity * 0.3);
        let lightness = 0.5 + (params.intensity * 0.3);
        
        // Aplicar transformaciones básicas al color base
        let adjusted_r = (base_color.red + hue_shift).clamp(0.0, 1.0);
        let adjusted_g = (base_color.green * saturation).clamp(0.0, 1.0);
        let adjusted_b = (base_color.blue * lightness).clamp(0.0, 1.0);
        
        srgba(adjusted_r, adjusted_g, adjusted_b, alpha)
    }
}

/// Sistema de partículas optimizado
pub struct ParticleSystem {
    pub particles: Vec<Particle>,
    pub max_particles: usize,
}

#[derive(Debug, Clone)]
pub struct Particle {
    pub position: Vec2,
    pub velocity: Vec2,
    pub size: f32,
    pub color: Srgba<f32>,
    pub life: f32,
    pub max_life: f32,
}

impl ParticleSystem {
    pub fn new() -> Self {
        Self {
            particles: Vec::with_capacity(2000),
            max_particles: 2000,
        }
    }

    pub fn update(&mut self, _current_time: f32) {
        // Actualizar partículas existentes
        for particle in &mut self.particles {
            particle.position += particle.velocity * 0.016; // 60 FPS
            particle.life -= 0.016;
            particle.velocity *= 0.98; // Fricción
        }

        // Eliminar partículas muertas
        self.particles.retain(|p| p.life > 0.0);
    }

    pub fn emit_particle(&mut self, position: Vec2, velocity: Vec2, color: Srgba<f32>, life: f32) {
        if self.particles.len() < self.max_particles {
            self.particles.push(Particle {
                position,
                velocity,
                size: 2.0,
                color,
                life,
                max_life: life,
            });
        }
    }

    pub fn emit_burst(&mut self, position: Vec2, count: f32, color: Srgba<f32>) {
        let particle_count = count.min(50.0) as usize;
        for _ in 0..particle_count {
            let angle = random_range(0.0, TAU);
            let speed = random_range(20.0, 100.0);
            let velocity = pt2(angle.cos() * speed, angle.sin() * speed);
            
            self.emit_particle(position, velocity, color, random_range(0.5, 2.0));
        }
    }
}
