// src/model.rs - Estructuras de datos principales

use nannou::prelude::*;
use nannou_osc as osc;
use crate::config::AppConfig;
use crate::osc_server::OscServer;
use crate::visual::audio_visual_mapping::{VisualNote, AudioVisualMapper, AudioVisualMappingConfig};
use crate::visual::audio_visual_mapping_pro::{ProAudioVisualMapper, ProMappingConfig, ColorPalette, EventKind, VisualShape};
use crate::visual::shader_manager::ShaderManager;

/// Estadísticas de mensajes OSC
#[derive(Debug, Clone, Default)]
pub struct OscStats {
    pub total_messages: u32,
    pub successful_messages: u32,
    pub failed_messages: u32,
    pub last_message_time: f64,
}

impl OscStats {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn success_rate(&self) -> f32 {
        if self.total_messages == 0 {
            0.0
        } else {
            self.successful_messages as f32 / self.total_messages as f32
        }
    }
}

/// Evento musical discreto (nota individual)
#[derive(Debug, Clone)]
pub struct Note {
    pub event_type: String,
    pub instrument: String,  // Nuevo campo para tipo de instrumento
    pub time_alive: f32,
    pub duration: f32,
    pub position: Vec2,
    pub size: f32,
    pub color: Srgb<u8>,
}

/// Alias para compatibilidad con código existente
pub type MusicalEvent = Note;

/// Datos de análisis continuo (FFT, análisis espectral)
#[derive(Debug, Clone, Default)]
pub struct AnalysisData {
    pub amplitude: f32,
    pub brightness: f32,
    pub noisiness: f32,
}

/// Evento de drone (tonos continuos)
#[derive(Debug, Clone)]
pub struct DroneEvent {
    pub frequency: f32,
    pub amplitude: f32,
    pub time_alive: f32,
    pub position: Vec2,
    pub radius: f32,
    pub color: Hsv,
}

/// Datos del cluster (masa de partículas reactiva)
#[derive(Debug, Clone)]
pub struct ClusterData {
    pub frequency: f32,
    pub amplitude: f32,
    pub audio_level: f32,
    pub size: f32,
    pub density: f32,
    pub time_alive: f32,
}

impl Default for ClusterData {
    fn default() -> Self {
        Self {
            frequency: 300.0,
            amplitude: 200.0,
            audio_level: 0.0,
            size: 50.0,
            density: 0.5,
            time_alive: 0.0,
        }
    }
}

/// Configuración de display/visualización
#[derive(Debug, Clone)]
pub struct DisplayConfig {
    pub show_debug: bool,
    pub show_grid: bool,
    pub background_style: BackgroundStyle,
    pub visual_quality: VisualQuality,
}

#[derive(Debug, Clone)]
pub enum BackgroundStyle {
    Modern,
    Simple,
    Gradient,
    None,
}

#[derive(Debug, Clone)]
pub enum VisualQuality {
    Low,
    Medium,
    High,
    Ultra,
}

/// Modo de visualización para diferentes tipos de eventos
#[derive(Debug, Clone)]
pub enum DisplayMode {
    Events,      // Mostrar solo eventos musicales discretos
    Analysis,    // Mostrar solo análisis continuo
    Drones,      // Mostrar solo eventos de drone
    Cluster,     // Mostrar solo cluster de partículas
    Combined,    // Mostrar todo combinado
}

/// Modo de scroll para el visualizador
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ScrollMode {
    Fixed,      // Notas estáticas sin movimiento horizontal
    Scrolling,  // Notas se desplazan horizontalmente con el tiempo
}

impl Default for ScrollMode {
    fn default() -> Self {
        ScrollMode::Fixed
    }
}

impl Default for DisplayConfig {
    fn default() -> Self {
        Self {
            show_debug: true,
            show_grid: true,
            background_style: BackgroundStyle::Modern,
            visual_quality: VisualQuality::High,
        }
    }
}

impl Default for DisplayMode {
    fn default() -> Self {
        DisplayMode::Combined
    }
}

