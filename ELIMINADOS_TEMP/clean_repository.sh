#!/bin/bash
# Script de limpieza del repositorio sc_score_visualizer
# Elimina archivos prescindibles manteniendo lo esencial

set -e  # Salir si hay errores

REPO_DIR="/Users/sdcarr/Documents/github/sc-score/sc_score_visualizer"
BACKUP_DIR="$REPO_DIR/ELIMINADOS_$(date +%Y%m%d_%H%M%S)"

echo "🗑️ LIMPIEZA DEL REPOSITORIO SC_SCORE_VISUALIZER"
echo "=============================================="
echo "📁 Directorio: $REPO_DIR"
echo "💾 Backup en: $BACKUP_DIR"
echo ""

# Crear directorio de backup
mkdir -p "$BACKUP_DIR"

# Función para mover archivos a backup
move_to_backup() {
    local file="$1"
    local reason="$2"
    if [ -f "$file" ]; then
        echo "🗑️ Moviendo: $(basename "$file") ($reason)"
        mv "$file" "$BACKUP_DIR/"
    elif [ -d "$file" ]; then
        echo "📁 Moviendo directorio: $(basename "$file") ($reason)"
        mv "$file" "$BACKUP_DIR/"
    fi
}

echo "🧪 ELIMINANDO ARCHIVOS DE PRUEBA Y TESTING..."
echo "---------------------------------------------"

# Archivos Python de prueba (prescindibles)
move_to_backup "$REPO_DIR/test_final_validation.py" "archivo de prueba OSC"
move_to_backup "$REPO_DIR/test_final.py" "archivo de prueba"
move_to_backup "$REPO_DIR/test_osc_client.py" "cliente OSC de prueba"
move_to_backup "$REPO_DIR/test_midi_virtual.py" "prueba MIDI virtual"
move_to_backup "$REPO_DIR/test_3_args.py" "prueba de argumentos OSC"
move_to_backup "$REPO_DIR/test_osc.py" "prueba OSC básica"
move_to_backup "$REPO_DIR/test_mixed_types.py" "prueba tipos mixtos"

# Scripts shell temporales (prescindibles)
move_to_backup "$REPO_DIR/diagnose_communication.sh" "diagnóstico temporal"
move_to_backup "$REPO_DIR/fix_function.sh" "script de arreglo temporal"
move_to_backup "$REPO_DIR/test_complete.sh" "script de prueba completa"
move_to_backup "$REPO_DIR/test_nc.sh" "script de prueba netcat"
move_to_backup "$REPO_DIR/test_cluster.sh" "script de prueba cluster"
move_to_backup "$REPO_DIR/brutal_clean.sh" "script de limpieza brutal"

echo ""
echo "📄 ELIMINANDO DOCUMENTACIÓN REDUNDANTE..."
echo "-----------------------------------------"

# Documentación redundante (mantener solo README.md principal)
move_to_backup "$REPO_DIR/README_NEW.md" "README duplicado"
move_to_backup "$REPO_DIR/GUIA_DRONE_CLUSTER.md" "guía específica prescindible"
move_to_backup "$REPO_DIR/MAPAS_PROFESIONALES_RESUMEN.md" "resumen prescindible"
move_to_backup "$REPO_DIR/SISTEMA_REACTIVO_COMPLETO.md" "documentación técnica prescindible"
move_to_backup "$REPO_DIR/CAMBIO_MODO_VISUAL_RESUMEN.md" "resumen prescindible"
move_to_backup "$REPO_DIR/MIDI_IMPLEMENTATION.md" "implementación MIDI duplicada"
move_to_backup "$REPO_DIR/GUIA_DRONES.md" "guía duplicada"
move_to_backup "$REPO_DIR/ESTADO_FINAL.md" "estado temporal"
move_to_backup "$REPO_DIR/VISUAL_NOTE_SYSTEM.md" "documentación interna"
move_to_backup "$REPO_DIR/LIMPIEZA_COMPLETADA.md" "documentación temporal"

echo ""
echo "🔧 ELIMINANDO ARCHIVOS DE CÓDIGO DUPLICADO..."
echo "---------------------------------------------"

# Archivos Rust duplicados/temporales
move_to_backup "$REPO_DIR/src/main_old_backup.rs" "backup antiguo del main"
move_to_backup "$REPO_DIR/src/main_refactored.rs" "versión refactorizada temporal"
move_to_backup "$REPO_DIR/src/main_clean.rs" "versión limpia temporal"
move_to_backup "$REPO_DIR/src/config_backup.rs" "backup de configuración"
move_to_backup "$REPO_DIR/src/audio_simple.rs" "versión simple prescindible"
move_to_backup "$REPO_DIR/src/osc_simple.rs" "OSC simple prescindible"
move_to_backup "$REPO_DIR/src/osc_complex.rs" "OSC complejo prescindible"
move_to_backup "$REPO_DIR/src/visual_backup.rs" "backup visual"
move_to_backup "$REPO_DIR/src/live_coding_main.rs" "main alternativo"

# Archivos de demostración
move_to_backup "$REPO_DIR/demo_professional_mapping.rs" "demo prescindible"
move_to_backup "$REPO_DIR/demo_professional_mapping_integration.rs" "demo prescindible"

echo ""
echo "📁 ELIMINANDO DIRECTORIOS DUPLICADOS..."
echo "---------------------------------------"

# Directorios duplicados
if [ -d "$REPO_DIR/src/visual_modules" ] && [ -d "$REPO_DIR/src/visual" ]; then
    move_to_backup "$REPO_DIR/src/visual_modules" "directorio visual duplicado"
fi

if [ -d "$REPO_DIR/src/audio_modules" ] && [ -d "$REPO_DIR/src/audio" ]; then
    move_to_backup "$REPO_DIR/src/audio_modules" "directorio audio duplicado"
fi

echo ""
echo "🧹 ELIMINANDO ARCHIVOS DEL SISTEMA..."
echo "------------------------------------"

# Archivos del sistema macOS
find "$REPO_DIR" -name ".DS_Store" -type f -delete 2>/dev/null || true
echo "🗑️ Eliminados archivos .DS_Store"

# Archivos temporales de Rust
find "$REPO_DIR" -name "*.tmp" -type f -delete 2>/dev/null || true
find "$REPO_DIR" -name "*.bak" -type f -delete 2>/dev/null || true
echo "🗑️ Eliminados archivos temporales"

echo ""
echo "✅ LIMPIEZA COMPLETADA"
echo "====================="
echo "📊 Archivos movidos a: $BACKUP_DIR"
echo "🔍 Revisa el directorio de backup antes de eliminarlo"
echo ""
echo "📁 ESTRUCTURA LIMPIA MANTENIDA:"
echo "   ✅ src/ - Código fuente principal"
echo "   ✅ Cargo.toml - Configuración del proyecto"
echo "   ✅ README.md - Documentación principal"
echo "   ✅ config.toml - Configuración de la aplicación"
echo "   ✅ start.sh - Script de inicio principal"
echo "   ✅ tests/ - Archivos de prueba esenciales (.scd)"
echo ""
echo "🗑️ Para eliminar definitivamente el backup:"
echo "   rm -rf '$BACKUP_DIR'"
echo ""
echo "💡 El proyecto ahora está limpio y organizado!"
