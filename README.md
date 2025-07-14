# SC Score Visualizer v2.0

Un sistema de visualización de audio en tiempo real para SuperCollider, construido con Rust y Nannou. Este proyecto proporciona una interfaz visual avanzada que responde dinámicamente a eventos musicales a través del protocolo OSC.

## ✨ Características Principales

- **📡 Comunicación OSC en Tiempo Real**: Recepción y procesamiento de mensajes OSC desde SuperCollider
- **🎨 Visualización Avanzada**: Sistema de mapeo audio-visual con múltiples estilos y efectos
- **🎹 Soporte MIDI**: Integración opcional con dispositivos MIDI
- **📸 Sistema de Captura**: Guardado automático y manual de eventos visuales en JSON y PNG
- **⚙️ Configuración Flexible**: Sistema de configuración completo mediante archivos TOML
- **🎯 Alto Rendimiento**: Renderizado optimizado con nannou y OpenGL
- **🔧 Arquitectura Modular**: Código bien estructurado y fácilmente extensible

## 🚀 Instalación

### Prerrequisitos

- **Rust 1.70+** (con Cargo)
- **SuperCollider** (para envío de eventos OSC)
- **macOS, Linux o Windows**

### Instalación desde el código fuente

```bash
# Clonar el repositorio
git clone https://github.com/twistin/supercollider-score-visualizer.git
cd sc_score_visualizer

# Compilar en modo debug
cargo build

# Compilar en modo release (optimizado)
cargo build --release

# Ejecutar
cargo run
```

## 🎮 Uso

### Ejecutar el Visualizador

```bash
cargo run
```

El visualizador se iniciará y comenzará a escuchar mensajes OSC en `127.0.0.1:57124` por defecto.

### Controles de Teclado

| Tecla     | Función                                      |
| --------- | -------------------------------------------- |
| `Espacio` | Pausar/reanudar visualización                |
| `R`       | Reiniciar visualización                      |
| `D`       | Toggle información de debug                  |
| `G`       | Toggle grilla                                |
| `H`       | Mostrar/ocultar ayuda                        |
| `P`       | **Captura manual** (guarda eventos actuales) |
| `F`       | Toggle pantalla completa                     |
| `Esc`     | Salir                                        |

### Captura de Eventos

El sistema incluye un módulo de captura que permite:

- **Captura Manual**: Presiona `P` para guardar el estado actual de los eventos visuales
- **Guardado JSON**: Todos los eventos se guardan con timestamps en formato JSON
- **Capturas PNG**: Sistema preparado para capturas de pantalla automáticas
- **Directorio de Salida**: Archivos guardados en `./captures/`

## ⚙️ Configuración

El comportamiento del visualizador se controla mediante el archivo `config.toml`:

```toml
[osc]
listen_host = "127.0.0.1"
listen_port = 57124
buffer_size = 1024

[window]
width = 1200
height = 800
title = "SC Score Visualizer v2.0"
vsync = true

[visual]
quality = "High"
background_style = "Modern"
show_debug = true
fps_target = 60
max_events = 200
event_fade_time = 3.0

[audio]
freq_min = 20.0
freq_max = 20000.0
amp_min = 0.0
amp_max = 1.0

[performance]
max_notes = 100
max_drones = 10
cleanup_interval_frames = 300
```

### Configuración OSC

Para conectar desde SuperCollider:

```supercollider
// Configurar NetAddr para el visualizador
~visualizer = NetAddr("127.0.0.1", 57124);

// Enviar un evento de nota
~visualizer.sendMsg("/note", freq, amp, dur);

// Enviar un drone
~visualizer.sendMsg("/drone", freq, amp, dur);

// Enviar evento de cluster
~visualizer.sendMsg("/cluster", freq, amp, dur, density);
```

## 🏗️ Arquitectura

```
src/
├── main.rs              # Punto de entrada y loop principal
├── model.rs             # Estructuras de datos principales
├── config.rs            # Sistema de configuración
├── osc_server.rs        # Servidor OSC y manejo de mensajes
├── audio.rs             # Procesamiento de eventos de audio
├── capture.rs           # Sistema de captura y guardado
├── midi.rs              # Soporte MIDI opcional
├── logging.rs           # Sistema de logging
├── errors.rs            # Manejo de errores
├── visual/
│   ├── mod.rs
│   ├── audio_visual_mapping.rs      # Mapeo básico audio-visual
│   ├── audio_visual_mapping_pro.rs  # Mapeo avanzado
│   └── shader_manager.rs            # Gestión de shaders
└── visuals.rs           # Renderizado y efectos visuales
```

### Componentes Principales

1. **Model**: Estructura central que mantiene el estado de la aplicación
2. **OscServer**: Maneja la comunicación OSC entrante
3. **AudioVisualMapper**: Convierte eventos de audio en representaciones visuales
4. **CaptureSystem**: Gestiona el guardado de eventos y capturas
5. **VisualNote**: Representación de eventos musicales visuales

