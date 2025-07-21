// 🔄 Ciclo de vida de la aplicación
// Maneja la inicialización, actualización y renderizado

use nannou::prelude::*;
use crate::app::state::AppState; // Ruta corregida a AppState
use crate::visual::{OptimizedRenderer, ShaderManager, render_frame};
use crate::config::AppConfig;
use crate::errors::{VisualizerError, VisualizerResult, init_error, config_error, window_error, sync_error}; // Importar tipos de error y macros
use crate::model::Model; // Usar el Model principal de model.rs
use crate::{log_system, log_visual, log_performance, log_osc}; // Importar macros de logging
use std::sync::Arc;
use tokio::sync::RwLock; // Mantener tokio para ShaderManager asíncrono

// La definición de la estructura `Model` se ha eliminado de aquí, ya que está definida en `crate::model`.

/// Inicializa la aplicación con configuración avanzada
pub async fn initialize_app(app: &App, config: AppConfig) -> anyhow::Result<Model> { // Hecho asíncrono
    log_system!(Info, "🚀 Inicializando aplicación con configuración avanzada...");
    
    // Configurar ventana usando parámetros de configuración
    log_system!(Info, "🖼️ Configurando ventana {}x{}", config.window.width, config.window.height); // Usar config.window
    let mut window_builder = app.new_window()
        .size(config.window.width, config.window.height)
        .title(&config.window.title)
        .view(render_frame)
        .key_pressed(handle_key_press);
    
    // Aplicar configuración de pantalla completa si está habilitada
    if config.visual.fullscreen { // Usar config.visual
        log_system!(Info, "🖥️ Configurando modo pantalla completa");
        window_builder = window_builder.fullscreen();
    }
    
    // Configurar vsync (Nannou no expone control directo, pero se mantiene el log)
    if !config.window.vsync { // Usar config.window
        log_system!(Info, "🔄 VSync deshabilitado (configuración)");
    }
    
    window_builder.build()
        .map_err(|e| window_error!(format!("Error creando ventana: {}", e)))?; // Usar macro window_error

    // Inicializar el Model principal de model.rs
    // El OscServer es ahora parte del Model, así que lo creamos primero.
    let osc_server = crate::osc_server::OscServer::new(config.osc.clone())
        .map_err(|e| init_error!("OSC Server", format!("Error inicializando servidor OSC: {}", e)))?;
    
    let mut model = Model::new_with_config(osc_server, config.clone()); // Pasar config al Model
    
    // Inicializar servidor MIDI dentro del Model
    model.init_midi();
    log_system!(Info, "⏱️ Sincronización temporal configurada desde: {:.2}s", app.time);
    
    // Inicializar gestor de shaders con hot-reload si está habilitado
    if model.config.visual.shader_hot_reload { // Usar la config del modelo
        log_system!(Info, "🔥 Configurando hot-reload de shaders...");
        
        // Llamar al método asíncrono de inicialización en el propio Model
        model.init_shader_manager().await
            .map_err(|e| init_error!("Shader Manager", format!("Error inicializando gestor de shaders: {}", e)))?;
        
        log_system!(Info, "✅ Gestor de shaders inicializado y hot-reload activo.");
    } else {
        log_system!(Warning, "⚠️ Hot-reload de shaders deshabilitado por configuración");
    }

    log_system!(Info, "✅ Aplicación inicializada correctamente con configuración avanzada");
    log_system!(Info, "📊 Configuración activa: max_notes={}, max_drones={}, max_cluster_particles={}", // Usar campos de configuración relevantes
          config.performance.max_notes,
          config.performance.max_drones,
          config.performance.max_cluster_particles);
    
    Ok(model)
}

/// Actualiza el estado de la aplicación
pub fn update_app(app: &App, model: &mut Model, update: Update) -> anyhow::Result<()> {
    // Actualizar tiempo y eventos usando los métodos del Model principal
    model.update_time(app.time); // Esto actualiza model.elapsed_time
    model.update_events(update.since_last.as_secs_f32()); // Esto actualiza notas/drones legacy y limpia
    model.update_visual_notes(update.since_last.as_secs_f32(), app.window_rect()); // Actualizar notas visuales profesionales

    // Log periódico del estado (cada 5 segundos)
    if (app.time as u64) % 5 == 0 && app.time.fract() < 0.01 {
        log_system!(Debug, "⏱️ Estado: tiempo={:.2}s, eventos_activos_legacy={:?}", 
               app.time, model.get_active_events_count()); // Usar el método del modelo
    }
    
    Ok(())
}