/// Modelo principal de la aplicación
pub struct Model {
    pub osc_server: OscServer,  // Servidor OSC robusto
    pub midi_server: Option<crate::midi::MidiServer>, // Servidor MIDI opcional
    pub config: AppConfig,      // Configuración externa
    
    // Notas visuales avanzadas (nuevo sistema)
    pub visual_notes: Vec<VisualNote>,
    pub audio_visual_mapper: AudioVisualMapper,
    
    // Mapeador profesional (nuevo sistema avanzado)
    pub pro_mapper: ProAudioVisualMapper,
    
    // Sistema de gestión de shaders y renderizado profesional
    pub shader_manager: ShaderManager,
    
    // Control de tiempo y estado
    pub elapsed_time: f32,
    pub scroll_mode: ScrollMode,
    pub scroll_speed: f32,  // Velocidad de scroll en píxeles por segundo
    
    // Datos legacy (mantener para compatibilidad)
    pub notes: Vec<Note>,
    pub analysis: AnalysisData,
    pub drone_events: Vec<DroneEvent>,
    pub cluster_data: ClusterData,
    pub display_config: DisplayConfig,
    pub display_mode: DisplayMode,
    pub osc_stats: OscStats,
    
    // Alias para compatibilidad
    pub events: Vec<MusicalEvent>,
}

impl Model {
    /// Crea una nueva instancia del modelo (método legacy)
    pub fn new(_receiver: osc::Receiver) -> Self {
        let config = AppConfig::default();
        let osc_server = OscServer::new(config.osc.clone())
            .expect("Error creando servidor OSC con configuración por defecto");
            
        Model {
            osc_server,
            midi_server: None, // Inicializar sin servidor MIDI
            config,
            visual_notes: Vec::new(),
            audio_visual_mapper: AudioVisualMapper::new(AudioVisualMappingConfig::default()),
            pro_mapper: ProAudioVisualMapper::new(ProMappingConfig::default(), ColorPalette::Classical),
            shader_manager: ShaderManager::default(),
            elapsed_time: 0.0,
            scroll_mode: ScrollMode::default(),
            scroll_speed: 100.0, // 100 píxeles por segundo por defecto
            notes: Vec::new(),
            analysis: AnalysisData::default(),
            drone_events: Vec::new(),
            cluster_data: ClusterData::default(),
            display_config: DisplayConfig::default(),
            display_mode: DisplayMode::default(),
            osc_stats: OscStats::new(),
            events: Vec::new(), // Alias para compatibilidad
        }
    }
    
    /// Crea una nueva instancia del modelo con configuración externa
    pub fn new_with_config(osc_server: OscServer, config: AppConfig) -> Self {
        Model {
            osc_server,
            midi_server: None, // Inicializar sin servidor MIDI
            config,
            visual_notes: Vec::new(),
            audio_visual_mapper: AudioVisualMapper::new(AudioVisualMappingConfig::default()),
            pro_mapper: ProAudioVisualMapper::new(ProMappingConfig::default(), ColorPalette::Classical),
            shader_manager: ShaderManager::default(),
            elapsed_time: 0.0,
            scroll_mode: ScrollMode::default(),
            scroll_speed: 100.0, // 100 píxeles por segundo por defecto
            notes: Vec::new(),
            analysis: AnalysisData::default(),
            drone_events: Vec::new(),
            cluster_data: ClusterData::default(),
            display_config: DisplayConfig::default(),
            display_mode: DisplayMode::default(),
            osc_stats: OscStats::new(),
            events: Vec::new(), // Alias para compatibilidad
        }
    }
    
    /// Añade una nueva nota musical (método principal)
    pub fn add_note(&mut self, freq: f32, amp: f32, dur: f32, win: Rect) {
        let pos_y = map_range(freq.log2(), (80.0f32).log2(), (2000.0f32).log2(), win.bottom() + 20.0, win.top() - 20.0);
        let new_note = Note {
            event_type: "note".to_string(),
            instrument: "default".to_string(),  // Valor por defecto
            duration: dur,
            time_alive: 0.0,
            position: pt2(win.left() + 50.0, pos_y),
            size: map_range(amp, 0.0, 1.0, 2.0, 15.0),
            color: Srgb::from_format(hsv(map_range(freq, 200.0, 2000.0, 0.55, 0.95), 0.9, 1.0).into()),
        };
        
        self.notes.push(new_note.clone());
        self.events.push(new_note); // Mantener compatibilidad
    }

