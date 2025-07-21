// 🎨 Renderizador optimizado con batching
// Renderiza elementos visuales agrupados para máximo rendimiento

use std::cell::RefCell;
use nannou::prelude::*;
use crate::visual::batching::*;
use crate::visual::time_sync::{TimeSync, EventTimeState};
use crate::musical_events::MusicalEvent;
use tracing::{debug, info};

/// Renderizador optimizado que usa batching para mejor rendimiento
pub struct OptimizedRenderer {
    pub batched_renderer: RefCell<BatchedRenderer>,
    pub render_stats: RefCell<RenderStats>,
    pub enable_debug_info: RefCell<bool>,
    pub time_sync: RefCell<TimeSync>,
    pub enable_time_sync: RefCell<bool>,
}

#[derive(Debug, Default, Clone)]
pub struct RenderStats {
    pub frames_rendered: u64,
    pub total_elements_rendered: u64,
    pub draw_calls_saved: u64,
    pub last_frame_elements: usize,
    pub avg_fps: f32,
    pub frame_time_ms: f32,
    pub sync_active_events: usize,
    pub sync_total_events: usize,
    pub debug_log_counter: u32,  // Contador para reducir frecuencia de logs
}

impl OptimizedRenderer {
    /// Crea un nuevo renderizador optimizado con sincronización temporal habilitada
    pub fn new() -> Self {
        info!("🎨 Inicializando renderizador optimizado con batching y sincronización temporal");
        
        let mut time_sync = TimeSync::new();
        time_sync.set_start_time(0.0); // Establecer tiempo de inicio
        
        Self {
            batched_renderer: RefCell::new(BatchedRenderer::new()),
            render_stats: RefCell::new(RenderStats::default()),
            enable_debug_info: RefCell::new(false),
            time_sync: RefCell::new(time_sync),
            enable_time_sync: RefCell::new(true), // Habilitado por defecto
        }
    }

    /// Renderiza todos los eventos usando batching optimizado y sincronización temporal
    pub fn render(&self, draw: &Draw, win: &Rect, events: &[MusicalEvent], current_time: f32) {
        let start_time = std::time::Instant::now();
        
        // Filtrar eventos según sincronización temporal si está habilitada
        let (visible_events, sync_stats) = if *self.enable_time_sync.borrow() {
            let time_sync = self.time_sync.borrow();
            let visible_indices = time_sync.filter_visible_events(events, current_time);
            let stats = time_sync.get_sync_stats(events, current_time);
            
            let visible_events: Vec<&MusicalEvent> = visible_indices.iter()
                .map(|(index, _)| &events[*index])
                .collect();
                
            // Solo mostrar mensaje de debug cada 60 frames (aproximadamente 1 segundo)
            let mut render_stats = self.render_stats.borrow_mut();
            render_stats.debug_log_counter += 1;
            if render_stats.debug_log_counter >= 60 {
                render_stats.debug_log_counter = 0;
                debug!("⏱️ Eventos sincronizados: {}/{} visibles", 
                       stats.active, stats.total_events);
            }
            drop(render_stats);
                   
            (visible_events, Some(stats))
        } else {
            (events.iter().collect(), None)
        };
        
        // Procesar eventos visibles y generar batches
        self.process_synchronized_events(draw, win, &visible_events, current_time);
        
        // Renderizar cada tipo de elemento en batch
        self.render_points_batch(draw);
        self.render_lines_batch(draw);
        self.render_particles_batch(draw);
        self.render_clusters_batch(draw);
        
        // Renderizar información de debug si está habilitada
        if *self.enable_debug_info.borrow() {
            self.render_debug_info(draw, win, sync_stats.as_ref());
        }
        
        // Actualizar estadísticas
        self.update_stats(start_time, sync_stats);
        
        // Log de rendimiento cada 60 frames
        if self.render_stats.borrow().frames_rendered % 60 == 0 {
            self.log_performance();
        }
        
        // Dibuja la barra de timeline profesional
        crate::visual::renderer::draw_timeline_bar(draw, win, current_time, &crate::config::AppConfig::default());
    }

    /// Procesa eventos con sincronización temporal
    fn process_synchronized_events(&self, draw: &Draw, win: &Rect, events: &[&MusicalEvent], current_time: f32) {
        // Limpiar el batch del frame anterior
        self.batched_renderer.borrow_mut().batch.clear();
        
        for &event in events {
            self.process_synchronized_event(draw, win, event, current_time);
        }
        
        // Actualizar sistema de partículas
        self.batched_renderer.borrow_mut().particle_system.update(current_time);
        
        // Añadir partículas al batch
        self.add_particles_to_batch(win);
    }

