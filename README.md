# SC Score Visualizer v2.0

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.70+-blue.svg)](https://www.rust-lang.org)
[![Version](https://img.shields.io/badge/version-2.0.0-green.svg)](https://github.com/twistin/supercollider-score-visualizer/releases)
[![OSC](https://img.shields.io/badge/OSC-supported-orange.svg)](http://opensoundcontrol.org/)
[![Platform](https://img.shields.io/badge/platform-Linux%20|%20macOS%20|%20Windows-lightgrey.svg)](https://github.com/twistin/supercollider-score-visualizer)

Un sistema de visualización de audio en tiempo real para SuperCollider, construido con Rust y Nannou. Este proyecto proporciona una interfaz visual avanzada que responde dinámicamente a eventos musicales a través del protocolo OSC.

## 📋 Tabla de Contenidos

- [✨ Características Principales](#-características-principales)
- [⚡ Inicio Rápido](#-inicio-rápido)
- [🚀 Instalación](#-instalación)
- [🎮 Uso](#-uso)
- [⚙️ Configuración](#️-configuración)
- [📡 Protocolo OSC](#-protocolo-osc)
- [🎨 Estilos Visuales](#-estilos-visuales)
- [🏗️ Arquitectura](#️-arquitectura)
- [📄 Sistema de Captura](#-sistema-de-captura)
- [🔧 Desarrollo](#-desarrollo)
- [🔧 Solución de Problemas](#-solución-de-problemas)
- [🤝 Contribución](#-contribución)
- [📄 Licencia](#-licencia)

## ✨ Características Principales

- **📡 Comunicación OSC en Tiempo Real**: Recepción y procesamiento de mensajes OSC desde SuperCollider
- **🎨 Visualización Avanzada**: Sistema de mapeo audio-visual con múltiples estilos y efectos
- **🎹 Soporte MIDI**: Integración opcional con dispositivos MIDI
- **📸 Sistema de Captura**: Guardado automático y manual de eventos visuales en JSON y PNG
- **⚙️ Configuración Flexible**: Sistema de configuración completo mediante archivos TOML
- **🎯 Alto Rendimiento**: Renderizado optimizado con nannou y OpenGL
- **🔧 Arquitectura Modular**: Código bien estructurado y fácilmente extensible

## ⚡ Inicio Rápido

¿Quieres probarlo inmediatamente? Sigue estos pasos:

```bash
# 1. Clona el repositorio
git clone https://github.com/twistin/supercollider-score-visualizer.git
cd supercollider-score-visualizer

# 2. Ejecuta el visualizador
cargo run

# 3. En SuperCollider, envía un evento de prueba
~viz = NetAddr("127.0.0.1", 7777);
~viz.sendMsg("/note", 440, 0.5, 2.0);
```

## 🚀 Instalación

### Prerrequisitos

- **Rust 1.70+** (con Cargo) - [Instalar Rust](https://rustup.rs/)
- **SuperCollider** (para envío de eventos OSC) - [Descargar SuperCollider](https://supercollider.github.io/)
- **Dependencias del sistema**:
  - **Linux**: `libasound2-dev`, `pkg-config`
  - **macOS**: Xcode Command Line Tools
  - **Windows**: Visual Studio Build Tools

### Instalación de Dependencias del Sistema

**Ubuntu/Debian:**
```bash
sudo apt-get install libasound2-dev pkg-config
```

**macOS:**
```bash
xcode-select --install
```

**Windows:**
```bash
# Instalar Visual Studio Build Tools
# https://visualstudio.microsoft.com/visual-cpp-build-tools/
```

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
# Ejecutar en modo debug (con información detallada)
cargo run

# Ejecutar en modo release (optimizado)
cargo run --release

# Con configuración personalizada
cargo run -- --config mi_config.toml

# Con logging detallado
RUST_LOG=debug cargo run
```

El visualizador se iniciará y comenzará a escuchar mensajes OSC en `127.0.0.1:7777` por defecto.

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
| `1-9`     | Cambiar estilos visuales                     |
| `↑/↓`     | Ajustar opacidad                             |
| `+/-`     | Zoom in/out                                  |
| `Esc`     | Salir                                        |

### Ejemplos Básicos con SuperCollider

```supercollider
// Configuración inicial
~viz = NetAddr("127.0.0.1", 7777);

// Nota simple
~viz.sendMsg("/note", 440, 0.5, 2.0);

// Secuencia de notas
(
Routine({
    [60, 64, 67, 72].do { |note|
        ~viz.sendMsg("/note", note.midicps, 0.7, 1.0);
        0.5.wait;
    }
}).play;
)

// Drone continuo
~viz.sendMsg("/drone", 220, 0.3, 10.0);

// Cluster de partículas
~viz.sendMsg("/cluster", 880, 0.6, 3.0, 20);
```

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
listen_port = 7777
buffer_size = 1024
timeout_ms = 10
max_messages_per_frame = 50

[window]
width = 1200
height = 800
title = "SC Score Visualizer v2.0"
vsync = true
resizable = true

[visual]
quality = "High"              # Low, Medium, High, Ultra
background_style = "Modern"   # Modern, Simple, Gradient, None
show_debug = true
show_grid = true
fps_target = 60
time_window = 10.0
max_events = 200
background_color = [8, 15, 30]
event_fade_time = 3.0

[audio]
freq_min = 20.0
freq_max = 20000.0
amp_min = 0.0
amp_max = 1.0
dur_min = 0.1
dur_max = 10.0

[performance]
max_notes = 100
max_drones = 10
max_cluster_particles = 100
cleanup_interval_frames = 300

[midi]
enabled = true
default_note_duration = 1.0
velocity_scaling = 1.0

[logging]
level = "Info"               # Debug, Info, Warning, Error
show_osc_messages = true
show_performance_stats = true
stats_interval_frames = 300
```

### Configuración OSC

Para conectar desde SuperCollider:

```supercollider
// Configurar NetAddr para el visualizador
~visualizer = NetAddr("127.0.0.1", 7777);

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
~viz = NetAddr("127.0.0.1", 7777);

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

## 📸 Capturas de Pantalla

> **Nota**: Las capturas de pantalla se actualizarán próximamente con ejemplos visuales del sistema en funcionamiento.

### Ejemplo de Visualización
```
┌─────────────────────────────────────────────────────────────┐
│ SC Score Visualizer v2.0                    [Debug: ON]    │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│       ●                    ◐                               │
│                 ◯                     ●                    │
│                                                             │
│                     ◐        ●                             │
│                                        ◯                   │
│                                                             │
│  ◯                                                          │
│                          ●                                 │
│                                                             │
├─────────────────────────────────────────────────────────────┤
│ Events: 12 | FPS: 60 | OSC: 127.0.0.1:7777 | Grid: ON     │
└─────────────────────────────────────────────────────────────┘
```

### Estilos Visuales Disponibles
- **Modern**: Gradientes suaves y efectos modernos
- **Simple**: Visualización minimalista y limpia
- **Gradient**: Fondos dinámicos con degradados
- **None**: Sin fondo, solo eventos puros

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

## 🔧 Solución de Problemas

### Problemas Comunes

#### Error: "alsa.pc not found" (Linux)
```bash
# Ubuntu/Debian
sudo apt-get install libasound2-dev pkg-config

# Fedora/CentOS
sudo yum install alsa-lib-devel pkgconfig
```

#### Error: "No se puede conectar al puerto OSC"
1. Verifica que el puerto 7777 no esté siendo usado por otra aplicación
2. Cambia el puerto en `config.toml`:
   ```toml
   [osc]
   listen_port = 8888
   ```
3. Actualiza la configuración en SuperCollider:
   ```supercollider
   ~viz = NetAddr("127.0.0.1", 8888);
   ```

#### Rendimiento bajo o visualización lenta
1. Ajusta la calidad visual:
   ```toml
   [visual]
   quality = "Medium"  # o "Low"
   fps_target = 30
   ```
2. Reduce el número máximo de eventos:
   ```toml
   [performance]
   max_notes = 50
   max_drones = 5
   ```

#### Error de compilación en Windows
1. Instala Visual Studio Build Tools
2. Asegúrate de tener las herramientas de C++ instaladas
3. Reinicia el terminal después de la instalación

### Debugging

#### Activar logs detallados
```bash
RUST_LOG=debug cargo run
```

#### Verificar mensajes OSC
```bash
RUST_LOG=sc_score_visualizer::osc=debug cargo run
```

#### Probar conectividad OSC
```supercollider
// Test de conexión simple
~viz = NetAddr("127.0.0.1", 7777);
~viz.sendMsg("/test", "hello");
```

### Rendimiento

#### Configuración optimizada para máquinas lentas
```toml
[visual]
quality = "Low"
fps_target = 30
background_style = "Simple"
show_debug = false
max_events = 50

[performance]
max_notes = 30
max_drones = 3
cleanup_interval_frames = 180
```

#### Configuración para máquinas potentes
```toml
[visual]
quality = "Ultra"
fps_target = 120
background_style = "Modern"
max_events = 500

[performance]
max_notes = 300
max_drones = 20
max_cluster_particles = 500
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

## 📊 Estado del Proyecto

### Funcionalidades Implementadas ✅
- ✅ Visualización en tiempo real de eventos OSC
- ✅ Soporte para notas, drones y clusters
- ✅ Sistema de configuración mediante TOML
- ✅ Múltiples estilos visuales y calidades
- ✅ Captura manual de eventos
- ✅ Controles de teclado
- ✅ Soporte MIDI básico
- ✅ Sistema de logging configurable
- ✅ Arquitectura modular

### En Desarrollo 🚧
- 🚧 Captura automática de screenshots
- 🚧 Interfaz gráfica para configuración
- 🚧 Más efectos visuales avanzados
- 🚧 Soporte para múltiples ventanas
- 🚧 Integración con DAWs

### Roadmap 📋
- 📋 Soporte para WebSocket
- 📋 Plugin para SuperCollider
- 📋 Modo VR/AR experimental
- 📋 Análisis de espectro en tiempo real
- 📋 Sincronización con video
- 📋 Interfaz web remota

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

## 📞 Soporte y Comunidad

### Reportar Problemas
Para reportar bugs o solicitar funcionalidades:

- 🐛 **Issues**: [GitHub Issues](https://github.com/twistin/supercollider-score-visualizer/issues)
- 💬 **Discusiones**: [GitHub Discussions](https://github.com/twistin/supercollider-score-visualizer/discussions)
- 📧 **Email**: Para soporte directo o consultas privadas

### Canales de Comunicación
- **Bugs**: Usar GitHub Issues con etiqueta `bug`
- **Feature Requests**: Usar GitHub Issues con etiqueta `enhancement`
- **Preguntas**: Usar GitHub Discussions
- **Contribuciones**: Seguir el proceso de Pull Request

### Información Útil para Reportes
Al reportar un problema, incluye:
1. **Versión**: `cargo run --version`
2. **Sistema Operativo**: Linux/macOS/Windows + versión
3. **Rust Version**: `rustc --version`
4. **Configuración**: Tu archivo `config.toml`
5. **Logs**: Ejecutar con `RUST_LOG=debug cargo run`
6. **Pasos para reproducir**: Descripción detallada

### FAQ (Preguntas Frecuentes)

**¿Funciona con otras aplicaciones además de SuperCollider?**
Sí, cualquier aplicación que pueda enviar mensajes OSC puede comunicarse con el visualizador.

**¿Puedo usar múltiples instancias del visualizador?**
Sí, cada instancia puede configurarse para escuchar en puertos diferentes.

**¿Hay soporte para dispositivos móviles?**
Actualmente no, pero está en el roadmap futuro.

**¿Cómo contribuir al proyecto?**
Ver la sección [Contribución](#-contribución) para detalles completos.

---

**SC Score Visualizer** - Convirtiendo código en arte visual, un beat a la vez. 🎵✨

*¿Te gusta el proyecto? ¡Dale una ⭐ en GitHub!*
