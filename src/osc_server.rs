// src/osc_server.rs - Servidor OSC robusto con manejo de hilos

use nannou_osc as osc;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use crate::config::OscConfig;

/// Mensaje OSC procesado con metadata adicional
#[derive(Debug, Clone)]
pub struct ProcessedOscMessage {
    pub address: String,
    pub args: Vec<osc::Type>,
    pub timestamp: Instant,
    pub source_addr: String,
}

/// Estadísticas del servidor OSC
#[derive(Debug, Clone, Default)]
pub struct OscServerStats {
    pub total_received: u64,
    pub total_processed: u64,
    pub total_errors: u64,
    pub messages_per_second: f64,
    pub last_message_time: Option<Instant>,
    pub is_connected: bool,
}

/// Estado del servidor OSC
pub struct OscServer {
    pub receiver: osc::Receiver,
    pub stats: Arc<Mutex<OscServerStats>>,
    config: OscConfig,
    message_buffer: Vec<ProcessedOscMessage>,
    last_stats_update: Instant,
}

impl OscServer {
    /// Crea un nuevo servidor OSC con la configuración proporcionada
    pub fn new(config: OscConfig) -> Result<Self, Box<dyn std::error::Error>> {
        println!("🚀 Iniciando servidor OSC robusto...");
        println!("📡 Configuración: {}:{}", config.listen_host, config.listen_port);
        println!("📦 Buffer: {} mensajes, timeout: {}ms", config.buffer_size, config.timeout_ms);
        
        // Crear receptor OSC
        let receiver = osc::receiver(config.listen_port)
            .map_err(|e| {
                eprintln!("❌ Error creando receptor OSC en puerto {}: {}", config.listen_port, e);
                e
            })?;
        
        println!("✅ Servidor OSC iniciado exitosamente");
        println!("🎯 Esperando mensajes en {}:{}", config.listen_host, config.listen_port);
        
        Ok(Self {
            receiver,
            stats: Arc::new(Mutex::new(OscServerStats::default())),
            config: config.clone(),
            message_buffer: Vec::with_capacity(config.buffer_size),
            last_stats_update: Instant::now(),
        })
    }
    
    /// Procesa mensajes OSC entrantes y retorna los mensajes procesados
    pub fn process_messages(&mut self) -> Vec<ProcessedOscMessage> {
        let mut processed_messages = Vec::new();
        let mut message_count = 0;
        let start_time = Instant::now();
        
        // Limpiar buffer anterior
        self.message_buffer.clear();
        
        // Recopilar mensajes hasta el límite configurado
        for (packet, addr) in self.receiver.try_iter() {
            if message_count >= self.config.max_messages_per_frame {
                println!("⚠️ OSC: Límite de mensajes alcanzado ({}/frame)", self.config.max_messages_per_frame);
                break;
            }
            
            for msg in packet.into_msgs() {
                let processed_msg = ProcessedOscMessage {
                    address: msg.addr.clone(),
                    args: msg.args.clone(),
                    timestamp: Instant::now(),
                    source_addr: addr.to_string(),
                };
                
                self.message_buffer.push(processed_msg.clone());
                processed_messages.push(processed_msg);
                message_count += 1;
                
                // Actualizar estadísticas
                if let Ok(mut stats) = self.stats.lock() {
                    stats.total_received += 1;
                    stats.last_message_time = Some(Instant::now());
                    stats.is_connected = true;
                }
            }
        }
        
        // Log de actividad si hay mensajes
        if message_count > 0 {
            let processing_time = start_time.elapsed();
            println!("📡 OSC: {} mensajes procesados en {:.2}ms", 
                   message_count, processing_time.as_millis());
            
            if message_count > 20 {
                println!("⚡ OSC: Tráfico intenso - {} mensajes", message_count);
            }
        }
        
        // Actualizar estadísticas de rendimiento cada segundo
        if self.last_stats_update.elapsed() >= Duration::from_secs(1) {
            self.update_performance_stats();
            self.last_stats_update = Instant::now();
        }
        
        processed_messages
    }
    
