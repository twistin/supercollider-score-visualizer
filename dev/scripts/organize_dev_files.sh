#!/bin/bash

# Script para organizar archivos de desarrollo
# Uso: ./organize_dev_files.sh

echo "🧹 Organizando archivos de desarrollo..."

# Crear estructura de carpetas si no existe
mkdir -p dev/docs dev/scripts dev/backups

# Mover archivos .md de documentación técnica
if ls *.md 2>/dev/null | grep -v "^README.md$" > /dev/null; then
    echo "📄 Moviendo archivos de documentación..."
    mv *.md dev/docs/ 2>/dev/null || true
    # Mantener README.md en la raíz
    mv dev/docs/README.md ./ 2>/dev/null || true
fi

# Mover scripts de prueba y desarrollo
if ls *.sh 2>/dev/null > /dev/null; then
    echo "📜 Moviendo scripts de desarrollo..."
    mv *.sh dev/scripts/ 2>/dev/null || true
fi

# Mover archivos SuperCollider de prueba (excepto el principal)
if ls *.scd 2>/dev/null | grep -v "^sc_auto_visualizer.scd$" > /dev/null; then
    echo "🎵 Moviendo archivos SuperCollider de prueba..."
    for file in *.scd; do
        if [ "$file" != "sc_auto_visualizer.scd" ]; then
            mv "$file" dev/scripts/ 2>/dev/null || true
        fi
    done
fi

# Mostrar estructura final
echo ""
echo "✅ Organización completada!"
echo "📁 Estructura del proyecto:"
echo "Raíz limpia:"
ls -la | grep -E "\.(toml|md|scd|lock)$" | grep -v "^d"
echo ""
echo "📂 Archivos de desarrollo:"
echo "dev/docs/: $(ls dev/docs/ 2>/dev/null | wc -l) archivos"
echo "dev/scripts/: $(ls dev/scripts/ 2>/dev/null | wc -l) archivos"
echo ""
echo "🎯 Proyecto listo para producción!"
