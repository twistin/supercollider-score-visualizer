# SC Score Visualizer - Arquitectura Modular

## 📁 Estructura de Módulos Refactorizada

El proyecto ha sido refactorizado siguiendo una arquitectura modular clara con separación de responsabilidades:

### 🎯 Módulos Principales

#### `src/main.rs` - Punto de Entrada
- **Responsabilidad**: Inicialización, configuración y gestión del ciclo de vida de Nannou App
- **Funciones principales**:
  - `main()`: Punto de entrada de la aplicación
  - `model_setup()`: Configuración inicial del modelo y ventana
  - `update()`: Bucle de actualización principal
  - `view()`: Función de renderizado

#### `src/model.rs` - Estructuras de Datos
- **Responsabilidad**: Definición de todas las estructuras de datos y tipos
- **Estructuras principales**:
  - `Model`: Estructura principal que contiene el estado de la aplicación
  - `Note` / `MusicalEvent`: Eventos musicales discretos
  - `DroneEvent`: Eventos de tonos continuos
  - `ClusterData`: Datos del cluster de partículas
  - `AnalysisData`: Datos de análisis continuo
  - `DisplayConfig`: Configuración de pantalla
  - `BackgroundStyle`, `VisualQuality`, `DisplayMode`: Enums de configuración

#### `src/audio.rs` - Gestión de Audio OSC
- **Responsabilidad**: Parsing, recepción y procesamiento de mensajes OSC
- **Funciones principales**:
  - `process_osc_messages()`: Procesa todos los mensajes OSC entrantes
  - `handle_osc_message()`: Maneja mensajes OSC individuales
  - `handle_event_message()`: Procesa eventos discretos (/event)
  - `handle_analysis_message()`: Procesa datos de análisis (/analysis)
  - `handle_drone_message()`: Procesa eventos de drone (/drone)
  - `handle_cluster_message()`: Procesa datos de cluster (/cluster)
- **Utilidades**:
  - `OscStats`: Estadísticas de mensajes OSC
  - Funciones de extracción y validación de argumentos

#### `src/visual.rs` - Visualización y Renderizado
- **Responsabilidad**: Funciones de visualización, render, estilos y mapeo de coordenadas
- **Funciones principales**:
  - `draw_visualization()`: Función principal de renderizado
  - `draw_background()`: Renderizado del fondo
  - `draw_grid()`: Renderizado de la rejilla
  - `draw_cluster()`: Renderizado del cluster de partículas
  - `draw_drone()`: Renderizado de eventos de drone
  - `draw_analysis()`: Renderizado de análisis continuo
  - `draw_notes_only()`: Renderizado de notas musicales
  - `draw_debug_info()`: Información de debug
- **Estructuras de apoyo**:
  - `AudioVisualMapping`: Mapeo entre audio y propiedades visuales
  - `EventVisualRules`: Reglas de visualización por tipo de evento

## 🔧 Patrones de Diseño Implementados

### Separación de Responsabilidades
- **Data**: `model.rs` contiene únicamente estructuras de datos
- **Input**: `audio.rs` maneja toda la entrada de datos OSC
- **Output**: `visual.rs` maneja toda la salida visual
- **Control**: `main.rs` coordina el flujo de la aplicación

### Modularidad
- Cada módulo exporta solo las funciones y tipos necesarios
- Uso apropiado de `pub` para visibilidad controlada
- Importaciones específicas para evitar contaminación de namespace

### Extensibilidad
- Fácil agregar nuevos tipos de eventos en `model.rs`
- Fácil agregar nuevos handlers OSC en `audio.rs`
- Fácil agregar nuevas visualizaciones en `visual.rs`

## 📡 Flujo de Datos

```
OSC Messages → audio.rs → model.rs → visual.rs → Screen
     ↑            ↓          ↓          ↓         ↓
  SuperCollider  Parse    Update     Render    Display
```

1. **Entrada**: SuperCollider envía mensajes OSC
2. **Procesamiento**: `audio.rs` parsea y actualiza el modelo
3. **Estado**: `model.rs` mantiene el estado de la aplicación
4. **Renderizado**: `visual.rs` convierte el estado en visualización
5. **Salida**: Nannou renderiza en pantalla

## 🚀 Ventajas de la Nueva Arquitectura

### Para Desarrollo
- **Mantenibilidad**: Cada módulo tiene una responsabilidad clara
- **Testabilidad**: Módulos independientes son fáciles de probar
- **Colaboración**: Equipos pueden trabajar en módulos específicos
- **Debugging**: Problemas están localizados por módulo

### Para Expansión
- **Nuevos tipos de audio**: Agregar en `audio.rs`
- **Nuevas visualizaciones**: Agregar en `visual.rs`
- **Nuevos tipos de datos**: Agregar en `model.rs`
- **Configuraciones**: Modificar solo `main.rs`

### Para Performance
- **Compilación**: Cambios modulares requieren menos recompilación
- **Runtime**: Separación clara permite optimizaciones específicas
- **Memoria**: Estado organizado permite mejor gestión de memoria

## 📝 Notas de Implementación

- **Compatibilidad**: Se mantiene compatibilidad con código existente via aliases
- **Migración**: Los módulos antiguos se renombraron a `*_modules` para referencia
- **Limpieza**: Las advertencias de compilación indican código no utilizado que puede removerse
- **Testing**: La aplicación compila y ejecuta correctamente con la nueva estructura

## 🔮 Próximos Pasos

1. **Limpieza**: Remover código no utilizado identificado por las advertencias
2. **Testing**: Implementar tests unitarios para cada módulo
3. **Documentación**: Expandir documentación inline en cada módulo
4. **Performance**: Optimizar renderizado basado en la nueva estructura modular
5. **Features**: Agregar nuevas funcionalidades usando la arquitectura establecida