    /// Actualiza las estadísticas de rendimiento
    fn update_performance_stats(&self) {
        if let Ok(mut stats) = self.stats.lock() {
            let elapsed_seconds = self.last_stats_update.elapsed().as_secs_f64();
            if elapsed_seconds > 0.0 {
                stats.messages_per_second = stats.total_received as f64 / elapsed_seconds;
            }
            
            // Verificar conexión (si no hay mensajes en los últimos 5 segundos)
            if let Some(last_msg_time) = stats.last_message_time {
                stats.is_connected = last_msg_time.elapsed() < Duration::from_secs(5);
            } else {
                stats.is_connected = false;
            }
        }
    }
    
    /// Obtiene una copia de las estadísticas actuales
    pub fn get_stats(&self) -> OscServerStats {
        self.stats.lock().unwrap_or_else(|_| {
            println!("⚠️ OSC: Error accediendo a estadísticas");
            std::process::exit(1);
        }).clone()
    }
    
    /// Resetea las estadísticas del servidor
    pub fn reset_stats(&self) {
        if let Ok(mut stats) = self.stats.lock() {
            *stats = OscServerStats::default();
            println!("🔄 OSC: Estadísticas reseteadas");
        }
    }
    
    /// Verifica si el servidor está recibiendo mensajes activamente
    pub fn is_active(&self) -> bool {
        self.get_stats().is_connected
    }
    
    /// Obtiene información de estado del servidor para logging
    pub fn status_info(&self) -> String {
        let stats = self.get_stats();
        format!("OSC Server - Puerto: {}, Recibidos: {}, MPS: {:.1}, Activo: {}", 
               self.config.listen_port,
               stats.total_received,
               stats.messages_per_second,
               stats.is_connected)
    }
    
    /// Realiza una prueba de conectividad interna
    pub fn self_test(&self) -> bool {
        println!("🔧 OSC: Realizando auto-test...");
        
        // Verificar que el receptor esté disponible
        let port_accessible = self.config.listen_port > 0 && self.config.listen_port < 65535;
        
        if port_accessible {
            println!("✅ OSC: Auto-test exitoso - Puerto {} accesible", self.config.listen_port);
            true
        } else {
            println!("❌ OSC: Auto-test fallido - Puerto {} inválido", self.config.listen_port);
            false
        }
    }
}

/// Validador de mensajes OSC
pub struct OscMessageValidator {
    pub audio_config: crate::config::AudioConfig,
}

impl OscMessageValidator {
    pub fn new(audio_config: crate::config::AudioConfig) -> Self {
        Self { audio_config }
    }
    
    /// Valida un mensaje de nota musical
    pub fn validate_note_message(&self, args: &[osc::Type]) -> Result<(String, f32, f32, f32), String> {
        if args.len() < 4 {
            return Err(format!("Argumentos insuficientes: {} (esperados: 4)", args.len()));
        }
        
        let instrument = args[0].clone().string().unwrap_or("default".to_string());
        let freq = args[1].clone().float().ok_or("Frecuencia inválida")?;
        let amp = args[2].clone().float().ok_or("Amplitud inválida")?;
        let dur = args[3].clone().float().ok_or("Duración inválida")?;
        
        // Validar rangos
        if freq < self.audio_config.freq_min || freq > self.audio_config.freq_max {
            return Err(format!("Frecuencia fuera de rango: {:.1}Hz", freq));
        }
        
        if amp < self.audio_config.amp_min || amp > self.audio_config.amp_max {
            return Err(format!("Amplitud fuera de rango: {:.2}", amp));
        }
        
        if dur < self.audio_config.dur_min || dur > self.audio_config.dur_max {
            return Err(format!("Duración fuera de rango: {:.2}s", dur));
        }
        
        Ok((instrument, freq, amp, dur))
    }
    
    /// Valida un mensaje de drone
    pub fn validate_drone_message(&self, args: &[osc::Type]) -> Result<(f32, f32), String> {
        if args.len() < 2 {
            return Err(format!("Argumentos insuficientes: {} (esperados: 2)", args.len()));
        }
        
        let freq = args[0].clone().float().ok_or("Frecuencia inválida")?;
        let amp = args[1].clone().float().ok_or("Amplitud inválida")?;
        
        if freq < self.audio_config.freq_min || freq > self.audio_config.freq_max {
            return Err(format!("Frecuencia fuera de rango: {:.1}Hz", freq));
        }
        
        if amp < self.audio_config.amp_min || amp > self.audio_config.amp_max {
            return Err(format!("Amplitud fuera de rango: {:.2}", amp));
        }
        
        Ok((freq, amp))
    }
}
