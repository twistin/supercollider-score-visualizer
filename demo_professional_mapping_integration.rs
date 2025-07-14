// demo_professional_mapping_integration.rs
// Demostración completa de integración de mapeos profesionales

use sc_score_visualizer::visual::shader_manager::{ShaderManager, ShaderConfig, RenderQuality, string_to_event_kind};
use sc_score_visualizer::visual::audio_visual_mapping_pro::{ProMappingConfig, ColorPalette, EventKind};
use nannou::prelude::*;

/// Demostración de todas las funciones profesionales de mapeo
fn main() {
    println!("🎵 DEMOSTRACIÓN DE MAPEOS PROFESIONALES 🎵");
    println!("==========================================");
    
    demo_freq_to_x_scroll();
    demo_amp_to_opacity();
    demo_freq_to_color();
    demo_kind_to_shape();
    demo_complete_pipeline();
}

/// **Demostración de freq_to_x_scroll()**
/// Muestra cómo las frecuencias se mapean a posiciones X con scroll temporal
fn demo_freq_to_x_scroll() {
    println!("\n🔄 DEMO: freq_to_x_scroll() - Mapeo de frecuencia a posición X con scroll");
    println!("----------------------------------------------------------------------");
    
    let mut shader_manager = ShaderManager::default();
    shader_manager.update_time(0.0);
    
    let test_frequencies = vec![
        (100.0, "Grave (100 Hz)"),
        (440.0, "A4 (440 Hz)"),
        (1000.0, "Agudo (1 kHz)"),
        (4000.0, "Muy agudo (4 kHz)"),
    ];
    
    println!("Tiempo inicial (t=0.0):");
    for (freq, desc) in &test_frequencies {
        let pos_x = shader_manager.pro_mapper.freq_to_x_scroll(*freq, 0.0, 0.0);
        println!("  {} -> X: {:.1} px", desc, pos_x);
    }
    
    println!("\nDespués de 2 segundos (t=2.0):");
    for (freq, desc) in &test_frequencies {
        let pos_x = shader_manager.pro_mapper.freq_to_x_scroll(*freq, 2.0, 0.0);
        println!("  {} -> X: {:.1} px (moviéndose hacia la izquierda)", desc, pos_x);
    }
    
    println!("\n📝 Observación: Las frecuencias altas se mueven más rápido (mayor velocidad visual)");
}

/// **Demostración de amp_to_opacity()**
/// Muestra la conversión perceptual logarítmica de amplitud a opacidad
fn demo_amp_to_opacity() {
    println!("\n💡 DEMO: amp_to_opacity() - Conversión perceptual logarítmica");
    println!("------------------------------------------------------------");
    
    let shader_manager = ShaderManager::default();
    
    let test_amplitudes = vec![
        (0.001, "Muy suave"),
        (0.1, "Suave"),
        (0.3, "Medio"),
        (0.7, "Fuerte"),
        (1.0, "Máximo"),
    ];
    
    for (amp, desc) in test_amplitudes {
        let opacity = shader_manager.pro_mapper.amp_to_opacity(amp);
        let amp_db = 20.0 * amp.log10();
        println!("  {} (amp: {:.3}, {:.1} dB) -> Opacidad: {:.3}", 
                desc, amp, amp_db, opacity);
    }
    
    println!("\n📝 Observación: Uso de escala logarítmica (dB) para percepción natural del volumen");
}

