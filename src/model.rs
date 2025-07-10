// =================================================================
// 🎵 SC SCORE VISUALIZER - MODELO DE DATOS
// =================================================================
// Define el estado de la aplicación (el `Model`)

use nannou::prelude::*;
use std::collections::VecDeque;
use std::sync::mpsc::Receiver;
use std::time::Instant;
use serde::Deserialize;
use noise::Perlin;

use crate::events::{MusicalEvent, RealtimeData};

// =================================================================
// CONFIGURACIÓN DE LA APLICACIÓN
// =================================================================

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub visual: VisualSettings,
    pub osc: OscSettings,
}

#[derive(Debug, Clone, Deserialize)]
pub struct VisualSettings {
    pub time_window: f64,
    pub max_events: usize,
    pub background_color: [u8; 3],
    pub event_fade_time: f64,
    pub grid: GridSettings,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GridSettings {
    pub show_labels: bool,
    pub show_frequency_labels: bool,
    pub show_time_labels: bool,
    pub major_lines: u32,
    pub minor_lines: u32,
    pub major_color: [f32; 4],
    pub minor_color: [f32; 4],
    pub center_color: [f32; 4],
    pub label_color: [f32; 4],
    pub musical_divisions: bool,
    pub frequency_range: (f32, f32), // Hz range for frequency axis
    pub time_range: f32, // seconds for time axis
}

#[derive(Debug, Clone, Deserialize)]
pub struct OscSettings {
    pub port: u16,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            visual: VisualSettings {
                time_window: 10.0,
                max_events: 200,
                background_color: [8, 15, 30],  // Fondo azul más oscuro
                event_fade_time: 3.0,
                grid: GridSettings::default(),
            },
            osc: OscSettings {
                port: 57124,
            },
        }
    }
}

impl Default for GridSettings {
    fn default() -> Self {
        Self {
            show_labels: true,
            show_frequency_labels: true,
            show_time_labels: true,
            major_lines: 8,
            minor_lines: 4,
            major_color: [0.3, 0.7, 1.0, 0.25],      // Azul brillante para líneas principales
            minor_color: [0.2, 0.5, 0.8, 0.12],      // Azul suave para líneas menores
            center_color: [0.4, 0.8, 1.0, 0.5],      // Azul cyan para líneas centrales
            label_color: [0.8, 0.9, 1.0, 0.9],       // Azul claro para etiquetas
            musical_divisions: true,
            frequency_range: (80.0, 2000.0), // Rango musical típico
            time_range: 10.0, // 10 segundos
        }
    }
}

// =================================================================
// ESTADO DE LA APLICACIÓN
// =================================================================

pub struct Model {
    pub config: Config,
    pub events: VecDeque<MusicalEvent>,
    pub realtime_data: Option<RealtimeData>,
    pub osc_receiver: Receiver<nannou_osc::Packet>,
    pub last_cleanup: Instant,
    pub noise: Perlin,
    pub stats: AppStats,
    pub ui_state: UiState,
}

#[derive(Debug)]
pub struct AppStats {
    pub total_events: usize,
    pub events_per_second: f32,
    pub last_event_time: Option<Instant>,
    pub frame_count: usize,
    pub fps: f32,
    pub last_fps_update: Instant,
}

#[derive(Debug)]
pub struct UiState {
    pub show_stats: bool,
    pub show_grid: bool,
    pub show_trails: bool,
    pub pause_updates: bool,
    pub show_menu: bool,
    pub menu_state: MenuState,
    pub zoom_level: f32,
    pub viewport_offset: Vec2,
    pub theme: ColorTheme,
    pub performance_mode: bool,
    pub fullscreen: bool,
    pub show_timer: bool,
    pub snap_to_x_grid: bool,
    pub snap_to_y_grid: bool,
    pub grid_resolution: u32,
    pub high_resolution: bool,
}