    /// Procesa un evento individual con sincronización temporal
    fn process_synchronized_event(&self, _draw: &Draw, win: &Rect, event: &MusicalEvent, current_time: f32) {
        if *self.enable_time_sync.borrow() {
            let time_sync = self.time_sync.borrow();
            let event_state = time_sync.get_event_state(event, current_time);
            
            match event_state {
                EventTimeState::NotStarted => {
                    // No renderizar eventos que no han comenzado
                    return;
                },
                EventTimeState::Active { progress } => {
                    // Renderizar evento en progreso
                    self.render_active_event(event, progress, current_time, win);
                },
                EventTimeState::Finished => {
                    // Renderizar evento completo
                    self.render_finished_event(event, current_time, win);
                }
            }
        } else {
            // Renderizado sin sincronización (comportamiento original)
            self.render_event_original(event, current_time, win);
        }
    }

    /// Renderiza un evento que está activo (en progreso)
    fn render_active_event(&self, event: &MusicalEvent, progress: f32, current_time: f32, win: &Rect) {
        match event {
            MusicalEvent::Glissando { start_freq, end_freq, amp, .. } => {
                self.render_synchronized_glissando(*start_freq, *end_freq, *amp, progress, 
                                                 event, current_time, win);
            },
            MusicalEvent::Point { freq, amp, .. } => {
                // Para puntos, mostrar solo si está dentro de su duración
                if progress <= 1.0 {
                    self.render_synchronized_point(*freq, *amp, progress, event, current_time, win);
                }
            },
            MusicalEvent::Cluster { center_freq, bandwidth, amp, .. } => {
                self.render_synchronized_cluster(*center_freq, *bandwidth, *amp, progress, 
                                               event, current_time, win);
            },
            MusicalEvent::Noise { center_freq, bandwidth, amp, .. } => {
                self.render_synchronized_noise(*center_freq, *bandwidth, *amp, progress, 
                                             event, current_time, win);
            },
        }
    }

    /// Renderiza un evento que ya terminó (mostrar completo)
    fn render_finished_event(&self, event: &MusicalEvent, current_time: f32, win: &Rect) {
        // Para eventos terminados, mostrar la versión completa
        self.render_active_event(event, 1.0, current_time, win);
    }

