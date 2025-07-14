# Guía Completa de Live Coding con SC Score Visualizer

## 📋 Índice
1. [Introducción](#introducción)
2. [Configuración Inicial](#configuración-inicial)
3. [Instalación de Dependencias](#instalación-de-dependencias)
4. [Configuración del Entorno](#configuración-del-entorno)
5. [Inicio del Sistema](#inicio-del-sistema)
6. [Ejemplos Básicos](#ejemplos-básicos)
7. [Ejemplos Avanzados](#ejemplos-avanzados)
8. [Técnicas de Live Coding](#técnicas-de-live-coding)
9. [Solución de Problemas](#solución-de-problemas)
10. [Referencia Rápida](#referencia-rápida)

---

## 🎵 Introducción

SC Score Visualizer es un sistema de visualización de audio en tiempo real que permite crear sesiones de live coding utilizando **SuperCollider** para generar audio y **Nannou** (Rust) para la visualización. Este documento te guiará paso a paso para configurar y probar una sesión de live coding completa.

### ¿Qué es Live Coding?

Live Coding es la práctica de crear música y visualizaciones en tiempo real escribiendo código. Este sistema te permite:
- Generar sonidos con SuperCollider
- Visualizar el audio en tiempo real con efectos dinámicos
- Modificar tanto el audio como las visualizaciones en vivo
- Crear performances audiovisuales interactivas

---

## ⚙️ Configuración Inicial

### Prerrequisitos del Sistema

#### 1. Instalar Rust
```bash
# Instalar Rust a través de rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Verificar instalación
rustc --version
cargo --version
```

#### 2. Instalar SuperCollider
```bash
# En macOS con Homebrew
brew install supercollider

# En Ubuntu/Debian
sudo apt-get update
sudo apt-get install supercollider supercollider-ide

# En Arch Linux
sudo pacman -S supercollider sc3-plugins
```

#### 3. Dependencias del Sistema (Linux)
```bash
# Para Ubuntu/Debian
sudo apt-get install libasound2-dev pkg-config

# Para Arch Linux
sudo pacman -S alsa-lib pkgconf

# Para Fedora/RHEL
sudo dnf install alsa-lib-devel pkgconfig
```

---

## 🔧 Instalación de Dependencias

### Clonar e Instalar el Proyecto

```bash
# Clonar el repositorio
git clone https://github.com/twistin/supercollider-score-visualizer.git
cd supercollider-score-visualizer

# Compilar el proyecto
cargo build --release

# Ejecutar tests para verificar instalación
cargo test
```

### Verificar Instalación

```bash
# Verificar que el binario fue creado
ls -la target/release/sc_score_visualizer

# Verificar configuración
cat config.toml
```

---

## 🌐 Configuración del Entorno

### Configurar el Archivo `config.toml`

El archivo `config.toml` contiene todas las configuraciones del sistema:

```toml
[osc]
# Configuración OSC - debe coincidir con SuperCollider
listen_host = "127.0.0.1"
listen_port = 7777          # Puerto donde escucha el visualizador
buffer_size = 1024
timeout_ms = 10
max_messages_per_frame = 50

[window]
# Configuración de la ventana
width = 1200
height = 800
title = "SC Score Visualizer v2.0"
vsync = true
resizable = true

[visual]
# Configuración visual
quality = "High"                    # Low, Medium, High, Ultra
background_style = "Modern"         # Modern, Simple, Gradient, None
show_debug = true
show_grid = true
fps_target = 60
max_events = 200
event_fade_time = 3.0

[audio]
# Rangos de validación para audio
freq_min = 20.0
freq_max = 20000.0
amp_min = 0.0
amp_max = 1.0
```

### Configurar SuperCollider

Crear un archivo `setup_visualizer.scd` con la configuración básica:

```supercollider
// setup_visualizer.scd - Configuración básica del visualizador

// Configurar servidor de audio
s.boot;

// Configurar NetAddr para comunicación OSC
~visualizer = NetAddr("127.0.0.1", 7777);

// Función para enviar notas al visualizador
~sendNote = { |freq, amp, dur|
    ~visualizer.sendMsg("/note", freq, amp, dur);
};

// Función para enviar drones (sonidos continuos)
~sendDrone = { |freq, amp, dur|
    ~visualizer.sendMsg("/drone", freq, amp, dur);
};

// Función para enviar clusters (múltiples eventos)
~sendCluster = { |freq, amp, dur, density|
    ~visualizer.sendMsg("/cluster", freq, amp, dur, density);
};

// Función para limpiar eventos
~clearAll = {
    ~visualizer.sendMsg("/clear");
};

"Configuración del visualizador cargada ✅".postln;
```

---

## 🚀 Inicio del Sistema

### Paso 1: Iniciar el Visualizador

```bash
# Método 1: Usando el script de inicio
./start_visualizer.sh

# Método 2: Directamente con cargo
cargo run --release

# Método 3: Ejecutar binario directamente
./target/release/sc_score_visualizer
```

### Paso 2: Verificar Conexión OSC

El visualizador debe mostrar:
```
🚀 Iniciando SC Score Visualizer v2.0
🔧 Cargando configuración...
✅ Configuración cargada y validada
📡 Servidor OSC iniciado en 127.0.0.1:7777
🎵 Visualizador listo para recibir mensajes
```

### Paso 3: Iniciar SuperCollider

```bash
# Iniciar SuperCollider IDE
scide

# O desde terminal
sclang
```

### Paso 4: Cargar Configuración en SuperCollider

```supercollider
// Cargar el archivo de configuración
"setup_visualizer.scd".load;
```

---

## 🎹 Ejemplos Básicos

### Ejemplo 1: Primera Nota

```supercollider
// Enviar una nota simple
~sendNote.value(440, 0.5, 2.0);  // La4, amplitud media, 2 segundos

// Alternativa más explícita
~visualizer.sendMsg("/note", 440, 0.5, 2.0);
```

### Ejemplo 2: Secuencia de Notas

```supercollider
// Rutina que toca una escala
(
Routine({
    var escala = [60, 62, 64, 65, 67, 69, 71, 72].midicps;
    escala.do({ |freq|
        ~sendNote.value(freq, 0.6, 0.5);
        0.5.wait;
    });
}).play;
)
```

### Ejemplo 3: Drone Básico

```supercollider
// Crear un drone continuo
~sendDrone.value(110, 0.3, 5.0);  // La2, suave, 5 segundos
```

### Ejemplo 4: Cluster de Eventos

```supercollider
// Crear un cluster con múltiples frecuencias
~sendCluster.value(330, 0.4, 3.0, 8);  // Frecuencia base, amp, duración, densidad
```

---

## 🎨 Ejemplos Avanzados

### Ejemplo 5: Pattern Dinámico

```supercollider
// Pattern que cambia dinámicamente
(
Pdef(\visualPattern,
    Pbind(
        \type, \osc,
        \addr, ~visualizer,
        \path, "/note",
        \freq, Pseq([440, 523, 659, 784], inf),
        \amp, Prand([0.3, 0.5, 0.7], inf),
        \dur, Prand([0.25, 0.5, 0.75], inf),
        \delta, 0.25
    )
).play;
)

// Detener el pattern
Pdef(\visualPattern).stop;
```

### Ejemplo 6: Generador Aleatorio

```supercollider
// Generador de notas aleatorias
(
~randomGenerator = Routine({
    loop {
        var freq = rrand(200, 2000);
        var amp = rrand(0.2, 0.8);
        var dur = rrand(0.5, 3.0);
        
        ~sendNote.value(freq, amp, dur);
        rrand(0.1, 1.0).wait;
    }
}).play;
)

// Detener generador
~randomGenerator.stop;
```

### Ejemplo 7: Efectos Visuales Complejos

```supercollider
// Combinar diferentes tipos de eventos
(
~complexVisual = Routine({
    loop {
        // Enviar nota principal
        ~sendNote.value(rrand(300, 800), 0.7, 2.0);
        
        // Agregar drone de fondo
        ~sendDrone.value(rrand(80, 120), 0.2, 4.0);
        
        // Cluster esporádico
        if (0.3.coin) {
            ~sendCluster.value(rrand(500, 1500), 0.5, 1.5, rrand(5, 12));
        };
        
        rrand(0.5, 2.0).wait;
    }
}).play;
)
```

---

## 🎭 Técnicas de Live Coding

### Técnica 1: Modificación en Tiempo Real

```supercollider
// Definir variables globales que se pueden modificar
~tempo = 0.5;
~baseFreq = 440;
~amplitude = 0.5;

// Rutina que usa estas variables
(
~livePattern = Routine({
    loop {
        ~sendNote.value(~baseFreq * rrand(0.5, 2.0), ~amplitude, ~tempo);
        ~tempo.wait;
    }
}).play;
)

// Modificar en vivo (ejecutar mientras suena)
~tempo = 0.25;      // Más rápido
~baseFreq = 330;    // Frecuencia base diferente
~amplitude = 0.8;   // Más fuerte
```

### Técnica 2: Capas de Sonido

```supercollider
// Crear múltiples capas
(
// Capa 1: Melodía principal
~melody = Routine({
    var notas = [60, 64, 67, 72].midicps;
    loop {
        ~sendNote.value(notas.choose, 0.6, 0.5);
        0.5.wait;
    }
}).play;

// Capa 2: Bajo
~bass = Routine({
    var bajo = [36, 40, 43].midicps;
    loop {
        ~sendNote.value(bajo.choose, 0.4, 1.0);
        1.0.wait;
    }
}).play;

// Capa 3: Texturas
~texture = Routine({
    loop {
        if (0.4.coin) {
            ~sendDrone.value(rrand(200, 400), 0.2, 3.0);
        };
        2.0.wait;
    }
}).play;
)
```

### Técnica 3: Control Visual Directo

```supercollider
// Controlar aspectos visuales específicos
(
~visualControl = Routine({
    loop {
        // Nota con parámetros visuales específicos
        ~visualizer.sendMsg("/note", 
            rrand(200, 800),    // frecuencia
            rrand(0.3, 0.8),    // amplitud
            rrand(1.0, 3.0)     // duración
        );
        
        // Cambiar configuración visual ocasionalmente
        if (0.2.coin) {
            ~visualizer.sendMsg("/config", "background_style", "Gradient");
        };
        
        0.8.wait;
    }
}).play;
)
```

---

## 🔧 Controles del Visualizador

### Controles de Teclado

| Tecla | Función |
|-------|---------|
| `G` | Mostrar/ocultar grilla |
| `D` | Mostrar/ocultar información debug |
| `P` | Pausar/reanudar visualización |
| `R` | Reiniciar visualización |
| `H` | Mostrar/ocultar ayuda |
| `F` | Pantalla completa |
| `ESC` | Salir |
| `1-4` | Cambiar temas visuales |
| `TAB` | Menú completo |

### Mensajes OSC Soportados

| Mensaje | Parámetros | Descripción |
|---------|------------|-------------|
| `/note` | `freq, amp, dur` | Evento de nota musical |
| `/drone` | `freq, amp, dur` | Sonido continuo |
| `/cluster` | `freq, amp, dur, density` | Cluster de eventos |
| `/clear` | - | Limpiar todos los eventos |
| `/config` | `param, value` | Cambiar configuración |

---

## 🐛 Solución de Problemas

### Problema 1: Sin Visualización

**Síntomas:** El visualizador se inicia pero no muestra nada cuando se envían mensajes OSC.

**Solución:**
```bash
# Verificar que el puerto OSC esté correcto
netstat -an | grep 7777

# Verificar configuración
cat config.toml | grep listen_port
```

```supercollider
// Verificar conexión OSC en SuperCollider
~visualizer = NetAddr("127.0.0.1", 7777);
~visualizer.sendMsg("/note", 440, 0.5, 1.0);
```

### Problema 2: Error de Compilación

**Síntomas:** `cargo build` falla con errores de dependencias.

**Solución:**
```bash
# Instalar dependencias del sistema
sudo apt-get install libasound2-dev pkg-config

# Limpiar y recompilar
cargo clean
cargo build --release
```

### Problema 3: SuperCollider No Conecta

**Síntomas:** SuperCollider no puede enviar mensajes OSC.

**Solución:**
```supercollider
// Verificar que el servidor esté corriendo
s.boot;

// Recrear NetAddr
~visualizer = NetAddr("127.0.0.1", 7777);

// Probar conexión
~visualizer.sendMsg("/note", 440, 0.5, 1.0);
```

### Problema 4: Rendimiento Lento

**Síntomas:** Visualización con lag o fps bajos.

**Solución:**
```toml
# Ajustar config.toml
[visual]
quality = "Medium"          # Reducir calidad
max_events = 100           # Menos eventos simultáneos
fps_target = 30            # FPS más bajo

[performance]
max_notes = 50
cleanup_interval_frames = 150
```

---

## 📚 Referencia Rápida

### Comandos Básicos SuperCollider

```supercollider
// Configuración inicial
s.boot;
~visualizer = NetAddr("127.0.0.1", 7777);

// Enviar eventos
~visualizer.sendMsg("/note", freq, amp, dur);
~visualizer.sendMsg("/drone", freq, amp, dur);
~visualizer.sendMsg("/cluster", freq, amp, dur, density);
~visualizer.sendMsg("/clear");

// Utilerías
rrand(min, max);           // Número aleatorio
.choose;                   // Elegir elemento aleatorio
.midicps;                  // MIDI a frecuencia
.wait;                     // Esperar en Routine
```

### Comandos Básicos Terminal

```bash
# Compilar y ejecutar
cargo build --release
cargo run --release

# Ejecutar tests
cargo test

# Verificar puerto
netstat -an | grep 7777
lsof -i :7777
```

### Configuración Rápida

```toml
# config.toml mínimo
[osc]
listen_port = 7777

[window]
width = 1200
height = 800

[visual]
quality = "High"
background_style = "Modern"
```

---

## 🎯 Próximos Pasos

1. **Experimenta** con los ejemplos básicos
2. **Modifica** los parámetros en tiempo real
3. **Combina** diferentes tipos de eventos
4. **Crea** tus propias rutinas de live coding
5. **Explora** los efectos visuales avanzados

### Recursos Adicionales

- **Documentación SuperCollider:** [https://supercollider.github.io/](https://supercollider.github.io/)
- **Nannou Framework:** [https://nannou.cc/](https://nannou.cc/)
- **Ejemplos en el repositorio:** `./tests/` y `./examples/`

---

## 🤝 Contribución

Si encuentras problemas o tienes mejoras:

1. Reporta issues en GitHub
2. Contribuye con ejemplos nuevos
3. Mejora la documentación
4. Comparte tus sesiones de live coding

---

**¡Disfruta creando arte audiovisual en tiempo real!** 🎵✨