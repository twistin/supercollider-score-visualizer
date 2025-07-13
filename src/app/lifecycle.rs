// 🔄 Ciclo de vida de la aplicación
// Maneja la inicialización, actualización y renderizado

use nannou::prelude::*;
use crate::app::AppState;
use crate::audio::OscHandler;
use crate::visual::{OptimizedRenderer, ShaderManager, render_frame};
use crate::config::AppConfig;
use crate::utils::{VisualizerError, VisualizerResult};
use tracing::{info, warn, error, debug};
use std::sync::Arc;
use tokio::sync::RwLock;




/// Modelo principal de la aplicación (compatible con Nannou)
pub struct Model {
    pub state: AppState,
    pub renderer: OptimizedRenderer,
    pub enable_optimization: bool,
    pub config: AppConfig,
    pub shader_manager: Arc<RwLock<Option<ShaderManager>>>,
}

impl Model {
    /// Crea un nuevo modelo con configuración
    pub fn new_with_config(config: AppConfig) -> Self {
        Self {
            state: AppState::new_with_config(config.clone()),
            renderer: OptimizedRenderer::new(),
            enable_optimization: config.performance.batching_enabled,
            config,
            shader_manager: Arc::new(RwLock::new(None)),
        }
    }
    
    /// Crea un nuevo modelo con configuración por defecto
    pub fn new() -> Self {
        Self::new_with_config(AppConfig::default())
    }
    
    /// Inicializa el gestor de shaders de forma asíncrona
    pub async fn init_shader_manager(&self) -> VisualizerResult<()> {
        info!("🎨 Inicializando gestor de shaders con hot-reload...");
        
        let shaders_dir = std::path::Path::new("src/visual/shaders");
        if !shaders_dir.exists() {
            warn!("⚠️ Directorio de shaders no existe, creándolo: {:?}", shaders_dir);
            std::fs::create_dir_all(shaders_dir)
                .map_err(|e| VisualizerError::config(format!("Error creando directorio de shaders: {}", e)))?;
        }
        
        match ShaderManager::new(shaders_dir).await {
            Ok(manager) => {
                let mut shader_manager = self.shader_manager.write().await;
                *shader_manager = Some(manager);
                info!("✅ Gestor de shaders inicializado correctamente");
                Ok(())
            }
            Err(e) => {
                error!("❌ Error inicializando gestor de shaders: {}", e);
                Err(VisualizerError::config(format!("Error inicializando gestor de shaders: {}", e)))
            }
        }
    }
    
    /// Obtiene el gestor de shaders si está disponible
    pub async fn get_shader_manager(&self) -> Option<Arc<RwLock<ShaderManager>>> {
        let manager_guard = self.shader_manager.read().await;
        manager_guard.as_ref().map(|manager| Arc::new(RwLock::new(manager.clone())))
    }
}

impl Default for Model {
    fn default() -> Self {
        Self::new()
    }
}

/// Inicializa la aplicación con configuración avanzada
pub fn initialize_app(app: &App, config: AppConfig) -> anyhow::Result<Model> {
    info!("🚀 Inicializando aplicación con configuración avanzada...");
    
    // Configurar ventana usando parámetros de configuración
    info!("🖼️ Configurando ventana {}x{}", config.visual.window_width, config.visual.window_height);
    let mut window_builder = app.new_window()
        .size(config.visual.window_width, config.visual.window_height)
        .title(&config.visual.window_title)
        .view(render_frame)
        .key_pressed(handle_key_press);
    
    // Aplicar configuración de pantalla completa si está habilitada
    if config.visual.fullscreen {
        info!("🖥️ Configurando modo pantalla completa");
        window_builder = window_builder.fullscreen();
    }
    
    // Configurar vsync
    if !config.visual.vsync {
        info!("🔄 VSync deshabilitado");
        // Nota: Nannou no expone control directo de VSync, pero se puede configurar en el futuro
    }
    
    window_builder.build()
        .map_err(|e| VisualizerError::window(format!("Error creando ventana: {}", e)))?;

    let model = Model::new_with_config(config.clone());
    
    // Configurar sincronización temporal según configuración
    if !config.performance.time_sync_enabled {
        model.renderer.toggle_time_sync();
        info!("⏱️ Sincronización temporal deshabilitada por configuración");
    }
    
    // Configurar renderer según configuración
    if config.visual.enable_debug_info {
        model.renderer.toggle_debug_info();
        info!("🔍 Información de debug habilitada por configuración");
    }
    
    // Configurar tiempo de sincronización
    model.renderer.set_sync_start_time(app.time);
    info!("⏱️ Sincronización temporal configurada desde: {:.2}s", app.time);
    
    // Inicializar el receptor OSC con configuración
    info!("🔊 Inicializando receptor OSC en {}:{}", config.audio.osc_host, config.audio.osc_port);
    let osc_handler = OscHandler::new(config.audio.osc_port);
    let events_handle = model.state.get_events_handle();
    osc_handler.start_receiver(events_handle)?;

    // Inicializar gestor de shaders con hot-reload si está habilitado
    if config.visual.shader_hot_reload {
        info!("🔥 Configurando hot-reload de shaders...");
        let shader_manager_arc = Arc::clone(&model.shader_manager);
        
        tokio::spawn(async move {
            let mut sm_guard = shader_manager_arc.write().await;
            *sm_guard = Some(ShaderManager::new("src/visual/shaders").await
                .expect("Error inicializando ShaderManager"));
            info!("✅ Gestor de shaders inicializado y hot-reload activo.");
        });
    } else {
        info!("⚠️ Hot-reload de shaders deshabilitado por configuración");
    }

    info!("✅ Aplicación inicializada correctamente con configuración avanzada");
    info!("📊 Configuración activa: batching={}, time_sync={}, max_particles={}", 
          config.performance.batching_enabled,
          config.performance.time_sync_enabled,
          config.performance.max_particles);
    
    Ok(model)
}

