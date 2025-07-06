#!/bin/bash
# Script de limpieza del directorio SC Score Visualizer
# Mantiene solo archivos esenciales para operación

echo "🧹 Limpiando directorio SC Score Visualizer..."
echo "Manteniendo solo archivos esenciales..."

# Archivos a mantener (esenciales)
KEEP_FILES=(
    "Cargo.toml"
    "Cargo.lock" 
    "config.toml"
    "README.md"
    "supercollider_clean.scd"
    "quick_start.sh"
    "cleanup.sh"
    "src/"
    "docs/"
    "target/"
    "archived_supercollider/"
    "archived_docs/"
)

# Crear backup de archivos importantes por si acaso
echo "📦 Creando backup de archivos de documentación..."
mkdir -p backup_docs
cp *.md backup_docs/ 2>/dev/null || true
cp *.scd backup_docs/ 2>/dev/null || true
cp *.py backup_docs/ 2>/dev/null || true

echo "🗑️  Removiendo archivos no esenciales..."

# Remover archivos de demo/test obsoletos
rm -f demo_*.scd
rm -f test_*.scd test_*.py 
rm -f supercollider_examples_*.scd
rm -f supercollider_audiovisual.scd
rm -f check_sc_syntax.py
rm -f verify_integration.py
rm -f start_visualizer.sh
rm -f test.scd
rm -f lenguaje_visual_universal.toml

# Remover documentación duplicada/obsoleta de correcciones
rm -f CORRECCION_*.md
rm -f ARCHIVO_CORREGIDO.md
rm -f ESTADO_FINAL.md
rm -f ESTADO_VERIFICADO_FINAL.md  
rm -f RESUMEN_EJECUTIVO_FINAL.md
rm -f RESUMEN_CORRECCIONES_FINALES.md
rm -f README_FINAL.md
rm -f GUIA_USO.md
rm -f INICIO_RAPIDO.md
rm -f MEJORAS.md
rm -f SOLUCION_*.md
rm -f VARIABLES_CORREGIDAS.md

# Remover test OSC duplicado
rm -f test_osc_connection.py

echo "✅ Limpieza completada!"
echo ""
echo "📁 Archivos mantenidos (esenciales):"
ls -la | grep -E "\.(toml|rs|scd|py|sh|md)$|^d" | head -20

echo ""
echo "📦 Backup creado en: backup_docs/"
echo "🎯 Directorio limpio y listo para uso!"
