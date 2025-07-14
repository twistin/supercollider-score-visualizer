#!/bin/bash
# Script de inicio para SC Score Visualizer

echo "🎵 SC Score Visualizer v2.0"
echo "==============================================="

# Verificar que Rust está instalado
if ! command -v cargo &> /dev/null; then
    echo "❌ Error: Cargo no está instalado. Instala Rust desde https://rustup.rs/"
    exit 1
fi

# Verificar dependencias del sistema
echo "🔍 Verificando dependencias del sistema..."

# Verificar ALSA (Linux)
if [[ "$OSTYPE" == "linux-gnu"* ]]; then
    if ! pkg-config --exists alsa; then
        echo "⚠️  ALSA no detectado. Instalando..."
        sudo apt-get update
        sudo apt-get install -y libasound2-dev pkg-config
    fi
fi

# Compilar y ejecutar
echo "🔧 Compilando proyecto..."
cargo build --release

if [ $? -eq 0 ]; then
    echo "✅ Compilación exitosa"
    echo "🚀 Iniciando SC Score Visualizer..."
    cargo run --release
else
    echo "❌ Error en la compilación"
    exit 1
fi