/// Actualiza el estado de la aplicación
pub fn update_app(app: &App, model: &mut Model, _update: Update) -> anyhow::Result<()> {
    // Actualizar tiempo y eventos
    model.state.update_time(app.time);
    model.state.update_events()?;
    
    // Log periódico del estado (cada 5 segundos)
    if (app.time as u64) % 5 == 0 && app.time.fract() < 0.01 {
        debug!("⏱️ Estado: tiempo={:.2}s, eventos_activos={:?}", 
               app.time, model.state.get_active_events_count());
    }
    
    Ok(())
}



/// Maneja las teclas presionadas
pub fn handle_key_press(app: &App, model: &mut Model, key: Key) {
    match key {
        Key::S => {
            info!("📸 Exportando visualización...");
            match export_visualization(app) {
                Ok(path) => {
                    info!("✅ Visualización guardada como: {:?}", path);
                }
                Err(e) => {
                    error!("❌ Error exportando visualización: {}", e);
                }
            }
        },
        Key::D => {
            info!("🔍 Alternando información de debug...");
            model.renderer.toggle_debug_info();
        },
        Key::O => {
            info!("⚡ Alternando optimización de batching...");
            model.enable_optimization = !model.enable_optimization;
            info!("🎯 Optimización: {}", if model.enable_optimization { "ON" } else { "OFF" });
        },
        Key::P => {
            info!("📊 Estadísticas de rendimiento:");
            let stats = model.renderer.get_stats();
            let batch_stats = model.renderer.get_batch_stats();
            info!("   Frames: {}, Elementos: {}, FPS: {:.1}", 
                 stats.frames_rendered, batch_stats.total_elements, stats.avg_fps);
            info!("   Sincronización: Activos: {}/{}", 
                 stats.sync_active_events, stats.sync_total_events);
        },
        Key::T => {
            info!("⏱️ Alternando sincronización temporal...");
            model.renderer.toggle_time_sync();
        },
        Key::R => {
            info!("🔄 Reiniciando tiempo de sincronización...");
            model.renderer.set_sync_start_time(app.time);
        },
        Key::C => {
            info!("💾 Guardando configuración actual...");
            let config_path = "config/current.toml";
            match model.config.save_to_file(config_path) {
                Ok(_) => {
                    info!("✅ Configuración guardada en: {}", config_path);
                }
                Err(e) => {
                    error!("❌ Error guardando configuración: {}", e);
                }
            }
        },
        Key::H => {
            info!("🔥 Recompilando shaders...");
            let shader_manager = Arc::clone(&model.shader_manager);
            tokio::spawn(async move {
                if let Some(manager) = shader_manager.read().await.as_ref() {
                    match manager.force_recompile_all().await {
                        Ok(_) => {
                            info!("✅ Shaders recompilados exitosamente");
                        }
                        Err(e) => {
                            error!("❌ Error recompilando shaders: {}", e);
                        }
                    }
                } else {
                    warn!("⚠️ Gestor de shaders no está disponible");
                }
            });
        },
        Key::L => {
            info!("📋 Listando shaders disponibles...");
            let shader_manager = Arc::clone(&model.shader_manager);
            tokio::spawn(async move {
                if let Some(manager) = shader_manager.read().await.as_ref() {
                    let shaders = manager.list_shaders().await;
                    let (total, success, errors) = manager.get_compilation_stats().await;
                    
                    info!("📊 Estadísticas de shaders: {} total, {} exitosos, {} con errores", total, success, errors);
                    for shader in shaders {
                        let status = match shader.compile_status {
                            crate::visual::CompileStatus::Success => "✅",
                            crate::visual::CompileStatus::Error(_) => "❌",
                            crate::visual::CompileStatus::Pending => "⏳",
                        };
                        info!("   {} {} ({})", status, shader.name, shader.shader_type.to_string());
                    }
                } else {
                    warn!("⚠️ Gestor de shaders no está disponible");
                }
            });
        },
        _ => {}
    }
}

/// Exporta la visualización actual
fn export_visualization(app: &App) -> VisualizerResult<std::path::PathBuf> {
    debug!("🔧 Preparando exportación de visualización");
    
    let window = app.main_window();
    let path = app.assets_path()
        .unwrap_or_else(|_| std::env::current_dir().unwrap())
        .join(format!("visualizer_export_{}.png", app.time as u32));
    
    debug!("💾 Capturando frame en: {:?}", path);
    window.capture_frame(&path);
    
    Ok(path)
}


