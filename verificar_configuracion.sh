#!/bin/bash

# =================================================================
# VERIFICACIÓN DE CONFIGURACIÓN - SC Score Visualizer
# =================================================================

echo "🔍 Verificando configuración de SC Score Visualizer..."
echo "====================================================="
echo ""

# Función para mostrar OK o ERROR
check_result() {
    if [ $1 -eq 0 ]; then
        echo "✅ $2"
    else
        echo "❌ $2"
        return 1
    fi
}

# Verificar Rust
echo "🦀 Verificando Rust..."
rustc --version > /dev/null 2>&1
check_result $? "Rust instalado"

cargo --version > /dev/null 2>&1
check_result $? "Cargo disponible"

# Verificar dependencias del sistema
echo ""
echo "🔧 Verificando dependencias del sistema...S"

if command -v pkg-config > /dev/null 2>&1; then
    echo "✅ pkg-config disponible"
else
    echo "❌ pkg-config no encontrado"
    echo "   Instalar con: sudo apt-get install pkg-config"
fi

# Verificar ALSA (Linux)
if [[ "$OSTYPE" == "linux-gnu"* ]]; then
    if pkg-config --exists alsa 2>/dev/null; then
        echo "✅ ALSA disponible"
    else
        echo "❌ ALSA no encontrado"
        echo "   Instalar con: sudo apt-get install libasound2-dev"
    fi
fi

# Verificar SuperCollider
echo ""
echo "🎵 Verificando SuperCollider..."
if command -v sclang > /dev/null 2>&1; then
    echo "✅ SuperCollider (sclang) disponible"
    sclang --version 2>&1 | head -1
else
    echo "❌ SuperCollider no encontrado"
    echo "   Instalar SuperCollider desde: https://supercollider.github.io/"
fi

# Verificar compilación del proyecto
echo ""
echo "🏗️ Verificando compilación del proyecto..."
if [ -f "./target/release/sc_score_visualizer" ]; then
    echo "✅ Binario compilado encontrado"
else
    echo "⚠️ Binario no encontrado, intentando compilar..."
    cargo build --release --quiet
    if [ $? -eq 0 ]; then
        echo "✅ Compilación exitosa"
    else
        echo "❌ Error en compilación"
        exit 1
    fi
fi

# Verificar archivos de configuración
echo ""
echo "📁 Verificando archivos de configuración..."
[ -f "config.toml" ] && echo "✅ config.toml encontrado" || echo "❌ config.toml no encontrado"
[ -f "setup_visualizer.scd" ] && echo "✅ setup_visualizer.scd encontrado" || echo "❌ setup_visualizer.scd no encontrado"
[ -f "test_conexion.scd" ] && echo "✅ test_conexion.scd encontrado" || echo "❌ test_conexion.scd no encontrado"
[ -f "ejemplos_live_coding.scd" ] && echo "✅ ejemplos_live_coding.scd encontrado" || echo "❌ ejemplos_live_coding.scd no encontrado"

# Verificar puerto OSC
echo ""
echo "🌐 Verificando configuración OSC..."
# El puerto se lee de config.toml. Se asume 57124 es el valor deseado por defecto.
# Si el config.toml tiene otro puerto, se usará ese.
PORT=$(grep 'listen_port' config.toml | grep -o '[0-9]*' 2>/dev/null || echo "57124")
echo "📡 Puerto OSC configurado: $PORT"

if lsof -i :$PORT > /dev/null 2>&1; then
    echo "⚠️ Puerto $PORT ya está en uso"
    echo "   Proceso: $(lsof -i :$PORT -t | head -1)"
else
    echo "✅ Puerto $PORT disponible"
fi

# Verificar documentación
echo ""
echo "📚 Verificando documentación..."
[ -f "docs/GUIA_LIVE_CODING.md" ] && echo "✅ Guía de live coding encontrada" || echo "❌ Guía de live coding no encontrada"
[ -f "README.md" ] && echo "✅ README.md encontrado" || echo "❌ README.md no encontrado"
[ -f "ARCHITECTURE.md" ] && echo "✅ ARCHITECTURE.md encontrado" || echo "❌ ARCHITECTURE.md no encontrado"
[ -f "MAPAS_PROFESIONALES_RESUMEN.md" ] && echo "✅ MAPAS_PROFESIONALES_RESUMEN.md encontrado" || echo "❌ MAPAS_PROFESIONALES_RESUMEN.md no encontrado"


# Ejecutar tests
echo ""
echo "🧪 Ejecutando tests..."
cargo test --quiet > /dev/null 2>&1
check_result $? "Tests del proyecto"

# Resumen
echo ""
echo "📋 RESUMEN DE VERIFICACIÓN"
echo "=========================="
echo ""
echo "Para iniciar una sesión de live coding:"
echo "1. Ejecutar: ./start_visualizer.sh"
echo "2. Abrir SuperCollider"
echo "3. Cargar: \"setup_visualizer.scd\".load;"
echo "4. Probar: \"test_conexion.scd\".load;"
echo ""
echo "📖 Documentación completa: docs/GUIA_LIVE_CODING.md"
echo ""

echo "🎵 Verificación completada!"