/// Maneja las teclas presionadas
pub fn handle_key_press(app: &App, model: &mut Model, key: Key) {
    match key {
        Key::S => {
            log_system!(Info, "📸 Exportando visualización...");
            match export_visualization(app) {
                Ok(path) => {
                    log_system!(Info, "✅ Visualización guardada como: {:?}", path);
                }
                Err(e) => {
                    log_system!(Error, "❌ Error exportando visualización: {}", e);
                }
            }
        },
        Key::D => {
            log_system!(Info, "🔍 Alternando información de debug...");
            model.display_config.show_debug = !model.display_config.show_debug; // Actualizar directamente la config de display
            log_system!(Info, "🐛 Debug info: {}", if model.display_config.show_debug { "ON" } else { "OFF" });
        },
        Key::O => {
            log_system!(Info, "⚡ Alternando optimización de batching...");
            model.config.performance.batching_enabled = !model.config.performance.batching_enabled; // Actualizar config directamente
            log_system!(Info, "🎯 Optimización: {}", if model.config.performance.batching_enabled { "ON" } else { "OFF" });
        },
        Key::P => {
            log_system!(Info, "📊 Estadísticas de rendimiento:");
            // Obtener estadísticas de los componentes del Modelo
            let osc_stats = model.osc_server.get_stats();
            log_performance!(Info, "📡 OSC: Total={}, Procesados={}, Errores={}, MPS={:.1}, Conectado={}",
                             osc_stats.total_received, osc_stats.total_processed,
                             osc_stats.total_errors, osc_stats.messages_per_second,
                             osc_stats.is_connected);
            log_performance!(Info, "🎵 Eventos Legacy: {} | Drones: {} | Notas Visuales: {}",
                             model.notes.len(), model.drone_events.len(), model.visual_notes.len());
        },
        Key::T => {
            log_system!(Info, "⏱️ Alternando sincronización temporal...");
            // Esta funcionalidad necesita ser implementada en model.rs si aún se desea.
            log_system!(Warning, "⚠️ Sincronización temporal no implementada en el modelo actual.");
        },
        Key::R => {
            log_system!(Info, "🔄 Reiniciando tiempo y eventos...");
            model.elapsed_time = 0.0; // Resetear tiempo transcurrido
            model.osc_server.reset_stats(); // Resetear estadísticas OSC
            model.clear_events(); // Limpiar todos los eventos (legacy y visuales)
            log_system!(Info, "✅ Tiempo y eventos reiniciados.");
        },
        Key::C => {
            log_system!(Info, "💾 Guardando configuración actual...");
            let config_path = "config.toml"; // Usar el config.toml estándar
            match model.config.save_to_file(config_path) {
                Ok(_) => {
                    log_system!(Info, "✅ Configuración guardada en: {}", config_path);
                }
                Err(e) => {
                    log_system!(Error, "❌ Error guardando configuración: {}", e);
                }
            }
        },
        Key::H => {
            log_system!(Info, "🔥 Recompilando shaders...");
            let shader_manager_arc = Arc::clone(&model.shader_manager);
            // Usar std::thread::spawn y block_on para llamadas asíncronas en un contexto síncrono
            std::thread::spawn(move || {
                let rt = tokio::runtime::Runtime::new().unwrap();
                rt.block_on(async {
                    if let Some(manager) = shader_manager_arc.read().await.as_ref() {
                        match manager.force_recompile_all().await {
                            Ok(_) => {
                                log_system!(Info, "✅ Shaders recompilados exitosamente");
                            }
                            Err(e) => {
                                log_system!(Error, "❌ Error recompilando shaders: {}", e);
                            }
                        }
                    } else {
                        log_system!(Warning, "⚠️ Gestor de shaders no está disponible");
                    }
                });
            });
        },
        Key::L => {
            log_system!(Info, "📋 Listando shaders disponibles...");
            let shader_manager_arc = Arc::clone(&model.shader_manager);
            std::thread::spawn(move || {
                let rt = tokio::runtime::Runtime::new().unwrap();
                rt.block_on(async {
                    if let Some(manager) = shader_manager_arc.read().await.as_ref() {
                        let shaders = manager.list_shaders().await;
                        let (total, success, errors) = manager.get_compilation_stats().await;
                        
                        log_system!(Info, "📊 Estadísticas de shaders: {} total, {} exitosos, {} con errores", total, success, errors);
                        for shader in shaders {
                            let status = match shader.compile_status {
                                crate::visual::CompileStatus::Success => "✅",
                                crate::visual::CompileStatus::Error(_) => "❌",
                                crate::visual::CompileStatus::Pending => "⏳",
                            };
                            log_system!(Info, "   {} {} ({})", status, shader.name, shader.shader_type.to_string());
                        }
                    } else {
                        log_system!(Warning, "⚠️ Gestor de shaders no está disponible");
                    }
                });
            });
        },
        _ => {}
    }
}

/// Exporta la visualización actual
fn export_visualization(app: &App) -> VisualizerResult<std::path::PathBuf> {
    log_system!(Debug, "🔧 Preparando exportación de visualización");
    
    let window = app.main_window();
    let path = app.assets_path()
        .unwrap_or_else(|_| std::env::current_dir().unwrap())
        .join(format!("visualizer_export_{}.png", app.time as u32));
    
    log_system!(Debug, "💾 Capturando frame en: {:?}", path);
    window.capture_frame(&path);
    
    Ok(path)
}
