# 🎵 SC Score Visualizer

Sistema de visualización musical universal que convierte cualquier audio/MIDI en visualizaciones abstractas en tiempo real.

## 🚀 Inicio Rápido

### 1. Ejecutar Visualizador
```bash
cargo run --release
```

### 2. Ejecutar SuperCollider
```supercollider
// Abrir: supercollider_clean.scd
// Ejecutar todo: Cmd+A, Cmd+Return
// Probar: scvTest()
```

## ✅ Estado: PRODUCCIÓN
- ✅ **Sistema visual universal** implementado
- ✅ **OSC robusta** (puerto 57124)
- ✅ **SuperCollider integrado** sin errores
- ✅ **Código optimizado** y limpio

## 📖 Documentación Completa
Ver **[DOCUMENTACION_MAESTRA.md](DOCUMENTACION_MAESTRA.md)** para:
- Guía completa de uso
- Especificaciones técnicas
- Solución de problemas
- Configuración avanzada
- Arquitectura del sistema

## 🎯 Archivos Principales
- `supercollider_clean.scd` - ✅ **USAR ESTE** (100% funcional)
- `config.toml` - Configuración optimizada
- `src/main.rs` - Motor visual universal
- `diagnostico_visual.scd` - Tests y diagnóstico

## 🔧 Funciones SuperCollider
```supercollider
scvTest()          // Prueba básica
scvXenakis()       // Composición Xenakis
scvQuick()         // Demo rápido
```

---

**⚡ Para uso inmediato**: `supercollider_clean.scd` + `cargo run --release`

## Características

### 🎼 Tipos de Eventos Musicales

- **Puntos**: Eventos discretos con envolvente de ataque/decaimiento
- **Glissandi**: Líneas curvas con control de curvatura (Bézier)
- **Clusters**: Masas sonoras con múltiples voces y evolución de densidad
- **Ruido**: Texturas granulares con control espectral
- **Masas Sonoras**: Estructuras espectrales complejas con turbulencia

### 🎨 Estética Visual

- Inspirada en *Metastaseis*, *Pithoprakta* y *Nomos Alpha* de Xenakis
- Paleta de colores estilo partitura manuscrita
- Representación precisa de tiempo y frecuencia
- Grilla opcional para referencia temporal/espectral
- Efectos de textura y dispersión espacial

### ⚡ Características Técnicas

- **Comunicación OSC en tiempo real** (puerto 57120)
- **Arquitectura multi-hilo** para rendering fluido
- **Escalado temporal y espectral** interactivo
- **Gestión optimizada de memoria** con límites configurables
- **Exportación de capturas** en formato PNG
- **Efectos generativos** con ruido Perlin

## Instalación

### Requisitos