    /// Renderiza un glissando sincronizado en tiempo real
    fn render_synchronized_glissando(&self, start_freq: f32, end_freq: f32, amp: f32, progress: f32,
                                   event: &MusicalEvent, current_time: f32, win: &Rect) {
        let event_time = event.start_time();
        let params = event.visual_params();
        
        // Calcular posiciones
        // Mapeo de tiempo y posición mejorado con márgenes
        let time_start = current_time - 1.0; // Mostrar un poco del pasado
        let time_end = current_time + 9.0;   // Y más del futuro
        let x1 = map_range(event_time, time_start, time_end, win.left() + 60.0, win.right() - 60.0);
        let x2 = map_range(event_time + event.duration(), time_start, time_end, 
                          win.left() + 60.0, win.right() - 60.0);
        let y1 = map_range(start_freq, 20.0, 2000.0, win.bottom() + 40.0, win.top() - 40.0);
        let y2 = map_range(end_freq, 20.0, 2000.0, win.bottom() + 40.0, win.top() - 40.0);
        
        // Calcular punto actual del glissando basado en progreso
        let current_x = x1 + (x2 - x1) * progress;
        let current_y = y1 + (y2 - y1) * progress;
        
        let weight = map_range(amp, 0.0, 1.0, 1.0, 8.0) * params.spatial_spread;
        let alpha = map_range(amp, 0.0, 1.0, 0.4, 0.8) * params.intensity;
        let color = self.create_color_from_params(srgb(1.0, 0.5, 0.3), &params, alpha);
        
        // Renderizar solo la parte que ya ha sonado (línea progresiva)
        if progress > 0.0 {
            self.batched_renderer.borrow_mut().batch.add_line(
                pt2(x1, y1),
                pt2(current_x, current_y),
                weight,
                color,
                1.0 // Línea completa hasta el punto actual
            );
            
            // Añadir punto brillante en la posición actual
            let current_point_radius = weight * 1.5;
            let current_point_alpha = alpha * 1.2;
            let current_point_color = srgba(color.red, color.green, color.blue, current_point_alpha);
            
            self.batched_renderer.borrow_mut().batch.add_point(
                pt2(current_x, current_y),
                current_point_radius,
                current_point_color,
                params.intensity * 1.3,
                params.timbre
            );
            
            // Efectos adicionales para glissando activo
            if progress < 1.0 {
                // Añadir partículas en la posición actual
                self.batched_renderer.borrow_mut().particle_system.emit_particle(
                    pt2(current_x, current_y),
                    pt2(random_range(-20.0, 20.0), random_range(-20.0, 20.0)),
                    color,
                    0.8 + random_range(0.0, 0.4)
                );
                
                // Crear efecto de "trail" con puntos más pequeños
                let trail_steps = 5;
                for i in 1..trail_steps {
                    let trail_progress = (progress - (i as f32 * 0.05)).max(0.0);
                    if trail_progress > 0.0 {
                        let trail_x = x1 + (x2 - x1) * trail_progress;
                        let trail_y = y1 + (y2 - y1) * trail_progress;
                        let trail_alpha = alpha * 0.3 * (1.0 - i as f32 / trail_steps as f32);
                        let trail_color = srgba(color.red, color.green, color.blue, trail_alpha);
                        
                        self.batched_renderer.borrow_mut().batch.add_point(
                            pt2(trail_x, trail_y),
                            weight * 0.3,
                            trail_color,
                            params.intensity * 0.5,
                            params.timbre
                        );
                    }
                }
            }
        }
        
        // Si está cerca del final, mostrar destino tenuemente
        if progress > 0.7 {
            let preview_alpha = ((progress - 0.7) * 3.33).min(1.0) * alpha * 0.4;
            let preview_color = srgba(color.red * 0.8, color.green * 0.8, color.blue * 0.8, preview_alpha);
            
            // Línea tenue hacia el destino
            self.batched_renderer.borrow_mut().batch.add_line(
                pt2(current_x, current_y),
                pt2(x2, y2),
                weight * 0.5,
                preview_color,
                1.0
            );
            
            // Punto destino tenue
            self.batched_renderer.borrow_mut().batch.add_point(
                pt2(x2, y2),
                weight * 0.8,
                preview_color,
                params.intensity * 0.4,
                params.timbre
            );
        }
        
        debug!("🎵 Glissando sincronizado: {:.1}Hz→{:.1}Hz, progreso: {:.1}%", 
              start_freq, end_freq, progress * 100.0);
    }

