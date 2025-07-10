# SC Score Visualizer - Auto-Visualización Profesional

Sistema completo de auto-visualización para live coding musical con SuperCollider y visualizador Rust/Nannou.

## 🚀 Inicio Rápido

### 1. Iniciar el Visualizador
```bash
./start_system.sh
```

### 2. Configurar SuperCollider
```supercollider
// Primera vez: ejecutar bloques 1-6 de sc_auto_visualizer.scd
// Uso diario: ejecutar solo bloque 6
```

### 3. Live Coding con Visualización Automática
```supercollider
// Pbind con visualización
a = ~visualPbind.value(\freq, Pseq([220, 330, 440], inf), \dur, 0.25);
a.play;

// Synth individual
~visualSynth.value(\default, [\freq, 880, \amp, 0.3]);

// NodeProxy
~pad = { SinOsc.ar(220) * 0.1 };
~visualProxy.value(~pad, 220, 0.1);
```

## 📋 Archivos Principales

- **`sc_auto_visualizer.scd`** - Sistema completo de auto-visualización
- **`test_visualizer.scd`** - Test rápido de conectividad
- **`start_system.sh`** - Script de inicio automático
- **`src/main.rs`** - Código fuente del visualizador

## 🎛️ Controles en Vivo

```supercollider
~visualizerOn.value();          // Activar visualización
~visualizerOff.value();         // Desactivar visualización
~visualizerDebug.value(true);   // Modo debug
~testVisualizer.value();        // Test de conexión
```

## 🔧 Requisitos

- SuperCollider
- Rust (para compilar el visualizador)
- macOS/Linux/Windows

## 🎵 Características

- ✅ **Auto-visualización**: Cualquier sonido se visualiza automáticamente
- ✅ **Compatible con ProxySpace**: Funciona con flujos de live coding estándar
- ✅ **Análisis en tiempo real**: Análisis espectral continuo
- ✅ **Fácil de usar**: Una vez configurado, funciona transparentemente
- ✅ **Profesional**: Robusto para sesiones largas de improvisación

## 🐛 Troubleshooting

- **No veo visualización**: Verificar que `./start_system.sh` esté ejecutándose
- **Errores OSC**: Ejecutar `~testVisualizer.value()` en SuperCollider
- **Debug**: Activar `~visualizerDebug.value(true)` para ver eventos

## 📡 Configuración OSC

- **Puerto**: 57124
- **Mensajes**:
  - `/event` - Eventos musicales discretos
  - `/realtime_audio` - Análisis espectral continuo

¡Listo para live coding con visualización automática! 🎊