## 📡 Protocolo OSC

### Mensajes Soportados

| Dirección  | Parámetros                | Descripción               |
| ---------- | ------------------------- | ------------------------- |
| `/note`    | `freq, amp, dur`          | Evento de nota musical    |
| `/drone`   | `freq, amp, dur`          | Sonido continuo/drone     |
| `/cluster` | `freq, amp, dur, density` | Cluster de eventos        |
| `/beat`    | `time`                    | Marcador de tiempo        |
| `/stop`    | -                         | Detener todos los eventos |

### Ejemplo de Uso con SuperCollider

```supercollider
(
// Configurar conexión
~viz = NetAddr("127.0.0.1", 57124);

// Secuencia de ejemplo
Routine({
    loop {
        ~viz.sendMsg("/note", 
            rrand(200, 800),    // frecuencia
            rrand(0.1, 0.8),    // amplitud
            rrand(0.5, 2.0)     // duración
        );
        0.5.wait;
    }
}).play;
)
```

## 🎨 Estilos Visuales

El visualizador soporta múltiples estilos configurables:

- **Modern**: Estilo contemporáneo con gradientes y efectos suaves
- **Simple**: Visualización minimalista
- **Gradient**: Fondos con degradados dinámicos
- **None**: Sin fondo, solo eventos

### Tipos de Eventos Visuales

- **Notas**: Círculos que pulsan y se desvanecen
- **Drones**: Ondas continuas y suaves
- **Clusters**: Múltiples partículas sincronizadas
- **Beats**: Marcadores de tiempo visuales

## 🔧 Desarrollo

### Compilación

```bash
# Modo desarrollo (con debug info)
cargo build

# Modo release (optimizado)
cargo build --release

# Con logs detallados
RUST_LOG=debug cargo run
```

### Testing

```bash
# Ejecutar tests
cargo test

# Tests con output detallado
cargo test -- --nocapture
```

### Linting y Formato

```bash
# Formatear código
cargo fmt

# Ejecutar linter
cargo clippy

# Verificar todo
cargo check --all-targets
```

## 📄 Sistema de Captura

El sistema de captura permite guardar eventos visuales para análisis posterior:

### Archivos JSON

```json
{
  "timestamp": "2024-01-15T10:30:45.123Z",
  "events": [
    {
      "event_type": "Note",
      "frequency": 440.0,
      "amplitude": 0.7,
      "duration": 1.5,
      "position": [100, 200],
      "color": [255, 100, 50, 200],
      "created_at": 1234567890.123
    }
  ],
  "config": {
    "capture_enabled": true,
    "auto_save": false,
    "output_dir": "./captures/"
  }
}
```

### Capturas PNG

- Guardado automático en beats (configurable)
- Captura manual con tecla `P`
- Resolución configurable
- Nombrado automático con timestamp

## 🤝 Contribución

¡Las contribuciones son bienvenidas! Por favor:

1. Haz fork del proyecto
2. Crea una rama para tu feature (`git checkout -b feature/AmazingFeature`)
3. Commit tus cambios (`git commit -m 'Add some AmazingFeature'`)
4. Push a la rama (`git push origin feature/AmazingFeature`)
5. Abre un Pull Request

### Guías de Desarrollo

- Seguir las convenciones de Rust (usar `rustfmt` y `clippy`)
- Añadir tests para nuevas funcionalidades
- Documentar APIs públicas
- Mantener la compatibilidad con versiones anteriores

## 📝 Changelog

### v2.0.0 (Actual)

- ✨ Sistema de captura de eventos en JSON y PNG
- 🎨 Mejoras visuales y nuevos estilos
- ⚡ Optimizaciones de rendimiento
- 🔧 Sistema de configuración mejorado
- 📡 Protocolo OSC más robusto

### v1.x

- Implementación inicial
- Soporte básico OSC
- Visualización fundamental

## 📄 Licencia

Este proyecto está licenciado bajo la Licencia MIT. Ver [LICENSE](LICENSE) para más detalles.

## 🙏 Agradecimientos

- **[Nannou](https://nannou.cc/)** - Framework de creative coding para Rust
- **[SuperCollider](https://supercollider.github.io/)** - Plataforma de síntesis de audio
- Comunidad de Rust por las increíbles herramientas y librerías

## 📞 Soporte

Para reportar bugs o solicitar funcionalidades:

- 🐛 [Issues en GitHub](https://github.com/twistin/supercollider-score-visualizer/issues)
- 💬 [Discusiones](https://github.com/twistin/supercollider-score-visualizer/discussions)

---

**SC Score Visualizer** - Convirtiendo código en arte visual, un beat a la vez. 🎵✨
