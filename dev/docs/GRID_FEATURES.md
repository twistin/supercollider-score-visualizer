# 🎵 SC Score Visualizer - Funcionalidades de la Rejilla

## Descripción General

La rejilla profesional del visualizador ha sido completamente rediseñada para ofrecer máxima configurabilidad y utilidad para live coding musical. Proporciona referencias visuales precisas tanto para frecuencias como para tiempo.

## Características Principales

### 🎼 Modo Musical vs Lineal

- **Modo Musical (M)**: Divisiones basadas en notas musicales (C, C#, D, etc.)
- **Modo Lineal (M)**: Divisiones regulares y uniformes

### 🏷️ Sistema de Etiquetas

- **Etiquetas de Frecuencia (F)**: Muestra notas musicales y frecuencias en Hz
- **Etiquetas de Tiempo (L)**: Muestra divisiones temporales en segundos
- **Información Contextual**: Rango de frecuencias y configuración actual

### 📏 Resolución Configurable

- **Líneas Principales**: Divisiones mayores (configurable con +/-)
- **Líneas Menores**: Subdivisiones entre líneas principales
- **Colores Diferenciados**: Diferentes opacidades para mejor legibilidad

### 🎛️ Presets Rápidos

- **Preset 1**: Rango vocal (80-800 Hz)
- **Preset 2**: Rango instrumental (200-2000 Hz)
- **Preset 3**: Rango completo (20-20000 Hz)

## Controles de Teclado

| Tecla | Función |
|-------|---------|
| `G` | Activar/desactivar rejilla |
| `L` | Activar/desactivar etiquetas |
| `M` | Cambiar modo musical/lineal |
| `F` | Activar/desactivar etiquetas de frecuencia |
| `+` | Incrementar resolución |
| `-` | Decrementar resolución |
| `1` | Preset vocal |
| `2` | Preset instrumental |
| `3` | Preset completo |

## Configuración Avanzada

### Archivo `config.toml`

```toml
[visual.grid]
show_labels = true                    # Mostrar etiquetas
show_frequency_labels = true          # Mostrar etiquetas de frecuencia
show_time_labels = true              # Mostrar etiquetas de tiempo
major_lines = 8                      # Número de líneas principales
minor_lines = 4                      # Divisiones entre líneas principales
major_color = [1.0, 1.0, 1.0, 0.15] # Color líneas principales [R,G,B,A]
minor_color = [1.0, 1.0, 1.0, 0.08] # Color líneas menores [R,G,B,A]
center_color = [1.0, 1.0, 1.0, 0.35] # Color líneas centrales [R,G,B,A]
label_color = [1.0, 1.0, 1.0, 0.7]  # Color etiquetas [R,G,B,A]
musical_divisions = true             # Usar divisiones musicales
frequency_range = [80.0, 2000.0]    # Rango de frecuencias [min, max]
time_range = 10.0                    # Rango temporal en segundos
```

### Personalización de Colores

Todos los colores usan formato RGBA (Red, Green, Blue, Alpha):
- `[1.0, 1.0, 1.0, 0.15]` = Blanco semi-transparente
- `[0.0, 1.0, 0.0, 0.8]` = Verde brillante
- `[1.0, 0.5, 0.0, 0.3]` = Naranja tenue

## Casos de Uso

### Live Coding Musical

- **Modo Musical**: Perfecto para improvisación, muestra notas exactas
- **Presets**: Cambio rápido entre rangos según instrumento
- **Etiquetas**: Referencia visual para afinación y armonía

### Análisis Audio

- **Modo Lineal**: Mejor para análisis espectral detallado
- **Resolución Alta**: Más líneas para mayor precisión
- **Rango Completo**: Análisis de todo el espectro audible

### Performances

- **Etiquetas Desactivadas**: Menos distracciones visuales
- **Colores Sutiles**: Rejilla como referencia discreta
- **Modo Dinámico**: Cambio de configuración durante la performance

## Integración con SuperCollider

Las mejoras de la rejilla se integran perfectamente con los helpers de SuperCollider:

```supercollider
// Ejemplo de uso con diferentes rangos
~visualizer.setFreqRange(80, 800);   // Rango vocal
~visualizer.setFreqRange(200, 2000); // Rango instrumental

// Cambiar modo desde SuperCollider
~visualizer.setGridMode(\musical);   // Modo musical
~visualizer.setGridMode(\linear);    // Modo lineal
```

## Beneficios Profesionales

1. **Precisión Musical**: Referencias exactas de notas y frecuencias
2. **Flexibilidad**: Adaptable a diferentes estilos y necesidades
3. **Rendimiento**: Optimizado para uso en tiempo real
4. **Estética**: Diseño profesional y elegante
5. **Usabilidad**: Controles intuitivos y configuración fácil

## Próximas Mejoras

- Soporte para temperamentos alternativos (Just Intonation, etc.)
- Escalas musicales personalizadas
- Sincronización con tempo de SuperCollider
- Análisis armónico visual
- Exportación de configuraciones

---

*Esta documentación forma parte del SC Score Visualizer v2.0 - Sistema profesional de auto-visualización para live coding musical*
