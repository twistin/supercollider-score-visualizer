// 📖 Ejemplo de uso del sistema de hot-reload de shaders
// Este archivo muestra cómo integrar el sistema de shaders en tu aplicación
use nannou::prelude::*;

use std::path::Path;
use tracing::{info, warn, error};
use crate::visual::{ShaderManager, ShaderType, ShaderInfo, CompileStatus};

/// Ejemplo de implementación del sistema de shaders
pub struct ShaderExample {
    manager: ShaderManager,
}

impl ShaderExample {
    /// Crea un nuevo ejemplo de shaders
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        // Inicializar el gestor de shaders
        let shaders_dir = Path::new("src/visual/shaders");
        let manager = ShaderManager::new(shaders_dir).await?;
        
        Ok(Self { manager })
    }
    
    /// Demuestra cómo usar el sistema de shaders
    pub async fn demonstrate_usage(&self) -> Result<(), Box<dyn std::error::Error>> {
        info!("🎨 Demostrando uso del sistema de shaders...");
        
        // 1. Listar shaders disponibles
        self.list_available_shaders().await;
        
        // 2. Registrar callback para cambios de shaders
        self.register_shader_callbacks().await;
        
        // 3. Obtener contenido de un shader específico
        self.get_shader_content().await;
        
        // 4. Recompilar shaders
        self.recompile_shaders().await?;
        
        // 5. Obtener estadísticas
        self.show_statistics().await;
        
        Ok(())
    }
    
    /// Lista todos los shaders disponibles
    async fn list_available_shaders(&self) {
        info!("📋 Listando shaders disponibles...");
        
        let shaders = self.manager.list_shaders().await;
        for shader in shaders {
            let status_icon = match shader.compile_status {
                CompileStatus::Success => "✅",
                CompileStatus::Error(_) => "❌",
                CompileStatus::Pending => "⏳",
            };
            
            info!("   {} {} ({})", status_icon, shader.name, shader.shader_type.to_string());
            
            if let CompileStatus::Error(ref error) = shader.compile_status {
                warn!("     Error: {}", error);
            }
        }
    }
    
    /// Registra callbacks para cambios de shaders
    async fn register_shader_callbacks(&self) {
        info!("🔗 Registrando callbacks para cambios de shaders...");
        
        self.manager.register_shader_callback(|shader_info: &ShaderInfo| {
            match &shader_info.compile_status {
                CompileStatus::Success => {
                    info!("✅ Shader '{}' recargado exitosamente", shader_info.name);
                }
                CompileStatus::Error(error) => {
                    error!("❌ Error en shader '{}': {}", shader_info.name, error);
                }
                CompileStatus::Pending => {
                    info!("⏳ Shader '{}' en proceso de compilación", shader_info.name);
                }
            }
        }).await;
    }
    
    /// Obtiene el contenido de un shader específico
    async fn get_shader_content(&self) {
        info!("📄 Obteniendo contenido de shaders...");
        
        // Intentar obtener el shader vertex
        if let Some(content) = self.manager.get_shader_content("vertex").await {
            info!("✅ Shader vertex cargado, {} caracteres", content.len());
            
            // Mostrar las primeras líneas del shader
            let lines: Vec<&str> = content.lines().take(5).collect();
            for (i, line) in lines.iter().enumerate() {
                info!("   {}: {}", i + 1, line);
            }
        } else {
            warn!("⚠️ Shader vertex no encontrado");
        }
    }
    
    /// Recompila todos los shaders
    async fn recompile_shaders(&self) -> Result<(), Box<dyn std::error::Error>> {
        info!("🔄 Recompilando todos los shaders...");
        
        self.manager.force_recompile_all().await?;
        
        info!("✅ Recompilación completada");
        Ok(())
    }
    
    /// Muestra estadísticas de compilación
    async fn show_statistics(&self) {
        info!("📊 Estadísticas de compilación:");
        
        let (total, success, errors) = self.manager.get_compilation_stats().await;
        
        info!("   Total de shaders: {}", total);
        info!("   Compilados exitosamente: {}", success);
        info!("   Con errores: {}", errors);
        
        if total > 0 {
            let success_rate = (success as f32 / total as f32) * 100.0;
            info!("   Tasa de éxito: {:.1}%", success_rate);
        }
    }
    
    /// Verifica si un shader está listo para usar
    pub async fn is_shader_ready(&self, name: &str) -> bool {
        self.manager.is_shader_ready(name).await
    }
    
    /// Obtiene información detallada de un shader
    pub async fn get_shader_info(&self, name: &str) -> Option<ShaderInfo> {
        self.manager.get_shader_info(name).await
    }
    
    /// Lista shaders por tipo
    pub async fn list_shaders_by_type(&self, shader_type: ShaderType) -> Vec<ShaderInfo> {
        self.manager.list_shaders_by_type(&shader_type).await
    }
}

/// Función de utilidad para crear un shader de ejemplo
pub async fn create_example_shader() -> Result<(), Box<dyn std::error::Error>> {
    info!("🎨 Creando shader de ejemplo...");
    
    let shader_dir = Path::new("src/visual/shaders");
    if !shader_dir.exists() {
        std::fs::create_dir_all(shader_dir)?;
    }
    
    let example_shader = r#"
// 🌟 Shader de ejemplo con efectos dinámicos
// Este shader demuestra el sistema de hot-reload

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>,
    @location(1) uv: vec2<f32>,
}

struct Uniforms {
    time: f32,
    audio_level: f32,
}

@group(0) @binding(0)
var<uniform> uniforms: Uniforms;

@vertex
fn vs_main(@location(0) position: vec3<f32>) -> VertexOutput {
    var output: VertexOutput;
    output.clip_position = vec4<f32>(position, 1.0);
    output.color = vec3<f32>(1.0, 0.5, 0.2);
    output.uv = position.xy * 0.5 + 0.5;
    return output;
}

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    // Efecto de ondas basado en tiempo
    let wave = sin(uniforms.time * 2.0 + input.uv.x * 10.0) * 0.5 + 0.5;
    
    // Color dinámico con audio
    let color = input.color * (0.5 + wave * 0.5) * (1.0 + uniforms.audio_level);
    
    return vec4<f32>(color, 1.0);
}
"#;
    
    let example_path = shader_dir.join("example.wgsl");
    std::fs::write(&example_path, example_shader)?;
    
    info!("✅ Shader de ejemplo creado en: {:?}", example_path);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    
    #[tokio::test]
    async fn test_shader_example() {
        let temp_dir = tempdir().unwrap();
        let shader_path = temp_dir.path().join("test.wgsl");
        
        // Crear shader de prueba
        std::fs::write(&shader_path, r#"
            @vertex
            fn vs_main() -> @builtin(position) vec4<f32> {
                return vec4<f32>(0.0, 0.0, 0.0, 1.0);
            }
            
            @fragment
            fn fs_main() -> @location(0) vec4<f32> {
                return vec4<f32>(1.0, 0.0, 0.0, 1.0);
            }
        "#).unwrap();
        
        // Crear manager con directorio temporal
        let manager = ShaderManager::new(temp_dir.path()).await.unwrap();
        let example = ShaderExample { manager };
        
        // Verificar que el shader se cargó
        assert!(example.is_shader_ready("test").await);
        
        // Verificar que podemos obtener la información
        let shader_info = example.get_shader_info("test").await;
        assert!(shader_info.is_some());
        
        // Verificar estadísticas
        example.show_statistics().await;
    }
}
