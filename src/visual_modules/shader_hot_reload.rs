// 🔥 Sistema de Hot-Reload para Shaders
// Supervisa cambios en archivos .wgsl y recarga automáticamente

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::sync::RwLock;
use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher, Event, EventKind};
use tracing::{info, warn, error, debug};
use anyhow::{Result, Context};

pub type ShaderContent = String;
pub type ShaderName = String;

/// Gestor de hot-reload para shaders
pub struct ShaderHotReloader {
    shader_cache: Arc<RwLock<HashMap<ShaderName, ShaderContent>>>,
    watcher: Option<RecommendedWatcher>,
    shaders_dir: PathBuf,
}

impl Clone for ShaderHotReloader {
    fn clone(&self) -> Self {
        Self {
            shader_cache: Arc::clone(&self.shader_cache),
            watcher: None, // El watcher no se puede clonar, se inicializa en None
            shaders_dir: self.shaders_dir.clone(),
        }
    }
}

impl ShaderHotReloader {
    /// Crea un nuevo gestor de hot-reload
    pub fn new<P: AsRef<Path>>(shaders_dir: P) -> Result<Self> {
        let shaders_dir = shaders_dir.as_ref().to_path_buf();
        
        // Verificar que el directorio existe
        if !shaders_dir.exists() {
            return Err(anyhow::anyhow!(
                "Directorio de shaders no existe: {:?}", 
                shaders_dir
            ));
        }
        
        info!("🔥 Inicializando hot-reload de shaders en: {:?}", shaders_dir);
        
        let shader_cache = Arc::new(RwLock::new(HashMap::new()));
        
        Ok(Self {
            shader_cache,
            watcher: None,
            shaders_dir,
        })
    }
    
    /// Inicia el sistema de supervisión de archivos
    pub async fn start_watching(&mut self) -> Result<()> {
        info!("👀 Iniciando supervisión de archivos de shaders...");
        
        // Cargar shaders existentes
        self.load_all_shaders().await?;
        
        // Configurar el watcher
        let shader_cache = Arc::clone(&self.shader_cache);
        let shaders_dir = self.shaders_dir.clone();
        
        let mut watcher = RecommendedWatcher::new(
            move |res: Result<Event, notify::Error>| {
                let shader_cache = Arc::clone(&shader_cache);
                let shaders_dir = shaders_dir.clone();
                
                // Usar spawn_blocking para evitar problemas de Send
                std::thread::spawn(move || {
                    let rt = tokio::runtime::Runtime::new().unwrap();
                    rt.block_on(async {
                        match res {
                            Ok(event) => {
                                if let Err(e) = Self::handle_file_event(event, shader_cache, shaders_dir).await {
                                    error!("❌ Error manejando evento de archivo: {}", e);
                                }
                            }
                            Err(e) => {
                                error!("❌ Error en el watcher: {}", e);
                            }
                        }
                    });
                });
            },
            Config::default(),
        ).context("Error creando watcher")?;
        
        // Supervisar el directorio
        watcher.watch(&self.shaders_dir, RecursiveMode::Recursive)
            .context("Error configurando supervisión de directorio")?;
        
        self.watcher = Some(watcher);
        
        info!("✅ Sistema de hot-reload iniciado correctamente");
        Ok(())
    }
    
    /// Carga todos los shaders del directorio
    async fn load_all_shaders(&self) -> Result<()> {
        info!("📂 Cargando shaders existentes...");
        
        let mut count = 0;
        let entries = std::fs::read_dir(&self.shaders_dir)
            .context("Error leyendo directorio de shaders")?;
        
        for entry in entries {
            let entry = entry.context("Error procesando entrada del directorio")?;
            let path = entry.path();
            
            if path.is_file() && path.extension().map_or(false, |ext| ext == "wgsl") {
                if let Err(e) = self.load_shader_file(&path).await {
                    warn!("⚠️ Error cargando shader {:?}: {}", path, e);
                } else {
                    count += 1;
                }
            }
        }
        
        info!("✅ Cargados {} shaders", count);
        Ok(())
    }
    
    /// Carga un archivo de shader específico
    async fn load_shader_file(&self, path: &Path) -> Result<()> {
        let content = tokio::fs::read_to_string(path).await
            .with_context(|| format!("Error leyendo archivo: {:?}", path))?;
        
        let shader_name = path.file_stem()
            .and_then(|s| s.to_str())
            .ok_or_else(|| anyhow::anyhow!("Nombre de archivo inválido: {:?}", path))?
            .to_string();
        
        let mut cache = self.shader_cache.write().await;
        cache.insert(shader_name.clone(), content);
        
        debug!("📄 Shader cargado: {}", shader_name);
        Ok(())
    }
    