    /// Método para añadir un nuevo evento discreto (alias para compatibilidad)
    pub fn add_musical_event(&mut self, freq: f32, amp: f32, dur: f32, win: Rect) {
        self.add_note(freq, amp, dur, win);
    }
    
    /// Añade una nueva nota musical con instrumento específico
    pub fn add_note_with_instrument(&mut self, freq: f32, amp: f32, dur: f32, instrument: &str, win: Rect) {
        let pos_y = map_range(freq.log2(), (80.0f32).log2(), (2000.0f32).log2(), win.bottom() + 20.0, win.top() - 20.0);
        
        // Color basado en el instrumento
        let hue = match instrument {
            "piano" => 0.6,
            "guitar" => 0.3,
            "violin" => 0.8,
            "brass" => 0.1,
            "percussion" => 0.0,
            _ => map_range(freq, 200.0, 2000.0, 0.55, 0.95),
        };
        
        let new_note = Note {
            event_type: "note".to_string(),
            instrument: instrument.to_string(),
            duration: dur,
            time_alive: 0.0,
            position: pt2(win.left() + 50.0, pos_y),
            size: map_range(amp, 0.0, 1.0, 2.0, 15.0),
            color: Srgb::from_format(hsv(hue, 0.9, 1.0).into()),
        };
        
        self.notes.push(new_note.clone());
        self.events.push(new_note); // Mantener compatibilidad
        
        // Añadir también nota visual avanzada
        self.add_visual_note(freq, amp, dur, instrument, win);
    }
    
    /// Actualiza los datos de análisis continuo
    pub fn update_analysis_data(&mut self, amp: f32, bright: f32, noisy: f32) {
        self.analysis.amplitude = amp;
        self.analysis.brightness = bright;
        self.analysis.noisiness = noisy;
    }

    /// Añade un evento de drone
    pub fn add_drone_event(&mut self, freq: f32, amp: f32) {
        let drone_event = DroneEvent {
            frequency: freq,
            amplitude: amp,
            time_alive: 0.0,
            position: pt2(0.0, 0.0), // Centro de la pantalla
            radius: map_range(amp, 0.0, 1.0, 50.0, 200.0).max(50.0),
            color: hsv(map_range(freq, 80.0, 800.0, 0.0, 0.7), 0.9, 1.0),
        };
        
        println!("🎵 Drone añadido: {}Hz {}amp radio:{}", freq, amp, drone_event.radius);
        
        // Mantener solo los eventos de drone más recientes
        if self.drone_events.len() > 3 {
            self.drone_events.remove(0);
        }
        self.drone_events.push(drone_event);
    }

    /// Actualiza los datos del cluster
    pub fn update_cluster_data(&mut self, freq: f32, amp: f32, level: f32) {
        self.cluster_data.frequency = freq;
        self.cluster_data.amplitude = amp;
        self.cluster_data.audio_level = level;
        self.cluster_data.time_alive = 0.0; // Resetear tiempo para que se vea activo
        
        // Calcular tamaño basado en frecuencia (inverso: frecuencias altas = cluster pequeño)
        self.cluster_data.size = map_range(freq, 200.0, 600.0, 150.0, 50.0).max(30.0);
        
        // Calcular densidad basada en amplitud
        self.cluster_data.density = map_range(amp, 50.0, 800.0, 0.3, 1.0);
        
        println!("🌊 Cluster: freq={:.1}Hz amp={:.1} size={:.1} density={:.2}", 
                freq, amp, self.cluster_data.size, self.cluster_data.density);
    }

