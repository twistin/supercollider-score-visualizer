# SC Score Visualizer - Mejoras Implementadas

## Resumen de Mejoras

Tu proyecto de visualización musical inspirado en Xenakis ha sido significativamente mejorado y expandido. Aquí están las mejoras implementadas:

## 🚀 Mejoras Arquitecturales

### 1. **Modelo de Datos Expandido**
- **Eventos musicales más complejos**: Ahora soporta 5 tipos de eventos:
  - `Point`: Eventos puntuales con envolvente de ataque/decaimiento
  - `Glissando`: Líneas curvas con control de curvatura Bézier
  - `Cluster`: Masas sonoras con múltiples voces y evolución de densidad
  - `Noise`: Texturas granulares con control espectral
  - `SoundMass`: Estructuras espectrales complejas con turbulencia

### 2. **Parámetros Visuales Avanzados**
- **Amplitud**: Control de intensidad visual
- **Densidad**: Complejidad del evento (0.0-1.0)
- **Textura**: Rugosidad/granularidad visual (0.0-1.0)
- **Dispersión espacial**: Expansión del evento en el espacio
- **Matiz de color**: Control del color (0.0-360.0)

### 3. **Sistema de Renderizado Especializado**
Cada tipo de evento tiene su propia función de renderizado:
- `render_point_event()`: Puntos con envolvente y efectos de textura
- `render_glissando_event()`: Curvas Bézier con ruido opcional
- `render_cluster_event()`: Masas de voces conectadas
- `render_noise_event()`: Granos distribuidos espectralmente
- `render_sound_mass_event()`: Perfiles espectrales evolutivos

## 🎨 Mejoras Visuales

### 1. **Estética Xenakis**
- **Paleta de colores** inspirada en partituras manuscritas
- **Fondo cremoso** similar al papel de partitura
- **Grilla opcional** para referencia temporal/espectral
- **Efectos de densidad** y textura procedurales

### 2. **Efectos Visuales Avanzados**
- **Ruido Perlin** para efectos orgánicos de textura
- **Dispersión espacial** para masas sonoras
- **Envolventes dinámicas** con fade-out
- **Conexiones entre elementos** en clusters y masas

### 3. **Sistema de Coordenadas Mejorado**
- **Escala logarítmica** para frecuencias (20Hz - 8kHz)
- **Zoom temporal e espectral** interactivo
- **Ventana temporal** deslizante (15 segundos por defecto)
- **Conversión precisa** frecuencia→píxel

## 🎛️ Controles Interactivos

### Teclado:
- **[+] / [-]**: Zoom temporal
- **[↑] / [↓]**: Zoom espectral
- **[G]**: Alternar grilla
- **[R]**: Reset completo
- **[S]**: Captura de pantalla

### Información en Pantalla:
- Contador de eventos activos
- Tiempo transcurrido
- Escalas actuales
- Instrucciones de control

## 🔧 Mejoras Técnicas

### 1. **Gestión de Memoria**
- **VecDeque** para eventos (más eficiente que Vec)
- **Límite de eventos** (1000 máximo en memoria)
- **Limpieza automática** de eventos antiguos
- **Optimización de allocaciones**

### 2. **Parser OSC Robusto**
- **Manejo de errores** mejorado
- **Parámetros opcionales** con valores por defecto
- **Soporte completo** para todos los tipos de eventos
- **Validación** de tipos de datos

### 3. **Configuración Flexible**
```rust
const TIME_WINDOW: f64 = 15.0;     // Ventana temporal
const MAX_FREQ: f32 = 8000.0;     // Frecuencia máxima
const MIN_FREQ: f32 = 20.0;       // Frecuencia mínima
const OSC_PORT: u16 = 57120;      // Puerto OSC
const MAX_EVENTS: usize = 1000;   // Máximo eventos
```

## 📡 Protocolo OSC Extendido

### Formatos de Mensaje:

#### Punto:
```
/event "point" freq amp duration [attack] [decay] [density] [texture] [spatial_spread] [color_hue]
```

#### Glissando:
```
/event "gliss" start_freq end_freq amp duration [curvature] [density] [texture] [spatial_spread] [color_hue]
```

#### Cluster:
```
/event "cluster" center_freq freq_spread num_voices amp duration [density] [texture] [spatial_spread] [color_hue]
```

#### Ruido:
```
/event "noise" freq_center freq_bandwidth amp duration [grain_size] [spectral_tilt] [density] [texture] [spatial_spread] [color_hue]
```

#### Masa Sonora:
```
/event "mass" num_components amp duration [evolution_rate] [turbulence] [density] [texture] [spatial_spread] [color_hue] [freq1 amp1 freq2 amp2 ...]
```

## 🗂️ Archivos Añadidos

### 1. **supercollider_examples.scd**
- Ejemplos completos de uso desde SuperCollider
- Funciones de utilidad para envío de eventos
- Composiciones automáticas estilo Xenakis
- Patrones rítmicos complejos

### 2. **test_osc.py**
- Script de prueba independiente (no requiere SuperCollider)
- Envía eventos de ejemplo automáticamente
- Útil para probar la aplicación rápidamente

### 3. **README.md**
- Documentación completa del proyecto
- Instrucciones de instalación y uso
- Ejemplos de código
- Guía de configuración

## 🚀 Cómo Probar

### 1. **Ejecutar la Aplicación:**
```bash
cd /Users/sdcarr/Documents/github/sc-score/sc_score_visualizer
cargo run --release
```

### 2. **Enviar Eventos de Prueba:**
```bash
# Opción 1: Con Python
python3 test_osc.py

# Opción 2: Con SuperCollider (carga supercollider_examples.scd)
// En SuperCollider:
~xenakisComposition.();
```

## 📈 Mejoras de Rendimiento

- **Compilación optimizada** (`--release`)
- **Renderizado eficiente** con funciones especializadas
- **Gestión de memoria** mejorada
- **Threading** separado para OSC y rendering
- **VSync** automático con Nannou

## 🎯 Próximos Pasos Sugeridos

1. **Análisis espectral** en tiempo real
2. **Exportación de video** de las composiciones
3. **Presets visuales** para diferentes estilos
4. **Sistema de capas** para eventos superpuestos
5. **Integración MIDI** adicional
6. **Interfaz web** para control remoto

## 🎼 Inspiración Xenakis Implementada

- ✅ **Metastaseis**: Puntos dispersos con densidad evolutiva
- ✅ **Pithoprakta**: Glissandi convergentes/divergentes
- ✅ **Nomos Alpha**: Estructuras complejas con turbulencia
- ✅ **Masas sonoras**: Eventos estocásticos densos
- ✅ **Notación gráfica**: Representación visual precisa

El proyecto ahora representa fielmente la complejidad y belleza de las partituras gráficas de Xenakis, con capacidades técnicas modernas y una arquitectura extensible para futuras mejoras.

¡La aplicación está lista para crear visualizaciones musicales de alta calidad en tiempo real!
