#!/bin/bash
# =============================================
# SC Score Visualizer - Inicio Rápido
# =============================================

echo "🎨 SC Score Visualizer - Inicio Rápido"
echo "======================================"
echo ""

# Verificar Rust
if ! command -v cargo &> /dev/null; then
    echo "❌ Rust no encontrado. Instalar desde: https://rustup.rs/"
    exit 1
fi

# Compilar si es necesario
if [ ! -f "target/release/sc_score_visualizer" ]; then
    echo "🔧 Compilando visualizador..."
    cargo build --release
fi

echo "✅ Todo listo!"
echo ""
echo "🎵 PASOS PARA USAR:"
echo "1. Abrir SuperCollider"
echo "2. Ejecutar: \"realtime_analysis.scd\".loadDocument.front.execute;"
echo "3. Crear audio en SuperCollider (cualquier sonido)"
echo "4. ¡Ver la visualización en tiempo real!"
echo ""
echo "🚀 Iniciando visualizador..."
cargo run --release