- Rust 1.70+ (https://rustup.rs/)
- SuperCollider 3.12+ (opcional, para envío de eventos)

## Documentación

Para más detalles sobre el uso y funcionamiento del proyecto, consulta los siguientes documentos en la carpeta `docs/`:

- **[Inicio Rápido](docs/INICIO_RAPIDO.md)**: Guía para comenzar a usar el proyecto rápidamente
- **[Guía de Uso](docs/GUIA_USO.md)**: Documentación detallada de uso y características
- **[Estado Final](docs/ESTADO_FINAL.md)**: Resumen del estado actual del proyecto
- **[Mejoras](docs/MEJORAS.md)**: Lista de mejoras implementadas y propuestas
- **[Solución ProxySpace](docs/SOLUCION_PROXYSPACE.md)**: Documentación sobre la refactorización de variables
- **[Solución de Errores](docs/SOLUCION_ERROR.md)**: Guía de solución de problemas comunes
- **[Variables Corregidas](docs/VARIABLES_CORREGIDAS.md)**: Detalles sobre las correcciones de variables
- **[Archivo Corregido](docs/ARCHIVO_CORREGIDO.md)**: Información sobre las correcciones del archivo principal

### Compilación

```bash
git clone <repository-url>
cd sc_score_visualizer
cargo build --release
```

### Ejecución

```bash
cargo run
```

La aplicación se iniciará y comenzará a escuchar mensajes OSC en el puerto 57120.

## Uso

### Controles de Teclado

- **[+] / [-]**: Zoom temporal (acelerar/ralentizar tiempo)
- **[↑] / [↓]**: Zoom espectral (expandir/contraer frecuencias)
- **[G]**: Alternar grilla de tiempo/frecuencia
- **[R]**: Reset (escala y eventos)
- **[S]**: Capturar pantalla (guardar PNG)

### Envío de Eventos desde SuperCollider

Carga el archivo `supercollider_examples.scd` en SuperCollider para ejemplos completos.

#### Formato de Mensajes OSC

Todos los mensajes se envían a la dirección `/event` con los siguientes formatos:

##### Evento Puntual

```supercollider
~visualizer.sendMsg("/event", "point", freq, amp, duration, attack, decay, density, texture, spatial_spread, color_hue);
```

##### Glissando

```supercollider
~visualizer.sendMsg("/event", "gliss", start_freq, end_freq, amp, duration, curvature, density, texture, spatial_spread, color_hue);
```

##### Cluster

```supercollider
~visualizer.sendMsg("/event", "cluster", center_freq, freq_spread, num_voices, amp, duration, density, texture, spatial_spread, color_hue);
```

##### Ruido/Textura

```supercollider
~visualizer.sendMsg("/event", "noise", freq_center, freq_bandwidth, amp, duration, grain_size, spectral_tilt, density, texture, spatial_spread, color_hue);
```

##### Masa Sonora

```supercollider
~visualizer.sendMsg("/event", "mass", num_components, amp, duration, evolution_rate, turbulence, density, texture, spatial_spread, color_hue, freq1, amp1, freq2, amp2, ...);
```

### Parámetros Explicados

- **freq**: Frecuencia en Hz (20-8000)
- **amp**: Amplitud (0.0-1.0)
- **duration**: Duración en segundos
- **attack/decay**: Tiempos de envolvente (0.0-1.0)
- **density**: Densidad del evento (0.0-1.0)
- **texture**: Rugosidad/granularidad (0.0-1.0)
- **spatial_spread**: Dispersión espacial (0.0-1.0)
- **color_hue**: Matiz del color (0.0-360.0)
- **curvature**: Curvatura de glissando (-1.0 a 1.0)
- **num_voices**: Número de voces en cluster
- **freq_spread**: Dispersión frecuencial en Hz
- **grain_size**: Tamaño de grano para ruido
- **spectral_tilt**: Inclinación espectral (-1.0 a 1.0)
- **evolution_rate**: Velocidad de evolución espectral
- **turbulence**: Nivel de turbulencia (0.0-1.0)

## Ejemplos de Composición

### Secuencia Estilo Metastaseis

```supercollider
// Puntos dispersos con densidad creciente
50.do { |i|
    var freq = 200 + (4000 * i / 50);
    var density = i / 50;
    ~visualizer.sendMsg("/event", "point", freq, 0.5, 1.0, 0.1, 0.1, density, density.squared, 0.2, 360.rand);
    0.2.wait;
};
```

### Glissandi Convergentes Estilo Pithoprakta

```supercollider
// Glissandi con curvaturas aleatorias
12.do { |i|
    var startFreq = 220 + 660.rand;
    var endFreq = 220 + 660.rand;
    var curvature = -1.0 + 2.0.rand;
    ~visualizer.sendMsg("/event", "gliss", startFreq, endFreq, 0.6, 3.0, curvature, 0.8, 0.3, 0.4, 30);
    0.3.wait;
};
```

### Nubes Estocásticas

```supercollider
// Clusters con parámetros aleatorios
8.do {
    var centerFreq = 200 + 2000.rand;
    var spread = 100 + 300.rand;
    var voices = 6 + 8.rand;
    ~visualizer.sendMsg("/event", "cluster", centerFreq, spread, voices, 0.7, 4.0, 0.8, 0.5, 0.6, 120);
    2.0.rand.wait;
};
```

## Configuración Avanzada

### Constantes Configurables (src/main.rs)

```rust
const TIME_WINDOW: f64 = 15.0;     // Ventana temporal visible
const MAX_FREQ: f32 = 8000.0;     // Frecuencia máxima
const MIN_FREQ: f32 = 20.0;       // Frecuencia mínima
const OSC_PORT: u16 = 57120;      // Puerto OSC
const MAX_EVENTS: usize = 1000;   // Máximo eventos en memoria
```

### Paleta de Colores

La aplicación usa una paleta inspirada en las partituras de Xenakis:

- Negro profundo para estructuras principales
- Rojo intenso para eventos dinámicos
- Azul estructural para glissandi
- Marrón tierra para clusters
- Verde orgánico para texturas
- Violeta misterioso para masas sonoras

## Arquitectura del Código

```
src/main.rs
├── Modelo de Datos
│   ├── MusicalEvent (estructura principal)
│   ├── EventType (enum de tipos de eventos)
│   └── VisualConfig (configuración visual)
├── Configuración OSC
│   └── Hilo separado para recepción
├── Lógica de Actualización
│   ├── Procesamiento de mensajes
│   └── Gestión de memoria
├── Renderizado
│   ├── Funciones específicas por tipo
│   ├── Sistema de coordenadas
│   └── Efectos visuales
└── Utilidades
    ├── Parser OSC
    ├── Conversiones de coordenadas
    └── Generación de colores
```

## Extensiones Futuras

- [ ] Análisis espectral en tiempo real
- [ ] Exportación de animaciones (video)
- [ ] Sistema de capas visuales
- [ ] L-systems para patrones generativos
- [ ] Soporte MIDI adicional
- [ ] Interfaz web para control remoto
- [ ] Presets de configuración visual
- [ ] Modo de reproducción/grabación

## Contribución

Las contribuciones son bienvenidas. Por favor:

1. Fork el repositorio
2. Crea una rama para tu feature (`git checkout -b feature/nueva-caracteristica`)
3. Commit tus cambios (`git commit -am 'Añadir nueva característica'`)
4. Push a la rama (`git push origin feature/nueva-caracteristica`)
5. Crea un Pull Request

## Licencia

Este proyecto está bajo la licencia MIT. Ver archivo LICENSE para detalles.

## Créditos

- Inspirado en las partituras gráficas de **Iannis Xenakis**
- Construido con **Nannou** (framework creativo en Rust)
- Comunicación OSC vía **nannou_osc**
- Efectos generativos con **noise-rs**

---

*"La música es la manifestación artística de la inteligencia cósmica"* - Iannis Xenakis