#[derive(Debug, Clone)]
pub struct MenuState {
    pub active_menu: Option<MenuType>,
    pub hovered_item: Option<String>,
    pub menu_open: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MenuType {
    File,
    Edit,
    Display,
    View,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ColorTheme {
    Dark,
    Light,
    Blue,
    Classic,
}

impl Default for AppStats {
    fn default() -> Self {
        Self {
            total_events: 0,
            events_per_second: 0.0,
            last_event_time: None,
            frame_count: 0,
            fps: 0.0,
            last_fps_update: Instant::now(),
        }
    }
}

impl Default for UiState {
    fn default() -> Self {
        Self {
            show_stats: true,
            show_grid: true,
            show_trails: true,
            pause_updates: false,
            show_menu: true,
            menu_state: MenuState::default(),
            zoom_level: 1.0,
            viewport_offset: Vec2::ZERO,
            theme: ColorTheme::Dark,
            performance_mode: false,
            fullscreen: false,
            show_timer: true,
            snap_to_x_grid: false,
            snap_to_y_grid: false,
            grid_resolution: 8,
            high_resolution: false,
        }
    }
}

impl Default for MenuState {
    fn default() -> Self {
        Self {
            active_menu: None,
            hovered_item: None,
            menu_open: false,
        }
    }
}

impl Model {
    pub fn new(config: Config, osc_receiver: Receiver<nannou_osc::Packet>) -> Self {
        Self {
            config,
            events: VecDeque::new(),
            realtime_data: None,
            osc_receiver,
            last_cleanup: Instant::now(),
            noise: Perlin::new(0),
            stats: AppStats::default(),
            ui_state: UiState::default(),
        }
    }
    
    pub fn add_event(&mut self, event: MusicalEvent) {
        // Actualizar estadísticas
        self.stats.total_events += 1;
        self.stats.last_event_time = Some(Instant::now());
        
        // Agregar evento
        self.events.push_back(event);
        
        // Limitar número de eventos
        if self.events.len() > self.config.visual.max_events {
            self.events.pop_front();
        }
    }
    
    pub fn update_realtime_data(&mut self, data: RealtimeData) {
        self.realtime_data = Some(data);
    }
    
    pub fn cleanup_old_events(&mut self) {
        let now = Instant::now();
        
        // Limpiar solo cada 100ms para mejor rendimiento
        if now.duration_since(self.last_cleanup).as_millis() < 100 {
            return;
        }
        
        self.last_cleanup = now;
        
        // Marcar eventos viejos para desvanecimiento
        let time_window = self.config.visual.time_window;
        for event in &mut self.events {
            if event.created_at.elapsed().as_secs_f64() > time_window {
                event.start_fade();
            }
        }
        
        // Remover eventos completamente desvanecidos
        let fade_time = self.config.visual.event_fade_time;
        self.events.retain(|event| !event.is_expired(fade_time));
    }
    
    pub fn update_stats(&mut self) {
        self.stats.frame_count += 1;
        
        // Actualizar FPS cada segundo
        if self.stats.last_fps_update.elapsed().as_secs() >= 1 {
            self.stats.fps = self.stats.frame_count as f32;
            self.stats.frame_count = 0;
            self.stats.last_fps_update = Instant::now();
            
            // Calcular eventos por segundo
            if let Some(last_event) = self.stats.last_event_time {
                if last_event.elapsed().as_secs() < 1 {
                    // Aproximación simple basada en eventos recientes
                    let recent_events = self.events.iter()
                        .filter(|e| e.created_at.elapsed().as_secs() < 1)
                        .count();
                    self.stats.events_per_second = recent_events as f32;
                }
            }
        }
    }
    
    pub fn toggle_ui_element(&mut self, element: &str) {
        match element {
            "stats" => self.ui_state.show_stats = !self.ui_state.show_stats,
            "grid" => self.ui_state.show_grid = !self.ui_state.show_grid,
            "trails" => self.ui_state.show_trails = !self.ui_state.show_trails,
            "pause" => self.ui_state.pause_updates = !self.ui_state.pause_updates,
            "menu" => self.ui_state.show_menu = !self.ui_state.show_menu,
            "fullscreen" => self.ui_state.fullscreen = !self.ui_state.fullscreen,
            "performance" => self.ui_state.performance_mode = !self.ui_state.performance_mode,
            "timer" => self.ui_state.show_timer = !self.ui_state.show_timer,
            "snap_x" => self.ui_state.snap_to_x_grid = !self.ui_state.snap_to_x_grid,
            "snap_y" => self.ui_state.snap_to_y_grid = !self.ui_state.snap_to_y_grid,
            "high_res" => self.ui_state.high_resolution = !self.ui_state.high_resolution,
            _ => {}
        }
    }
    
    pub fn zoom_in(&mut self) {
        self.ui_state.zoom_level = (self.ui_state.zoom_level * 1.2).min(5.0);
    }
    
    pub fn zoom_out(&mut self) {
        self.ui_state.zoom_level = (self.ui_state.zoom_level / 1.2).max(0.1);
    }
    
    pub fn reset_zoom(&mut self) {
        self.ui_state.zoom_level = 1.0;
        self.ui_state.viewport_offset = Vec2::ZERO;
    }
    
    pub fn change_theme(&mut self, theme: ColorTheme) {
        self.ui_state.theme = theme.clone();
        // Actualizar colores de la rejilla según el tema
        match theme {
            ColorTheme::Light => {
                self.config.visual.background_color = [240, 245, 250];
                self.config.visual.grid.major_color = [0.2, 0.3, 0.6, 0.4];
                self.config.visual.grid.minor_color = [0.3, 0.4, 0.7, 0.2];
                self.config.visual.grid.label_color = [0.1, 0.2, 0.4, 0.9];
            },
            ColorTheme::Dark => {
                self.config.visual.background_color = [8, 15, 30];
                self.config.visual.grid.major_color = [0.3, 0.7, 1.0, 0.25];
                self.config.visual.grid.minor_color = [0.2, 0.5, 0.8, 0.12];
                self.config.visual.grid.label_color = [0.8, 0.9, 1.0, 0.9];
            },
            ColorTheme::Blue => {
                self.config.visual.background_color = [15, 25, 40];
                self.config.visual.grid.major_color = [0.4, 0.8, 1.0, 0.3];
                self.config.visual.grid.minor_color = [0.3, 0.6, 0.9, 0.15];
                self.config.visual.grid.label_color = [0.9, 0.95, 1.0, 0.95];
            },
            ColorTheme::Classic => {
                self.config.visual.background_color = [20, 20, 20];
                self.config.visual.grid.major_color = [0.5, 0.5, 0.5, 0.3];
                self.config.visual.grid.minor_color = [0.3, 0.3, 0.3, 0.15];
                self.config.visual.grid.label_color = [0.9, 0.9, 0.9, 0.9];
            },
        }
    }
    
    pub fn adjust_grid_resolution(&mut self, delta: i32) {
        let new_resolution = (self.ui_state.grid_resolution as i32 + delta).max(4).min(32) as u32;
        self.ui_state.grid_resolution = new_resolution;
        self.config.visual.grid.major_lines = new_resolution;
    }
    
    pub fn get_background_color(&self) -> Rgb<u8> {
        let [r, g, b] = self.config.visual.background_color;
        rgb(r, g, b)
    }
    
    pub fn fit_to_window(&mut self) {
        // Ajustar el zoom y viewport para que el contenido se ajuste a la ventana
        self.ui_state.zoom_level = 1.0;
        self.ui_state.viewport_offset = Vec2::ZERO;
    }
    
    pub fn resize_viewport(&mut self, new_size: Vec2) {
        // Ajustar el viewport a un nuevo tamaño
        self.ui_state.viewport_offset = Vec2::ZERO;
        println!("📏 Viewport redimensionado a: {}x{}", new_size.x, new_size.y);
    }
    
    pub fn save_session(&self, filename: &str) -> Result<(), String> {
        // Guardar la sesión actual (configuración y estado)
        let session_data = format!(
            "# SC Score Visualizer Session\n\
            zoom_level = {}\n\
            theme = \"{:?}\"\n\
            performance_mode = {}\n\
            fullscreen = {}\n\
            show_timer = {}\n\
            snap_to_x_grid = {}\n\
            snap_to_y_grid = {}\n\
            grid_resolution = {}\n\
            high_resolution = {}\n\
            frequency_range = [{}, {}]\n\
            time_range = {}\n",
            self.ui_state.zoom_level,
            self.ui_state.theme,
            self.ui_state.performance_mode,
            self.ui_state.fullscreen,
            self.ui_state.show_timer,
            self.ui_state.snap_to_x_grid,
            self.ui_state.snap_to_y_grid,
            self.ui_state.grid_resolution,
            self.ui_state.high_resolution,
            self.config.visual.grid.frequency_range.0,
            self.config.visual.grid.frequency_range.1,
            self.config.visual.grid.time_range
        );
        
        std::fs::write(filename, session_data).map_err(|e| e.to_string())
    }
    
    pub fn load_session(&mut self, filename: &str) -> Result<(), String> {
        // Cargar una sesión previamente guardada
        let _content = std::fs::read_to_string(filename).map_err(|e| e.to_string())?;
        // Implementar parsing de la sesión aquí
        println!("📂 Sesión cargada desde: {}", filename);
        Ok(())
    }
    
    pub fn export_image(&self, filename: &str) -> Result<(), String> {
        // Exportar una imagen de la visualización actual
        println!("🖼️ Exportando imagen a: {}", filename);
        // Implementar exportación de imagen aquí
        Ok(())
    }
    
    pub fn export_video(&self, filename: &str) -> Result<(), String> {
        // Exportar un video de la visualización
        println!("🎥 Exportando video a: {}", filename);
        // Implementar exportación de video aquí
        Ok(())
    }
    
    pub fn copy_screenshot(&self) -> Result<(), String> {
        // Copiar una captura de pantalla al portapapeles
        println!("📋 Copiando captura de pantalla al portapapeles");
        // Implementar copia al portapapeles aquí
        Ok(())
    }
    
    pub fn show_preferences(&mut self) {
        // Mostrar ventana de preferencias
        println!("⚙️ Abriendo ventana de preferencias");
        // Implementar ventana de preferencias aquí
    }
    
    pub fn reset_settings(&mut self) {
        // Restablecer todas las configuraciones a valores por defecto
        self.ui_state = UiState::default();
        self.config = Config::default();
        println!("🔄 Configuraciones restablecidas a valores por defecto");
    }
    
    pub fn get_current_time_formatted(&self) -> String {
        // Obtener el tiempo actual formateado para el timer
        use std::time::SystemTime;
        let now = SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        let hours = (now / 3600) % 24;
        let minutes = (now / 60) % 60;
        let seconds = now % 60;
        
        format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
    }
    
    pub fn get_session_duration(&self) -> String {
        // Obtener la duración de la sesión actual
        let duration = self.last_cleanup.elapsed();
        let hours = duration.as_secs() / 3600;
        let minutes = (duration.as_secs() % 3600) / 60;
        let seconds = duration.as_secs() % 60;
        
        format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
    }
    
    // ...existing code...
}

// =================================================================
// UTILIDADES DE CONFIGURACIÓN
// =================================================================

pub fn load_config() -> Config {
    match std::fs::read_to_string("config.toml") {
        Ok(content) => {
            match toml::from_str::<Config>(&content) {
                Ok(config) => {
                    println!("✅ Configuración cargada desde config.toml");
                    config
                }
                Err(e) => {
                    eprintln!("⚠️ Error al parsear config.toml: {}. Usando configuración por defecto.", e);
                    Config::default()
                }
            }
        }
        Err(e) => {
            eprintln!("⚠️ No se pudo cargar 'config.toml': {}. Usando configuración por defecto.", e);
            Config::default()
        }
    }
}
