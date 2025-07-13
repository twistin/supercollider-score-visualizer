// 🎨 Gestor de Shaders con Hot-Reload
// Integra el sistema de hot-reload con el renderizado

use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;
use tokio::sync::RwLock;
use anyhow::{Result, Context};
use tracing::{info, warn, error, debug};

use super::shader_hot_reload::{ShaderHotReloader, ShaderContent, ShaderName};

/// Tipos de shaders disponibles
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ShaderType {
    Vertex,
    Fragment,
    Compute,
    Particles,
    PostProcess,
}

impl ShaderType {
    pub fn from_name(name: &str) -> Option<Self> {
        match name.to_lowercase().as_str() {
            "vertex" => Some(Self::Vertex),
            "fragment" => Some(Self::Fragment),
            "compute" => Some(Self::Compute),
            "particles" => Some(Self::Particles),
            "post_process" | "postprocess" => Some(Self::PostProcess),
            _ => None,
        }
    }
    
    pub fn to_string(&self) -> &'static str {
        match self {
            Self::Vertex => "vertex",
            Self::Fragment => "fragment",
            Self::Compute => "compute",
            Self::Particles => "particles",
            Self::PostProcess => "post_process",
        }
    }
}

/// Información de un shader compilado
#[derive(Debug, Clone)]
pub struct ShaderInfo {
    pub name: ShaderName,
    pub shader_type: ShaderType,
    pub content: ShaderContent,
    pub last_modified: std::time::SystemTime,
    pub compile_status: CompileStatus,
}

#[derive(Debug, Clone)]
pub enum CompileStatus {
    Success,
    Error(String),
    Pending,
}

/// Gestor principal de shaders
pub struct ShaderManager {
    hot_reloader: ShaderHotReloader,
    compiled_shaders: Arc<RwLock<HashMap<ShaderName, ShaderInfo>>>,
    shader_callbacks: Arc<RwLock<Vec<Box<dyn Fn(&ShaderInfo) + Send + Sync>>>>,
}

impl Clone for ShaderManager {
    fn clone(&self) -> Self {
        Self {
            hot_reloader: self.hot_reloader.clone(),
            compiled_shaders: Arc::clone(&self.compiled_shaders),
            shader_callbacks: Arc::clone(&self.shader_callbacks),
        }
    }
}

impl ShaderManager {
    /// Crea un nuevo gestor de shaders
    pub async fn new<P: AsRef<Path>>(shaders_dir: P) -> Result<Self> {
        let mut hot_reloader = ShaderHotReloader::new(shaders_dir)
            .context("Error creando hot-reloader")?;
        
        hot_reloader.start_watching().await
            .context("Error iniciando supervisión de shaders")?;
        
        let compiled_shaders = Arc::new(RwLock::new(HashMap::new()));
        let shader_callbacks = Arc::new(RwLock::new(Vec::new()));
        
        let manager = Self {
            hot_reloader,
            compiled_shaders,
            shader_callbacks,
        };
        
        // Compilar shaders iniciales
        manager.compile_all_shaders().await?;
        
        info!("🎨 Gestor de shaders inicializado");
        Ok(manager)
    }
    
    /// Compila todos los shaders disponibles
    pub async fn compile_all_shaders(&self) -> Result<()> {
        info!("🔨 Compilando todos los shaders...");
        
        let all_shaders = self.hot_reloader.get_all_shaders().await;
        let mut compiled_count = 0;
        let mut error_count = 0;
        
        for (name, content) in all_shaders {
            match self.compile_shader(&name, &content).await {
                Ok(_) => compiled_count += 1,
                Err(e) => {
                    error!("❌ Error compilando shader {}: {}", name, e);
                    error_count += 1;
                }
            }
        }
        
        info!("✅ Compilación completada: {} exitosos, {} errores", compiled_count, error_count);
        Ok(())
    }
    
    /// Compila un shader específico
    pub async fn compile_shader(&self, name: &ShaderName, content: &ShaderContent) -> Result<()> {
        debug!("🔨 Compilando shader: {}", name);
        
        // Determinar tipo de shader
        let shader_type = ShaderType::from_name(name)
            .unwrap_or_else(|| {
                warn!("⚠️ Tipo de shader desconocido para '{}', usando Fragment", name);
                ShaderType::Fragment
            });
        
        // Validar contenido
        let compile_status = match self.validate_shader_content(content, &shader_type) {
            Ok(_) => {
                info!("✅ Shader {} compilado exitosamente", name);
                CompileStatus::Success
            }
            Err(e) => {
                error!("❌ Error compilando shader {}: {}", name, e);
                CompileStatus::Error(e.to_string())
            }
        };
        
        // Crear información del shader
        let shader_info = ShaderInfo {
            name: name.clone(),
            shader_type,
            content: content.clone(),
            last_modified: std::time::SystemTime::now(),
            compile_status,
        };
        
        // Almacenar en cache
        {
            let mut compiled = self.compiled_shaders.write().await;
            compiled.insert(name.clone(), shader_info.clone());
        }
        
        // Ejecutar callbacks
        self.execute_shader_callbacks(&shader_info).await;
        
        Ok(())
    }
    