    /// Actualiza el ciclo de vida de todos los eventos
    pub fn update_events(&mut self, dt: f32) {
        // **Aplicar lógica de scroll si está habilitado**
        if self.scroll_mode == ScrollMode::Scrolling {
            self.apply_scroll_movement(dt);
        }
        
        // Actualizar notas musicales
        for note in &mut self.notes {
            note.time_alive += dt;
        }
        self.notes.retain(|note| note.time_alive <= note.duration + 2.0);
        
        // Mantener sincronización con events para compatibilidad
        self.events.clear();
        self.events.extend(self.notes.clone());
        
        // Actualizar eventos de drone
        for drone in &mut self.drone_events {
            drone.time_alive += dt;
        }
        self.drone_events.retain(|drone| drone.time_alive <= 5.0);
        
        // Actualizar cluster data
        self.cluster_data.time_alive += dt;
        
        // Si no hay datos nuevos del cluster por más de 1 segundo, reducir la visualización
        if self.cluster_data.time_alive > 1.0 {
            self.cluster_data.audio_level *= 0.95; // Fade out gradual
        }
    }
    
    /// **Aplica movimiento de scroll horizontal a todos los elementos**
    fn apply_scroll_movement(&mut self, dt: f32) {
        let scroll_delta = -self.scroll_speed * dt; // Negativo para mover hacia la izquierda
        
        // Scroll para notas musicales
        for note in &mut self.notes {
            note.position.x += scroll_delta;
        }
        
        // Scroll para eventos de drone
        for drone in &mut self.drone_events {
            drone.position.x += scroll_delta;
        }
        
        // Scroll para notas visuales avanzadas
        for visual_note in &mut self.visual_notes {
            visual_note.position.x += scroll_delta;
            visual_note.target_position.x += scroll_delta;
        }
        
        // Actualizar posición del cluster (si tiene posición)
        // El cluster_data no tiene posición específica, se renderiza globalmente
        
        // Opcional: eliminar elementos que salen completamente de la pantalla
        // (esto se podría hacer en cleanup_expired_events si se desea)
    }
    
    /// Limpia todos los eventos
    pub fn clear_events(&mut self) {
        self.notes.clear();
        self.events.clear();
        self.drone_events.clear();
        self.cluster_data = ClusterData::default();
        self.analysis = AnalysisData::default();
    }
    
    /// Configura el modo de visualización
    pub fn set_display_mode(&mut self, mode: DisplayMode) {
        self.display_mode = mode;
    }
    
    /// Configura las opciones de display
    pub fn set_display_config(&mut self, show_debug: bool, show_grid: bool) {
        self.display_config.show_debug = show_debug;
        self.display_config.show_grid = show_grid;
    }
    
    /// Configura la calidad visual
    pub fn set_visual_quality(&mut self, quality: VisualQuality) {
        self.display_config.visual_quality = quality;
    }
    
    /// **Configura el modo de scroll**
    pub fn set_scroll_mode(&mut self, mode: ScrollMode) {
        self.scroll_mode = mode;
    }
    
    /// **Obtiene el modo de scroll actual**
    pub fn get_scroll_mode(&self) -> ScrollMode {
        self.scroll_mode
    }
    
    /// **Configura la velocidad de scroll**
    pub fn set_scroll_speed(&mut self, speed: f32) {
        self.scroll_speed = speed.max(0.0); // Asegurar velocidad no negativa
    }
    
    /// **Obtiene la velocidad de scroll actual**
    pub fn get_scroll_speed(&self) -> f32 {
        self.scroll_speed
    }
    
    /// **Alterna entre modo fijo y scroll**
    pub fn toggle_scroll_mode(&mut self) {
        self.scroll_mode = match self.scroll_mode {
            ScrollMode::Fixed => ScrollMode::Scrolling,
            ScrollMode::Scrolling => ScrollMode::Fixed,
        };
    }
    
    /// Obtiene el número total de eventos activos
    pub fn get_active_events_count(&self) -> usize {
        self.notes.len() + self.drone_events.len()
    }
    
