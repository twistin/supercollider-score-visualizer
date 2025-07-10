# 🎵 SC Score Visualizer - Corrección de Etiquetas en Ejes

## ✅ Problema Resuelto

Se corrigió el problema donde las etiquetas de frecuencia (eje Y) y tiempo (eje X) no se mostraban correctamente en la visualización.

## 🔧 Cambios Realizados

### 1. **Reorganización de Funciones de Dibujo**
- **`draw_grid()`**: Separada la lógica de líneas y etiquetas
- **`draw_frequency_grid()`**: Solo dibuja líneas horizontales de frecuencia
- **`draw_time_grid()`**: Solo dibuja líneas verticales de tiempo
- **`draw_frequency_labels_left()`**: Nueva función para etiquetas de frecuencia en eje Y
- **`draw_time_labels_bottom()`**: Nueva función para etiquetas de tiempo en eje X

### 2. **Corrección de Orientación**
- **Frecuencias**: Líneas horizontales + etiquetas en eje Y (izquierda)
- **Tiempo**: Líneas verticales + etiquetas en eje X (abajo)

### 3. **Mejoras Visuales**
- Fondos semitransparentes para etiquetas: `rgba(0.03, 0.06, 0.12, 0.95)`
- Bordes azules para mejor contraste: `rgba(0.3, 0.6, 1.0, 0.6)`
- Tamaños de texto optimizados: 13-14px para etiquetas principales
- Formato inteligente: kHz para frecuencias >1000Hz

### 4. **Configuración Validada**
```toml
[visual.grid]
show_labels = true
show_frequency_labels = true
show_time_labels = true
musical_divisions = true
frequency_range = [80.0, 2000.0]
time_range = 10.0
```

## 📋 Estructura de Ejes

```
        Frecuencias (Hz/kHz)
        ↑
        │
C6 1kHz │─────────────────────────────  (Eje Y - Izquierda)
        │
A4 440Hz│─────────────────────────────
        │
C3 131Hz│─────────────────────────────
        │
        └─────────────────────────────→ Tiempo (s)
         0s    2.5s    5.0s    7.5s    10s
                (Eje X - Abajo)
```

## 🎮 Controles de Usuario

- **L**: Alternar etiquetas on/off
- **M**: Alternar modo musical/lineal
- **G**: Alternar rejilla on/off
- **ESC**: Salir

## 🚀 Cómo Probar

```bash
# Ejecución directa
./test_labels.sh

# O manualmente
cargo build --release
./target/release/sc_score_visualizer
```

## 💡 Características Clave

1. **Etiquetas de Frecuencia (Eje Y)**:
   - Modo Musical: Notas + octavas (C4, A4, etc.)
   - Modo Lineal: Frecuencias en Hz/kHz
   - Posición: Izquierda de la pantalla

2. **Etiquetas de Tiempo (Eje X)**:
   - Formato: Segundos decimales (0.0s, 2.5s, etc.)
   - Posición: Parte inferior de la pantalla

3. **Alineación Perfecta**:
   - Etiquetas alineadas exactamente con las líneas de la rejilla
   - Distribución logarítmica para frecuencias
   - Distribución lineal para tiempo

¡Las etiquetas ahora se muestran correctamente en ambos ejes! 🎵