/// **Demostración de freq_to_color()**
/// Muestra los diferentes sistemas de mapeo de frecuencia a color
fn demo_freq_to_color() {
    println!("\n🌈 DEMO: freq_to_color() - Mapeo de frecuencia a color con paletas artísticas");
    println!("-----------------------------------------------------------------------");
    
    let palettes = vec![
        (ColorPalette::Classical, "Clásica (círculo de quintas)"),
        (ColorPalette::Modern, "Moderna (lineal vibrante)"),
        (ColorPalette::Thermal, "Térmica (graves=frío, agudos=cálido)"),
        (ColorPalette::Spectral, "Espectral (basada en espectro visible)"),
        (ColorPalette::Ambient, "Ambiente (suave y calmante)"),
        (ColorPalette::Electronic, "Electrónica (energética)"),
    ];
    
    let test_notes = vec![
        (261.63, "C4"),
        (293.66, "D4"),
        (329.63, "E4"),
        (349.23, "F4"),
        (392.00, "G4"),
        (440.00, "A4"),
        (493.88, "B4"),
    ];
    
    for (palette, palette_name) in palettes {
        println!("\n  Paleta {}: ", palette_name);
        let config = ProMappingConfig::default();
        let mut mapper = sc_score_visualizer::visual::audio_visual_mapping_pro::ProAudioVisualMapper::new(config, palette);
        
        for (freq, note_name) in &test_notes {
            let color = mapper.freq_to_color(*freq, 0.7);
            println!("    {} ({:.2} Hz) -> RGB({}, {}, {})", 
                    note_name, freq, color.red, color.green, color.blue);
        }
    }
    
    println!("\n📝 Observación: Cada paleta mapea las mismas frecuencias a colores coherentes pero estéticamente diferentes");
}

/// **Demostración de kind_to_shape()**
/// Muestra cómo diferentes tipos de eventos se mapean a formas visuales
fn demo_kind_to_shape() {
    println!("\n🔷 DEMO: kind_to_shape() - Mapeo de tipo de evento a forma visual");
    println!("---------------------------------------------------------------");
    
    let shader_manager = ShaderManager::default();
    
    let event_types = vec![
        (EventKind::Note, "Nota musical"),
        (EventKind::Chord, "Acorde"),
        (EventKind::Percussion, "Percusión"),
        (EventKind::Sustained, "Sonido sostenido"),
        (EventKind::Transient, "Transitorio"),
        (EventKind::Noise, "Ruido/textura"),
        (EventKind::Control, "Evento de control"),
        (EventKind::Custom("glissando".to_string()), "Glissando personalizado"),
    ];
    
    let test_freq = 440.0;
    let test_amp = 0.6;
    let test_duration = 1.5;
    
    for (event_kind, description) in event_types {
        let shape = shader_manager.pro_mapper.kind_to_shape(&event_kind, test_freq, test_amp, test_duration);
        println!("  {} -> {:?}", description, shape);
    }
    
    println!("\n📝 Observación: Cada tipo de evento musical usa una forma que refleja sus características perceptuales");
}

/// **Demostración del pipeline completo**
/// Muestra cómo se integran todas las funciones en el sistema completo
fn demo_complete_pipeline() {
    println!("\n🎼 DEMO: Pipeline completo de conversión profesional");
    println!("==================================================");
    
    let mut shader_manager = ShaderManager::new(
        ShaderConfig {
            quality: RenderQuality::High,
            enable_post_processing: true,
            glow_intensity: 1.8,
            ..Default::default()
        },
        ProMappingConfig::default(),
        ColorPalette::Modern,
    );
    
    shader_manager.update_time(1.5);
    
    // Simular diferentes eventos musicales llegando al sistema
    let musical_events = vec![
        (220.0, 0.8, 2.0, "note", "piano", "Nota grave de piano"),
        (440.0, 0.6, 1.0, "chord", "strings", "Acorde de cuerdas"),
        (880.0, 0.9, 0.3, "percussion", "drums", "Golpe de batería"),
        (110.0, 0.4, 5.0, "sustained", "pad", "Pad sostenido"),
        (1760.0, 0.7, 0.1, "transient", "synth", "Transitorio de síntesis"),
    ];
    
    println!("\nCreando notas visuales con mapeos profesionales:");
    
    for (freq, amp, duration, event_type, instrument, description) in musical_events {
        println!("\n  📍 {}", description);
        
        // Usar el sistema completo para crear una nota visual
        let event_kind = string_to_event_kind(event_type);
        let birth_time = shader_manager.current_time;
        
        let visual_note = shader_manager.create_professional_visual_note(
            freq,
            amp,
            duration,
            event_kind,
            birth_time,
            instrument,
        );
        
        // Mostrar los resultados de todos los mapeos
        println!("    🎵 Frecuencia: {:.1} Hz", freq);
        println!("    🔊 Amplitud: {:.2} -> Opacidad: {:.3}", amp, visual_note.opacity);
        println!("    📍 Posición: ({:.1}, {:.1})", visual_note.position.x, visual_note.position.y);
        println!("    🎨 Color: RGB({}, {}, {})", 
                visual_note.color.red, visual_note.color.green, visual_note.color.blue);
        println!("    📐 Tamaño: {:.1} px", visual_note.size);
        println!("    🔷 Estilo: {:?}", visual_note.visual_style);
        println!("    ✨ Textura: {:?}", visual_note.texture_style);
        println!("    🎯 ID: {}", visual_note.id);
    }
    
    // Mostrar estadísticas del manager
    let stats = shader_manager.get_stats();
    println!("\n📊 Estadísticas del ShaderManager:");
    println!("    Cache de formas: {} elementos", stats.cache_size);
    println!("    Tiempo actual: {:.2} segundos", stats.current_time);
    println!("    Calidad de renderizado: {:?}", stats.quality);
    println!("    Tamaño de ventana: {:.0}x{:.0}", stats.window_size.x, stats.window_size.y);
}