    /// Renderiza un punto sincronizado con envolvente temporal
    fn render_synchronized_point(&self, freq: f32, amp: f32, progress: f32,
                               event: &MusicalEvent, current_time: f32, win: &Rect) {
        let event_time = event.start_time();
        let params = event.visual_params();
        
        // Mapeo de coordenadas mejorado para puntos
        let time_start = current_time - 1.0; 
        let time_end = current_time + 9.0;   
        let x = map_range(event_time, time_start, time_end, win.left() + 60.0, win.right() - 60.0);
        let y = map_range(freq, 20.0, 2000.0, win.bottom() + 40.0, win.top() - 40.0);
        
        // Envolvente visual más sofisticada: ADSR simplificado
        let attack_time = 0.1;   // 10% del tiempo total
        let decay_time = 0.3;    // 30% del tiempo total
        let sustain_level = 0.7; // 70% del nivel máximo
        let release_time = 0.6;  // 60% del tiempo total
        
        let envelope = if progress < attack_time {
            // Fase de ataque: crecimiento rápido
            progress / attack_time
        } else if progress < attack_time + decay_time {
            // Fase de decaimiento: baja al nivel de sostén
            let decay_progress = (progress - attack_time) / decay_time;
            1.0 - (1.0 - sustain_level) * decay_progress
        } else if progress < attack_time + decay_time + release_time {
            // Fase de sostén: mantiene nivel
            sustain_level
        } else {
            // Fase de liberación: decaimiento final
            let release_progress = (progress - attack_time - decay_time - release_time) / 
                                  (1.0 - attack_time - decay_time - release_time);
            sustain_level * (1.0 - release_progress)
        };
        
        let base_radius = map_range(amp, 0.0, 1.0, 5.0, 25.0); // Radios más grandes
        let radius = base_radius * params.spatial_spread * envelope.max(0.2); // Mínimo 20% de visibilidad
        let alpha = map_range(amp, 0.0, 1.0, 0.5, 0.95) * params.intensity * envelope.max(0.3); // Alpha mínimo
        let color = self.create_color_from_params(srgb(0.2, 0.6, 1.0), &params, alpha);
        
        // Punto principal
        self.batched_renderer.borrow_mut().batch.add_point(
            pt2(x, y),
            radius,
            color,
            params.intensity * envelope,
            params.timbre
        );
        
        // Efectos adicionales basados en el progreso
        if progress < 0.2 {
            // Fase de ataque: burst de partículas
            let burst_intensity = (progress / 0.2) * amp;
            if burst_intensity > 0.3 {
                self.batched_renderer.borrow_mut().particle_system.emit_burst(
                    pt2(x, y), 
                    burst_intensity * 20.0, 
                    color
                );
            }
        } else if progress > 0.8 {
            // Fase final: partículas dispersas
            let particle_count = ((1.0 - progress) * 5.0 * amp) as usize;
            for _ in 0..particle_count {
                let radius_safe = radius.max(1.0); // Evitar rangos vacíos
                let offset_x = random_range(-radius_safe, radius_safe);
                let offset_y = random_range(-radius_safe, radius_safe);
                let velocity = pt2(offset_x * 0.5, offset_y * 0.5);
                let fade_color = srgba(color.red, color.green, color.blue, color.alpha * 0.6);
                
                self.batched_renderer.borrow_mut().particle_system.emit_particle(
                    pt2(x + offset_x, y + offset_y),
                    velocity,
                    fade_color,
                    1.0 + random_range(0.0, 1.0)
                );
            }
        }
        
        // Añadir anillo exterior para eventos con alta amplitud
        if amp > 0.7 && envelope > 0.5 {
            let ring_radius = radius * 1.5;
            let ring_alpha = alpha * 0.3;
            let ring_color = srgba(color.red, color.green, color.blue, ring_alpha);
            
            self.batched_renderer.borrow_mut().batch.add_point(
                pt2(x, y),
                ring_radius,
                ring_color,
                params.intensity * 0.3,
                params.timbre
            );
        }
        
        debug!("🎵 Punto sincronizado: {:.1}Hz, amp: {:.2}, progreso: {:.1}%, envelope: {:.2}", 
              freq, amp, progress * 100.0, envelope);
    }

    /// Renderiza un cluster sincronizado
    fn render_synchronized_cluster(&self, center_freq: f32, bandwidth: f32, amp: f32, progress: f32,
                                 event: &MusicalEvent, current_time: f32, win: &Rect) {
        let event_time = event.start_time();
        let params = event.visual_params();
        
        // Mapeo de coordenadas mejorado para clusters
        let time_start = current_time - 1.0;
        let time_end = current_time + 9.0;
        let x = map_range(event_time, time_start, time_end, win.left() + 60.0, win.right() - 60.0);
        let y = map_range(center_freq, 20.0, 2000.0, win.bottom() + 40.0, win.top() - 40.0);
        
        // El cluster se expande gradualmente
        let radius = map_range(bandwidth, 0.0, 1000.0, 10.0, 100.0) * params.spatial_spread * progress;
        let alpha = map_range(amp, 0.0, 1.0, 0.2, 0.6) * params.intensity;
        let color = self.create_color_from_params(srgb(0.5, 0.0, 0.5), &params, alpha);
        
        self.batched_renderer.borrow_mut().batch.add_cluster(
            pt2(x, y),
            radius,
            color,
            params.intensity,
            params.spatial_spread * progress  // Expansión gradual
        );
    }

    /// Renderiza ruido sincronizado
    fn render_synchronized_noise(&self, center_freq: f32, bandwidth: f32, amp: f32, progress: f32,
                               event: &MusicalEvent, current_time: f32, win: &Rect) {
        let event_time = event.start_time();
        let params = event.visual_params();
        
        // Mapeo de coordenadas mejorado para ruido
        let time_start = current_time - 1.0;
        let time_end = current_time + 9.0;
        let x = map_range(event_time, time_start, time_end, win.left() + 60.0, win.right() - 60.0);
        let y = map_range(center_freq, 20.0, 2000.0, win.bottom() + 40.0, win.top() - 40.0);
        
        // Generar partículas para simular ruido, cantidad basada en progreso
        let particle_count = (bandwidth / 50.0 * amp * 20.0 * progress) as usize;
        let color = self.create_color_from_params(srgb(1.0, 1.0, 0.0), &params, 0.4);
        
        for _ in 0..particle_count {
            let bandwidth_safe = (bandwidth / 10.0).max(1.0); // Evitar rangos vacíos
            let offset_x = random_range(-bandwidth_safe, bandwidth_safe);
            let offset_y = random_range(-bandwidth_safe, bandwidth_safe);
            let velocity = pt2(random_range(-50.0, 50.0), random_range(-50.0, 50.0));
            
            self.batched_renderer.borrow_mut().particle_system.emit_particle(
                pt2(x + offset_x, y + offset_y),
                velocity,
                color,
                1.0 + random_range(0.0, 2.0)
            );
        }
    }