    /// Valida el contenido de un shader
    fn validate_shader_content(&self, content: &str, shader_type: &ShaderType) -> Result<()> {
        // Validaciones específicas por tipo de shader
        match shader_type {
            ShaderType::Vertex => {
                if !content.contains("@vertex") {
                    return Err(anyhow::anyhow!("Shader vertex debe contener función @vertex"));
                }
                if !content.contains("@builtin(position)") {
                    return Err(anyhow::anyhow!("Shader vertex debe retornar position"));
                }
            }
            ShaderType::Fragment => {
                if !content.contains("@fragment") {
                    return Err(anyhow::anyhow!("Shader fragment debe contener función @fragment"));
                }
                if !content.contains("@location(0)") {
                    return Err(anyhow::anyhow!("Shader fragment debe retornar color"));
                }
            }
            ShaderType::Compute => {
                if !content.contains("@compute") {
                    return Err(anyhow::anyhow!("Shader compute debe contener función @compute"));
                }
            }
            _ => {
                // Validaciones básicas para otros tipos
                if !content.contains("@fragment") && !content.contains("@vertex") {
                    return Err(anyhow::anyhow!("Shader debe contener al menos una función entry point"));
                }
            }
        }
        
        // Verificar sintaxis básica
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
    
    /// Ejecuta callbacks de shader actualizado
    async fn execute_shader_callbacks(&self, shader_info: &ShaderInfo) {
        let callbacks = self.shader_callbacks.read().await;
        for callback in callbacks.iter() {
            callback(shader_info);
        }
    }
    
    /// Registra un callback para cuando un shader se actualice
    pub async fn register_shader_callback<F>(&self, callback: F) 
    where
        F: Fn(&ShaderInfo) + Send + Sync + 'static,
    {
        let mut callbacks = self.shader_callbacks.write().await;
        callbacks.push(Box::new(callback));
    }
    
    /// Obtiene información de un shader
    pub async fn get_shader_info(&self, name: &str) -> Option<ShaderInfo> {
        let compiled = self.compiled_shaders.read().await;
        compiled.get(name).cloned()
    }
    
    /// Obtiene el contenido de un shader
    pub async fn get_shader_content(&self, name: &str) -> Option<ShaderContent> {
        let compiled = self.compiled_shaders.read().await;
        compiled.get(name).map(|info| info.content.clone())
    }
    
    /// Lista todos los shaders disponibles
    pub async fn list_shaders(&self) -> Vec<ShaderInfo> {
        let compiled = self.compiled_shaders.read().await;
        compiled.values().cloned().collect()
    }
    
    /// Lista shaders por tipo
    pub async fn list_shaders_by_type(&self, shader_type: &ShaderType) -> Vec<ShaderInfo> {
        let compiled = self.compiled_shaders.read().await;
        compiled.values()
            .filter(|info| &info.shader_type == shader_type)
            .cloned()
            .collect()
    }
    
    /// Obtiene estadísticas de compilación
    pub async fn get_compilation_stats(&self) -> (usize, usize, usize) {
        let compiled = self.compiled_shaders.read().await;
        let total = compiled.len();
        let mut success = 0;
        let mut errors = 0;
        
        for info in compiled.values() {
            match &info.compile_status {
                CompileStatus::Success => success += 1,
                CompileStatus::Error(_) => errors += 1,
                CompileStatus::Pending => {},
            }
        }
        
        (total, success, errors)
    }
    
    /// Fuerza la recompilación de todos los shaders
    pub async fn force_recompile_all(&self) -> Result<()> {
        info!("🔄 Forzando recompilación de todos los shaders...");
        self.compile_all_shaders().await
    }
    
    /// Verifica si un shader está disponible y compilado
    pub async fn is_shader_ready(&self, name: &str) -> bool {
        let compiled = self.compiled_shaders.read().await;
        compiled.get(name)
            .map(|info| matches!(info.compile_status, CompileStatus::Success))
            .unwrap_or(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use tokio::fs;
    
    #[tokio::test]
    async fn test_shader_manager() {
        let temp_dir = tempdir().unwrap();
        let shader_path = temp_dir.path().join("test_vertex.wgsl");
        
        // Crear shader de ejemplo
        fs::write(&shader_path, r#"
            @vertex
            fn vs_main() -> @builtin(position) vec4<f32> {
                return vec4<f32>(0.0, 0.0, 0.0, 1.0);
            }
        "#).await.unwrap();
        
        let manager = ShaderManager::new(temp_dir.path()).await.unwrap();
        
        // Verificar que el shader se cargó
        assert!(manager.is_shader_ready("test_vertex").await);
        
        // Verificar estadísticas
        let (total, success, errors) = manager.get_compilation_stats().await;
        assert_eq!(total, 1);
        assert_eq!(success, 1);
        assert_eq!(errors, 0);
    }
}