/// **Comparación entre sistema legacy y profesional**
fn demo_comparison() {
    println!("\n⚖️  DEMO: Comparación Legacy vs Profesional");
    println!("==========================================");
    
    // TODO: Esta función se puede expandir para mostrar diferencias específicas
    // entre el sistema anterior y el nuevo sistema profesional
    
    println!("Sistema Legacy:");
    println!("  ✅ Mapeo básico lineal de frecuencia a posición Y");
    println!("  ✅ Mapeo simple de amplitud a tamaño");
    println!("  ✅ Colores básicos basados en HSV");
    println!("  ❌ Sin scroll temporal");
    println!("  ❌ Sin escalas perceptuales");
    println!("  ❌ Sin mapeo de formas por tipo de evento");
    
    println!("\nSistema Profesional:");
    println!("  🚀 Mapeo logarítmico de frecuencia a posición Y (octavas)");
    println!("  🚀 Scroll temporal con velocidad basada en frecuencia");
    println!("  🚀 Opacidad perceptual usando escala dB");
    println!("  🚀 Paletas artísticas (Clásica, Térmica, Espectral, etc.)");
    println!("  🚀 Formas semánticas por tipo de evento musical");
    println!("  🚀 Sistema de shaders con calidad configurable");
    println!("  🚀 Cache de formas para optimización");
    println!("  🚀 Efectos de post-procesado (glow, motion blur)");
}

#[cfg(test)]
mod integration_tests {
    use super::*;
    
    #[test]
    fn test_professional_pipeline_integration() {
        let mut shader_manager = ShaderManager::default();
        shader_manager.update_time(1.0);
        
        // Test que el pipeline completo funciona sin errores
        let note = shader_manager.create_professional_visual_note(
            440.0,
            0.5,
            2.0,
            EventKind::Note,
            0.0,
            "test_instrument",
        );
        
        assert!(note.frequency == 440.0);
        assert!(note.amplitude == 0.5);
        assert!(note.opacity > 0.0 && note.opacity <= 1.0);
        assert!(!note.id.is_empty());
        assert!(note.size > 0.0);
    }
    
    #[test]
    fn test_all_event_kinds_produce_valid_shapes() {
        let shader_manager = ShaderManager::default();
        
        let event_kinds = vec![
            EventKind::Note,
            EventKind::Chord,
            EventKind::Percussion,
            EventKind::Sustained,
            EventKind::Transient,
            EventKind::Noise,
            EventKind::Control,
        ];
        
        for kind in event_kinds {
            let shape = shader_manager.pro_mapper.kind_to_shape(&kind, 440.0, 0.5, 1.0);
            // Verificar que se produce una forma válida (no panic)
            match shape {
                sc_score_visualizer::visual::audio_visual_mapping_pro::VisualShape::Circle { radius } => {
                    assert!(radius > 0.0);
                },
                sc_score_visualizer::visual::audio_visual_mapping_pro::VisualShape::Rectangle { width, height } => {
                    assert!(width > 0.0 && height > 0.0);
                },
                _ => {
                    // Otras formas son válidas por defecto
                }
            }
        }
    }
}