    /// Renderiza evento sin sincronización (comportamiento original)
    fn render_event_original(&self, event: &MusicalEvent, current_time: f32, win: &Rect) {
        // Usar el renderizador de batching original
        let _event_time = event.start_time();
        let progress = event.progress(current_time);
        let _params = event.visual_params();
        
        match event {
            MusicalEvent::Point { freq, amp, .. } => {
                self.render_synchronized_point(*freq, *amp, 1.0, event, current_time, win);
            },
            MusicalEvent::Glissando { start_freq, end_freq, amp, .. } => {
                self.render_synchronized_glissando(*start_freq, *end_freq, *amp, progress.min(1.0), 
                                                 event, current_time, win);
            },
            MusicalEvent::Cluster { center_freq, bandwidth, amp, .. } => {
                self.render_synchronized_cluster(*center_freq, *bandwidth, *amp, progress.min(1.0), 
                                               event, current_time, win);
            },
            MusicalEvent::Noise { center_freq, bandwidth, amp, .. } => {
                self.render_synchronized_noise(*center_freq, *bandwidth, *amp, progress.min(1.0), 
                                             event, current_time, win);
            },
        }
    }

    /// Renderiza todos los puntos en un batch optimizado
    fn render_points_batch(&self, draw: &Draw) {
        let batched_renderer = self.batched_renderer.borrow();
        let points = &batched_renderer.batch.points;
        
        if points.is_empty() {
            return;
        }
        
        // Agrupar puntos por tipo de timbre para optimización adicional
        let (smooth_points, rough_points): (Vec<_>, Vec<_>) = points.iter()
            .partition(|p| p.timbre < 0.7);
        
        // Renderizar puntos suaves en batch
        if !smooth_points.is_empty() {
            self.render_smooth_points_batch(draw, &smooth_points);
        }
        
        // Renderizar puntos rugosos en batch
        if !rough_points.is_empty() {
            self.render_rough_points_batch(draw, &rough_points);
        }
    }

    /// Renderiza puntos suaves en batch optimizado
    fn render_smooth_points_batch(&self, draw: &Draw, points: &[&PointInstance]) {
        for point in points {
            draw.ellipse()
                .x_y(point.position.x, point.position.y)
                .radius(point.radius)
                .color(point.color);
        }
    }

    /// Renderiza puntos rugosos en batch
    fn render_rough_points_batch(&self, draw: &Draw, points: &[&PointInstance]) {
        for point in points {
            let sub_points = (5.0 * point.timbre) as i32;
            
            // Crear múltiples círculos pequeños para efecto rugoso
            for _ in 0..sub_points {
                let radius_half = (point.radius * 0.5).max(1.0); // Evitar rangos vacíos
                let offset_x = random_range(-radius_half, radius_half);
                let offset_y = random_range(-radius_half, radius_half);
                let small_radius = point.radius * 0.3;
                
                draw.ellipse()
                    .x_y(point.position.x + offset_x, point.position.y + offset_y)
                    .radius(small_radius)
                    .color(point.color);
            }
        }
    }

    /// Renderiza todas las líneas en batch optimizado
    fn render_lines_batch(&self, draw: &Draw) {
        let batched_renderer = self.batched_renderer.borrow();
        let lines = &batched_renderer.batch.lines;
        
        if lines.is_empty() {
            return;
        }
        
        for line in lines {
            if line.progress >= 1.0 {
                // Línea completa
                draw.line()
                    .start(line.start)
                    .end(line.end)
                    .weight(line.weight)
                    .color(line.color);
            } else {
                // Línea parcial (glissando en progreso)
                let current_end = line.start + (line.end - line.start) * line.progress;
                draw.line()
                    .start(line.start)
                    .end(current_end)
                    .weight(line.weight)
                    .color(line.color);
            }
        }
    }

