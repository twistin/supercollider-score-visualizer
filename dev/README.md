# 🔧 Directorio de Desarrollo - SC Score Visualizer

Este directorio contiene archivos de desarrollo, pruebas y documentación técnica que no son necesarios para el funcionamiento básico del visualizador.

## 📁 Estructura

### `docs/`
Documentación técnica del proceso de desarrollo:
- `AXES_SUMMARY.md` - Resumen de trabajo en ejes
- `CORRECCIONES_EJES_FINAL.md` - Correcciones finales de ejes
- `CORRECCIONES_REALIZADAS.md` - Historial de correcciones
- `ELIMINACION_RECTANGULOS.md` - Proceso de eliminación de rectángulos
- `ETIQUETAS_CORREGIDAS.md` - Correcciones de etiquetas
- `GRID_FEATURES.md` - Características de la grilla
- `GRID_IMPROVEMENTS.md` - Mejoras de la grilla
- `GUIA_PRUEBAS.md` - Guía de pruebas
- `MENU_COMPLETO.md` - Documentación del menú
- `MODERN_GRID.md` - Grilla moderna
- `README_QUICK.md` - Guía rápida alternativa

### `scripts/`
Scripts de desarrollo y pruebas:
- `clean_repo.sh` - Limpiar repositorio
- `demo_modern_grid.sh` - Demo de grilla moderna
- `enviar_prueba_osc.sh` - Enviar datos OSC de prueba
- `reiniciar_rust_analyzer.sh` - Reiniciar Rust Analyzer
- `start_system.sh` - Iniciar sistema completo
- `test_*.sh` - Varios scripts de pruebas específicas
- `test_visualizer.scd` - Archivo SuperCollider para pruebas

## 🎯 Propósito

Estos archivos fueron creados durante el proceso de desarrollo para:
- Documentar cambios y correcciones
- Probar funcionalidades específicas
- Mantener un historial de mejoras
- Facilitar el debugging y testing

## 📋 Uso

Para usar estos archivos de desarrollo:

```bash
# Ejecutar scripts de prueba
cd dev/scripts
./test_funcionalidades.sh

# Leer documentación técnica
cd dev/docs
cat CORRECCIONES_REALIZADAS.md
```

## 🧹 Mantenimiento

Esta carpeta puede eliminarse sin afectar el funcionamiento del visualizador principal. Es útil mantenerla durante el desarrollo activo.