    /// Limpia eventos expirados automáticamente
    pub fn cleanup_expired_events(&mut self) {
        let before_notes = self.notes.len();
        let before_drones = self.drone_events.len();
        
        // Limpiar notas expiradas
        self.notes.retain(|note| note.time_alive <= note.duration + 2.0);
        
        // Limpiar drones expirados
        self.drone_events.retain(|drone| drone.time_alive <= 8.0);
        
        // Mantener sincronización con events
        self.events.clear();
        self.events.extend(self.notes.clone());
        
        // Log si se limpiaron eventos
        let cleaned_notes = before_notes - self.notes.len();
        let cleaned_drones = before_drones - self.drone_events.len();
        
        if cleaned_notes > 0 || cleaned_drones > 0 {
            println!("🧹 Eventos expirados limpiados: {} notas, {} drones", 
                   cleaned_notes, cleaned_drones);
        }
    }
    
    /// Inicializa el servidor MIDI si está habilitado en la configuración
    pub fn init_midi(&mut self) {
        if self.config.midi.enabled {
            println!("🎹 Intentando inicializar servidor MIDI...");
            match crate::midi::MidiServer::new(self.config.clone()) {
                Ok(server) => {
                    println!("✅ Servidor MIDI inicializado exitosamente");
                    self.midi_server = Some(server);
                },
                Err(e) => {
                    println!("⚠️  No se pudo inicializar MIDI: {}", e);
                    println!("   Continuando sin soporte MIDI");
                    self.midi_server = None;
                }
            }
        } else {
            println!("🎹 MIDI deshabilitado en configuración");
        }
    }
    
    /// Procesa mensajes MIDI y agrega notas
    pub fn process_midi_messages(&mut self, win: Rect) {
        if let Some(ref midi_server) = self.midi_server {
            let midi_notes = midi_server.get_notes();
            
            for midi_note in midi_notes {
                // Asignar posición basada en frecuencia
                let pos_y = map_range(
                    midi_note.freq.log2(), 
                    (80.0f32).log2(), 
                    (2000.0f32).log2(), 
                    win.bottom() + 20.0, 
                    win.top() - 20.0
                );
                let pos_x = win.left() + win.w() * 0.1; // Posición fija en el lado izquierdo
                
                // Crear Note para el visualizador
                let note = Note {
                    event_type: "midi_note".to_string(),
                    instrument: midi_note.instrument.clone(),
                    time_alive: 0.0,
                    duration: midi_note.dur,
                    position: Vec2::new(pos_x, pos_y),
                    size: midi_note.amp * 50.0 + 10.0,
                    color: self.get_instrument_color(&midi_note.instrument),
                };
                
                self.notes.push(note);
                
                // Incrementar estadísticas
                self.osc_stats.total_messages += 1;
                self.osc_stats.successful_messages += 1;
            }
        }
    }
    
    /// Obtiene el color para un instrumento específico
    pub fn get_instrument_color(&self, instrument: &str) -> Srgb<u8> {
        match instrument {
            "sine" => srgb(0, 120, 255),      // Azul
            "triangle" => srgb(0, 255, 120),  // Verde
            "square" => srgb(255, 60, 60),    // Rojo
            "sawtooth" => srgb(255, 255, 0),  // Amarillo
            "piano" => srgb(160, 60, 255),    // Púrpura
            "bell" => srgb(0, 255, 255),      // Cian
            "pad" => srgb(255, 0, 255),       // Magenta
            "lead" => srgb(255, 165, 0),      // Naranja
            "drums" => srgb(255, 100, 100),   // Rojo claro
            _ => srgb(200, 200, 200),         // Gris por defecto
        }
    }
    
    /// Añade una nueva nota visual avanzada (método principal para el nuevo sistema)
    pub fn add_visual_note(&mut self, freq: f32, amp: f32, dur: f32, instrument: &str, win: Rect) -> &VisualNote {
        let visual_note = self.audio_visual_mapper.map_audio_to_visual(
            freq,
            amp,
            dur,
            instrument,
            win,
        );
        
        self.visual_notes.push(visual_note);
        self.visual_notes.last().unwrap()
    }
    
    /// Actualiza todas las notas visuales (animación)
    pub fn update_visual_notes(&mut self, delta_time: f32, win: Rect) {
        // Actualizar todas las notas
        for note in &mut self.visual_notes {
            self.audio_visual_mapper.update_visual_note(note, delta_time);
        }
        
        // Remover notas que han expirado
        self.visual_notes.retain(|note| 
            !self.audio_visual_mapper.should_remove_note(note, win)
        );
    }
    