    /// Renderiza todas las partículas en batch optimizado
    fn render_particles_batch(&self, draw: &Draw) {
        let batched_renderer = self.batched_renderer.borrow();
        let particles = &batched_renderer.batch.particles;
        
        if particles.is_empty() {
            return;
        }
        
        for particle in particles {
            let life_alpha = particle.life / particle.max_life;
            let mut color = particle.color;
            color.alpha *= life_alpha;
            
            draw.ellipse()
                .x_y(particle.position.x, particle.position.y)
                .radius(particle.size)
                .color(color);
        }
    }

    /// Renderiza todos los clusters en batch optimizado
    fn render_clusters_batch(&self, draw: &Draw) {
        let batched_renderer = self.batched_renderer.borrow();
        let clusters = &batched_renderer.batch.clusters;
        
        if clusters.is_empty() {
            return;
        }
        
        for cluster in clusters {
            // Renderizar círculo exterior del cluster
            draw.ellipse()
                .x_y(cluster.center.x, cluster.center.y)
                .radius(cluster.radius)
                .stroke(cluster.color)
                .stroke_weight(2.0)
                .no_fill();
            
            // Renderizar puntos internos del cluster
            let point_count = (cluster.density * 20.0).max(5.0) as usize; // Mínimo 5 puntos
            for _ in 0..point_count {
                let angle = random_range(0.0, TAU);
                // Evitar rangos vacíos: garantizar al menos 5.0 de radio efectivo
                let max_distance = (cluster.radius * cluster.spread).max(5.0);
                let distance = random_range(0.0, max_distance);
                let x = cluster.center.x + angle.cos() * distance;
                let y = cluster.center.y + angle.sin() * distance;
                
                draw.ellipse()
                    .x_y(x, y)
                    .radius(2.0)
                    .color(cluster.color);
            }
        }
    }

    /// Renderiza información de debug y estadísticas
    fn render_debug_info(&self, draw: &Draw, win: &Rect, sync_stats: Option<&crate::visual::time_sync::SyncStats>) {
        let batched_renderer = self.batched_renderer.borrow();
        let stats = batched_renderer.batch.stats();
        let render_stats = self.render_stats.borrow();

        let mut debug_text = format!(
            "🎨 RENDER STATS\n\
             Points: {}\n\
             Lines: {}\n\
             Particles: {}\n\
             Clusters: {}\n\
             Total: {}\n\
             FPS: {:.1}\n\
             Frame Time: {:.2}ms",
            stats.points,
            stats.lines,
            stats.particles,
            stats.clusters,
            stats.total_elements,
            render_stats.avg_fps,
            render_stats.frame_time_ms
        );

        // Añadir información de sincronización si está disponible
        if let Some(sync) = sync_stats {
            debug_text.push_str(&format!(
                "\n\n⏱️ TIME SYNC\n\
                 Active: {}\n\
                 Finished: {}\n\
                 Not Started: {}\n\
                 Total Events: {}\n\
                 Completion: {:.1}%",
                sync.active,
                sync.finished,
                sync.not_started,
                sync.total_events,
                sync.completion_percentage()
            ));
        }
        
        // Posicionar el texto de debug en la esquina inferior izquierda para evitar superposición
        // con las etiquetas de frecuencia que están en la parte superior
        draw.text(&debug_text)
            .x_y(win.left() + 10.0, win.bottom() + 80.0)  // Mover a la parte inferior
            .font_size(10)  // Tamaño menor para menos intrusión
            .color(srgba(1.0, 1.0, 1.0, 0.9))  // Ligeramente menos opaco
            .left_justify();
    }

