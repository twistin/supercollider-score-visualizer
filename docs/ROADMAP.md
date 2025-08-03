# 🗺️ Roadmap de SC Score Visualizer

Este documento recoge tareas pendientes, refactorizaciones y pruebas previstas para futuras versiones.

---

## 🎯 Funcionalidades por implementar

- Captura de frames con tecla `S`
- Activar tecla de acceso rápido a modo visual "X"
- Migrar uso de `visual_quality` a una API uniforme
- Sustituir formas `Circle`, `Rect`, etc. por sistema `Shape`
- Consolidar `Xenakis mode` con `Legacy rendering`
- Mapeo consistente de `amp`, `duration`, `freq` en visualizaciones
- Afinar conversión a color: logarítmico, perceptual, paletas artísticas

---

## 🔧 Refactorización pendiente

- Fusionar `renderer.rs` y `view.rs`
- Sustituir `VisualNote` por struct más simple con traits
- Usar traits `Renderable`, `Mappable`, etc.
- Revisar y simplificar estructura del `Model`
- Modularización: separar `osc`, `midi`, `core`, `visual`, `audio`

---

## 🧪 Testing y optimización

- Añadir `#[cfg(test)]` para módulos clave
- Benchmark de renderizado por calidad
- Verificación de recepción OSC (puertos dinámicos)
- Simulación de carga de eventos en tiempo real
- Evaluar rendimiento en equipos de gama baja

---

✅ Última revisión: julio 2025