    /// Obtiene las notas visuales ordenadas por capa y prioridad
    pub fn get_visual_notes_sorted(&self) -> Vec<&VisualNote> {
        let mut sorted_notes: Vec<&VisualNote> = self.visual_notes.iter().collect();
        sorted_notes.sort_by(|a, b| {
            a.layer.cmp(&b.layer)
                .then(a.priority.cmp(&b.priority))
        });
        sorted_notes
    }
    
    /// Limpia todas las notas visuales
    pub fn clear_visual_notes(&mut self) {
        self.visual_notes.clear();
    }
    
    /// Métodos para el mapeador profesional
    
    /// Calcula posición X usando scroll profesional
    pub fn get_pro_x_position(&self, freq: f32, current_time: f32, note_birth_time: f32) -> f32 {
        self.pro_mapper.freq_to_x_scroll(freq, current_time, note_birth_time)
    }
    
    /// Calcula opacidad usando escala logarítmica profesional
    pub fn get_pro_opacity(&self, amplitude: f32) -> f32 {
        self.pro_mapper.amp_to_opacity(amplitude)
    }
    
    /// Obtiene color usando paletas artísticas profesionales
    pub fn get_pro_color(&self, freq: f32, amplitude: f32) -> Srgb<u8> {
        self.pro_mapper.freq_to_color(freq, amplitude)
    }
    
    /// Obtiene forma visual según tipo de evento profesional
    pub fn get_pro_shape(&self, kind: &EventKind, freq: f32, amplitude: f32, duration: f32) -> VisualShape {
        self.pro_mapper.kind_to_shape(kind, freq, amplitude, duration)
    }
    
    /// Cambia la paleta de colores del mapeador profesional
    pub fn set_pro_color_palette(&mut self, palette: ColorPalette) {
        self.pro_mapper.set_color_palette(palette);
    }
    
    /// Actualiza el tiempo de inicio para cálculos de scroll
    pub fn set_pro_start_time(&mut self, time: f32) {
        self.pro_mapper.set_start_time(time);
    }
    
    /// Método conveniente para crear nota con mapeo profesional
    pub fn add_pro_visual_note(&mut self, freq: f32, amp: f32, dur: f32, instrument: &str, event_kind: EventKind, current_time: f32) {
        // Obtener propiedades usando mapeo profesional
        let opacity = self.get_pro_opacity(amp);
        let color = self.get_pro_color(freq, amp);
        let shape = self.get_pro_shape(&event_kind, freq, amp, dur);
        
        // Crear nota visual usando el sistema existente pero con propiedades profesionales
        let mut visual_note = self.audio_visual_mapper.map_audio_to_visual(
            freq, amp, dur, instrument, 
            Rect::from_w_h(self.pro_mapper.get_config().window_width, self.pro_mapper.get_config().window_height)
        );
        
        // Aplicar mejoras profesionales
        visual_note.opacity = opacity;
        visual_note.color = color;
        
        // Calcular posición X profesional (nota: necesitaríamos el tiempo de nacimiento)
        // visual_note.position.x = self.get_pro_x_position(freq, current_time, current_time);
        
        self.visual_notes.push(visual_note);
    }
    
    /// **Actualiza el tiempo transcurrido en el modelo y shader manager**
    pub fn update_time(&mut self, elapsed: f32) {
        self.elapsed_time = elapsed;
        self.shader_manager.update_time(elapsed);
    }
    
    /// **Crea una nota visual usando el sistema profesional de mapeo**
    pub fn create_professional_note(
        &mut self,
        freq: f32,
        amplitude: f32,
        duration: f32,
        event_type: &str,
        instrument: &str,
    ) {
        use crate::visual::shader_manager::string_to_event_kind;
        
        let event_kind = string_to_event_kind(event_type);
        let birth_time = self.elapsed_time;
        
        let visual_note = self.shader_manager.create_professional_visual_note(
            freq,
            amplitude,
            duration,
            event_kind,
            birth_time,
            instrument,
        );
        
        self.visual_notes.push(visual_note);
    }
}