    /// Actualiza estadísticas de rendimiento
    fn update_stats(&self, start_time: std::time::Instant, sync_stats: Option<crate::visual::time_sync::SyncStats>) {
        let elapsed = start_time.elapsed();
        let frame_time_ms = elapsed.as_secs_f32() * 1000.0;
        
        let mut render_stats = self.render_stats.borrow_mut();
        render_stats.frames_rendered += 1;
        render_stats.frame_time_ms = frame_time_ms;
        render_stats.avg_fps = 1000.0 / frame_time_ms;
        
        let batched_renderer = self.batched_renderer.borrow();
        let stats = batched_renderer.batch.stats();
        render_stats.last_frame_elements = stats.total_elements;
        render_stats.total_elements_rendered += stats.total_elements as u64;
        
        // Calcular draw calls ahorrados (estimación)
        let traditional_draw_calls = stats.total_elements;
        let batched_draw_calls = 4; // Points, Lines, Particles, Clusters
        
        // Verificar que no haya overflow antes de restar
        if traditional_draw_calls >= batched_draw_calls {
            render_stats.draw_calls_saved += (traditional_draw_calls - batched_draw_calls) as u64;
        }
        
        // Actualizar estadísticas de sincronización temporal
        if let Some(sync) = sync_stats {
            render_stats.sync_active_events = sync.active;
            render_stats.sync_total_events = sync.total_events;
        }
    }

    /// Registra información de rendimiento en los logs
    fn log_performance(&self) {
        let stats = self.render_stats.borrow();
        
        debug!(
            "🎨 Rendimiento: {} frames, {} elementos, {:.1} FPS promedio, {:.2}ms por frame",
            stats.frames_rendered,
            stats.total_elements_rendered,
            stats.avg_fps,
            stats.frame_time_ms
        );
        
        if stats.frames_rendered % 300 == 0 {
            info!(
                "🏆 Optimización: {} draw calls ahorrados, {} elementos renderizados",
                stats.draw_calls_saved,
                stats.total_elements_rendered
            );
        }
    }

    /// Habilita/deshabilita información de debug
    pub fn toggle_debug_info(&self) {
        let mut debug_info = self.enable_debug_info.borrow_mut();
        *debug_info = !*debug_info;
        info!("🔍 Debug info: {}", if *debug_info { "ON" } else { "OFF" });
    }

    /// Habilita/deshabilita sincronización temporal
    pub fn toggle_time_sync(&self) {
        let mut time_sync_enabled = self.enable_time_sync.borrow_mut();
        *time_sync_enabled = !*time_sync_enabled;
        info!("⏱️ Sincronización temporal: {}", if *time_sync_enabled { "ON" } else { "OFF" });
    }

    /// Establece el tiempo de inicio para sincronización
    pub fn set_sync_start_time(&self, start_time: f32) {
        self.time_sync.borrow_mut().set_start_time(start_time);
    }

    /// Establece un offset de tiempo para sincronización
    pub fn set_sync_time_offset(&self, offset: f32) {
        self.time_sync.borrow_mut().set_time_offset(offset);
    }

    /// Retorna si la sincronización temporal está habilitada
    pub fn is_time_sync_enabled(&self) -> bool {
        *self.enable_time_sync.borrow()
    }

    /// Retorna estadísticas actuales
    pub fn get_stats(&self) -> RenderStats {
        self.render_stats.borrow().clone()
    }

    /// Retorna estadísticas del batch
    pub fn get_batch_stats(&self) -> BatchStats {
        self.batched_renderer.borrow().batch.stats()
    }

    /// Añade partículas del sistema al batch
    fn add_particles_to_batch(&self, win: &Rect) {
        let mut batched_renderer = self.batched_renderer.borrow_mut();
        
        // Recoger información de partículas válidas primero
        let valid_particles: Vec<_> = batched_renderer.particle_system.particles.iter()
            .filter(|p| p.life > 0.0 && win.contains(p.position))
            .map(|p| (p.position, p.velocity, p.size, p.color, p.life, p.max_life))
            .collect();
        
        // Agregar las partículas válidas al batch
        for (position, velocity, size, color, life, max_life) in valid_particles {
            batched_renderer.batch.add_particle(
                position,
                velocity,
                size,
                color,
                life,
                max_life
            );
        }
    }

    /// Crea un color basado en parámetros visuales
    fn create_color_from_params(&self, base_color: Srgb<f32>, params: &crate::musical_events::VisualParams, alpha: f32) -> Srgba<f32> {
        let hue_shift = params.color_hue * 0.3;
        let saturation = 0.7 + (params.intensity * 0.3);
        let lightness = 0.5 + (params.intensity * 0.3);
        
        // Aplicar transformaciones básicas al color base
        let adjusted_r = (base_color.red + hue_shift * 0.01).clamp(0.0, 1.0);
        let adjusted_g = (base_color.green * saturation).clamp(0.0, 1.0);
        let adjusted_b = (base_color.blue * lightness).clamp(0.0, 1.0);
        
        srgba(adjusted_r, adjusted_g, adjusted_b, alpha)
    }
}
