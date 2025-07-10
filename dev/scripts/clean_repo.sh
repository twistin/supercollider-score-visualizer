#!/bin/bash

# =============================================================================
# 🧹 LIMPIEZA AUTOMÁTICA DEL REPOSITORIO
# =============================================================================
# Este script limpia el repositorio dejando solo archivos esenciales

echo "🧹 Iniciando limpieza del repositorio..."

# Crear directorio temporal para archivos esenciales
mkdir -p temp_essential

# =============================================================================
# ARCHIVOS ESENCIALES A CONSERVAR
# =============================================================================

echo "📦 Respaldando archivos esenciales..."

# Código fuente del visualizador
cp -r src/ temp_essential/
cp Cargo.toml temp_essential/
cp config.toml temp_essential/

# Scripts SuperCollider esenciales (solo los mejores)
cp sc_auto_visualizer.scd temp_essential/
cp test_visualizer.scd temp_essential/
cp start_system.sh temp_essential/

# Documentación esencial
cp README.md temp_essential/
cp README_QUICK.md temp_essential/
cp LICENSE temp_essential/

# Git
cp -r .git temp_essential/

echo "✅ Archivos esenciales respaldados"

# =============================================================================
# ELIMINAR TODO LO DEMÁS
# =============================================================================

echo "🗑️ Eliminando archivos no esenciales..."

# Eliminar archivos de documentación redundante
rm -f *.md 2>/dev/null || true
rm -f ARQUITECTURA_* SISTEMA_* SOLUCION_* INICIO_* RESUMEN_* 2>/dev/null || true

# Eliminar scripts SuperCollider redundantes
rm -f *.scd 2>/dev/null || true

# Eliminar scripts de inicio redundantes
rm -f start*.sh verify*.sh 2>/dev/null || true

# Eliminar carpetas no esenciales
rm -rf backup/ docs/ scripts/ supercollider/ tests/ target/ 2>/dev/null || true

# Eliminar archivos específicos
rm -f .DS_Store fm.scd 2>/dev/null || true
rm -f *.py check_*.sh 2>/dev/null || true
rm -f audio_analyzer.rs lenguaje_visual_universal.rs live_coding_*.rs 2>/dev/null || true
rm -f main_old.rs main_refactored.rs musical_events.rs main.rs 2>/dev/null || true
rm -rf app/ audio/ config/ utils/ visual/ 2>/dev/null || true

echo "✅ Archivos no esenciales eliminados"

# =============================================================================
# RESTAURAR ARCHIVOS ESENCIALES
# =============================================================================

echo "📁 Restaurando archivos esenciales..."

# Mover archivos esenciales de vuelta
mv temp_essential/* ./
mv temp_essential/.git ./

# Limpiar directorio temporal
rm -rf temp_essential

echo "✅ Repositorio limpiado correctamente"

# =============================================================================
# CREAR ESTRUCTURA NUEVA
# =============================================================================

echo "🏗️ Creando nueva estructura..."

# Crear directorios necesarios
mkdir -p examples/
mkdir -p docs/

echo "✅ Nueva estructura creada"

echo ""
echo "🎯 LIMPIEZA COMPLETADA"
echo "📁 Archivos conservados:"
echo "   • src/ (código fuente Nannou)"
echo "   • Cargo.toml (configuración Rust)"
echo "   • config.toml (configuración visualizador)"
echo "   • sc_auto_visualizer.scd (SuperCollider auto-visualización)"
echo "   • test_visualizer.scd (test rápido)"
echo "   • start_system.sh (script de inicio)"
echo "   • README.md (documentación completa)"
echo "   • README_QUICK.md (guía rápida)"
echo "   • LICENSE (licencia)"
echo ""
echo "🚀 El repositorio está listo para live coding!"