    /// Maneja eventos de cambio de archivos
    async fn handle_file_event(
        event: Event,
        shader_cache: Arc<RwLock<HashMap<ShaderName, ShaderContent>>>,
        _shaders_dir: PathBuf,
    ) -> Result<()> {
        match event.kind {
            EventKind::Create(_) | EventKind::Modify(_) => {
                for path in event.paths {
                    if path.is_file() && path.extension().map_or(false, |ext| ext == "wgsl") {
                        Self::reload_shader(&path, &shader_cache).await?;
                    }
                }
            }
            EventKind::Remove(_) => {
                for path in event.paths {
                    if path.extension().map_or(false, |ext| ext == "wgsl") {
                        Self::remove_shader(&path, &shader_cache).await?;
                    }
                }
            }
            _ => {}
        }
        
        Ok(())
    }
    
    /// Recarga un shader específico
    async fn reload_shader(
        path: &Path,
        shader_cache: &Arc<RwLock<HashMap<ShaderName, ShaderContent>>>,
    ) -> Result<()> {
        let content = tokio::fs::read_to_string(path).await
            .with_context(|| format!("Error leyendo archivo: {:?}", path))?;
        
        let shader_name = path.file_stem()
            .and_then(|s| s.to_str())
            .ok_or_else(|| anyhow::anyhow!("Nombre de archivo inválido: {:?}", path))?
            .to_string();
        
        // Validar sintaxis básica del shader
        if let Err(e) = Self::validate_shader_syntax(&content) {
            error!("❌ Error de sintaxis en shader {}: {}", shader_name, e);
            return Err(e);
        }
        
        let mut cache = shader_cache.write().await;
        cache.insert(shader_name.clone(), content);
        
        info!("🔄 Shader recargado: {}", shader_name);
        Ok(())
    }
    
    /// Elimina un shader del cache
    async fn remove_shader(
        path: &Path,
        shader_cache: &Arc<RwLock<HashMap<ShaderName, ShaderContent>>>,
    ) -> Result<()> {
        let shader_name = path.file_stem()
            .and_then(|s| s.to_str())
            .ok_or_else(|| anyhow::anyhow!("Nombre de archivo inválido: {:?}", path))?
            .to_string();
        
        let mut cache = shader_cache.write().await;
        cache.remove(&shader_name);
        
        info!("🗑️ Shader eliminado: {}", shader_name);
        Ok(())
    }
    
    /// Validación básica de sintaxis WGSL
    fn validate_shader_syntax(content: &str) -> Result<()> {
        // Verificaciones básicas de sintaxis WGSL
        let required_patterns = [
            ("@vertex", "Falta función vertex"),
            ("@fragment", "Falta función fragment"),
        ];
        
        for (pattern, error_msg) in required_patterns {
            if !content.contains(pattern) {
                return Err(anyhow::anyhow!("{}: {}", error_msg, pattern));
            }
        }
        
        // Verificar que las llaves estén balanceadas
        let open_braces = content.matches('{').count();
        let close_braces = content.matches('}').count();
        if open_braces != close_braces {
            return Err(anyhow::anyhow!(
                "Llaves no balanceadas: {} abiertas, {} cerradas",
                open_braces, close_braces
            ));
        }
        
        Ok(())
    }
    
    /// Obtiene el contenido de un shader
    pub async fn get_shader(&self, name: &str) -> Option<ShaderContent> {
        let cache = self.shader_cache.read().await;
        cache.get(name).cloned()
    }
    
    /// Obtiene todos los shaders disponibles
    pub async fn get_all_shaders(&self) -> HashMap<ShaderName, ShaderContent> {
        let cache = self.shader_cache.read().await;
        cache.clone()
    }
    
    /// Lista los nombres de todos los shaders cargados
    pub async fn list_shader_names(&self) -> Vec<ShaderName> {
        let cache = self.shader_cache.read().await;
        cache.keys().cloned().collect()
    }
    
    /// Verifica si un shader existe
    pub async fn has_shader(&self, name: &str) -> bool {
        let cache = self.shader_cache.read().await;
        cache.contains_key(name)
    }
}

impl Drop for ShaderHotReloader {
    fn drop(&mut self) {
        info!("🔥 Deteniendo sistema de hot-reload de shaders");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use tokio::fs;
    
    #[tokio::test]
    async fn test_shader_hot_reload() {
        let temp_dir = tempdir().unwrap();
        let shader_path = temp_dir.path().join("test.wgsl");
        
        // Crear shader inicial
        fs::write(&shader_path, "@vertex fn vs_main() {}").await.unwrap();
        
        let mut reloader = ShaderHotReloader::new(temp_dir.path()).unwrap();
        reloader.start_watching().await.unwrap();
        
        // Verificar que el shader se cargó
        assert!(reloader.has_shader("test").await);
        
        // Modificar el shader
        fs::write(&shader_path, "@vertex fn vs_main() {} @fragment fn fs_main() {}").await.unwrap();
        
        // Esperar un poco para que se procese el cambio
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        
        // Verificar que el contenido se actualizó
        let content = reloader.get_shader("test").await.unwrap();
        assert!(content.contains("@fragment"));
    }
}
