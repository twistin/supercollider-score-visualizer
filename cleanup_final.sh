#!/bin/bash
# Limpieza definitiva del repositorio SC Score Visualizer
# Elimina permanentemente archivos innecesarios

set -e

echo "🗑️ LIMPIEZA DEFINITIVA - ELIMINACIÓN PERMANENTE"
echo "==============================================="
echo "⚠️  ATENCIÓN: Los archivos se eliminarán SIN posibilidad de recuperación"
echo ""

REPO_DIR="/Users/sdcarr/Documents/github/sc-score/sc_score_visualizer"
cd "$REPO_DIR"

# Función para eliminar archivos
delete_file() {
    local file="$1"
    local reason="$2"
    if [ -f "$file" ]; then
        echo "🗑️ Eliminando: $(basename "$file") ($reason)"
        rm -f "$file"
    elif [ -d "$file" ]; then
        echo "📁 Eliminando directorio: $(basename "$file") ($reason)"
        rm -rf "$file"
    fi
}

echo "🧪 ELIMINANDO ARCHIVOS DE DEMOSTRACIÓN Y PRUEBA..."
echo "---------------------------------------------------"

# Archivo de demo actual (ya no necesario)
delete_file "demo_professional_mapping.rs" "archivo de demostración"

# Archivos SuperCollider de prueba
delete_file "sc_auto_visualizer.scd" "archivo SuperCollider de prueba"

echo ""
echo "📁 ELIMINANDO CARPETA DE ARCHIVOS TEMPORALES..."
echo "-----------------------------------------------"

# Eliminar toda la carpeta de archivos temporales
delete_file "ELIMINADOS_TEMP" "carpeta de archivos temporales"

echo ""
echo "🔧 ELIMINANDO ARCHIVOS RUST ALTERNATIVOS/BACKUP..."
echo "-------------------------------------------------"

# Archivos Rust alternativos en src/
delete_file "src/main_refactored.rs" "versión alternativa del main"
delete_file "src/main_clean.rs" "versión limpia alternativa"
delete_file "src/audio_simple.rs" "versión simple de audio"
delete_file "src/osc_simple.rs" "versión simple de OSC"
delete_file "src/osc_complex.rs" "versión compleja de OSC"
delete_file "src/visual_backup.rs" "backup del sistema visual"
delete_file "src/live_coding_main.rs" "main alternativo para live coding"

# Archivos de ejemplo/demostración en visual/
delete_file "src/visual/shader_example.rs" "ejemplo de shaders"
delete_file "src/visual/shader_hot_reload.rs" "hot reload de shaders"

echo ""
echo "🧹 ELIMINANDO ARCHIVOS DEL SISTEMA Y TEMPORALES..."
echo "-------------------------------------------------"

# Archivos del sistema macOS
find . -name ".DS_Store" -type f -delete 2>/dev/null || true
echo "🗑️ Eliminados archivos .DS_Store"

# Archivos temporales
find . -name "*.tmp" -type f -delete 2>/dev/null || true
find . -name "*.bak" -type f -delete 2>/dev/null || true
find . -name "*.backup" -type f -delete 2>/dev/null || true
echo "🗑️ Eliminados archivos temporales"

# Directorio venv si existe (se puede recrear)
delete_file "venv" "entorno virtual Python (se puede recrear)"

echo ""
echo "🎨 CONFIGURANDO VS CODE PARA OCULTAR ARCHIVOS INNECESARIOS..."
echo "------------------------------------------------------------"

# Crear/actualizar configuración de VS Code
mkdir -p .vscode

# Configuración de archivos a excluir en VS Code
cat > .vscode/settings.json << 'EOF'
{
    "files.exclude": {
        "**/target": true,
        "**/.git": true,
        "**/.DS_Store": true,
        "**/*.tmp": true,
        "**/*.bak": true,
        "**/*.backup": true,
        "**/node_modules": true,
        "**/.pytest_cache": true,
        "**/__pycache__": true,
        "**/venv": true,
        "**/env": true,
        "**/.env": true,
        "**/captures": true,
        "**/ELIMINADOS*": true,
        "**/temp*": true,
        "**/demo_*": true,
        "**/test_*.py": true,
        "**/test_*.sh": true,
        "**/*_backup.*": true,
        "**/*_old.*": true,
        "**/*_temp.*": true
    },
    "search.exclude": {
        "**/target": true,
        "**/captures": true,
        "**/.git": true,
        "**/node_modules": true,
        "**/venv": true
    },
    "files.watcherExclude": {
        "**/target/**": true,
        "**/.git/**": true,
        "**/node_modules/**": true,
        "**/venv/**": true
    }
}
EOF

echo "✅ Configuración de VS Code actualizada"

# Actualizar .gitignore para evitar archivos innecesarios
cat >> .gitignore << 'EOF'

# Archivos temporales y de prueba
*.tmp
*.bak
*.backup
*_temp.*
*_old.*
demo_*
test_*.py
test_*.sh

# Directorios temporales
ELIMINADOS*/
temp*/
captures/

# Entornos virtuales
venv/
env/
.env

# Archivos del sistema
.DS_Store
Thumbs.db

# Cachés de Python
__pycache__/
*.pyc
.pytest_cache/
EOF

echo "✅ .gitignore actualizado"

echo ""
echo "✅ LIMPIEZA DEFINITIVA COMPLETADA"
echo "================================="
echo ""
echo "📁 ESTRUCTURA FINAL (solo archivos esenciales):"
echo "   ✅ src/ - Código fuente de la aplicación"
echo "   ✅ Cargo.toml - Configuración del proyecto Rust"
echo "   ✅ config.toml - Configuración de la aplicación"
echo "   ✅ README.md - Documentación principal"
echo "   ✅ LICENSE - Licencia del proyecto"
echo "   ✅ .gitignore - Control de versiones"
echo "   ✅ .vscode/ - Configuración VS Code"
echo ""
echo "🚫 ARCHIVOS ELIMINADOS PERMANENTEMENTE:"
echo "   ❌ Todos los archivos de demostración"
echo "   ❌ Todos los archivos de prueba temporales"
echo "   ❌ Todas las versiones alternativas de código"
echo "   ❌ Todos los archivos backup"
echo "   ❌ Todos los archivos del sistema"
echo ""
echo "🎯 VS CODE YA NO MOSTRARÁ:"
echo "   - Archivos de demostración (demo_*)"
echo "   - Archivos de prueba (test_*)"
echo "   - Archivos temporales (*.tmp, *.bak)"
echo "   - Directorio target/ de Rust"
echo "   - Archivos del sistema (.DS_Store)"
echo "   - Entornos virtuales (venv/)"
echo ""
echo "✨ El repositorio está ahora completamente limpio y optimizado"
echo "   para desarrollo profesional sin distracciones!"
