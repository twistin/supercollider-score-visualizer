#!/bin/bash

# =================================================================
# 🚀 INICIO RÁPIDO - SC SCORE VISUALIZER
# =================================================================
# Script para iniciar todo el sistema de visualización automática
# =================================================================

echo "🎵 SC Score Visualizer - Inicio Automático"
echo ""

# Verificar que el proyecto está compilado
if [ ! -f "./target/release/sc_score_visualizer" ]; then
    echo "⚠️ Visualizador no compilado. Compilando..."
    cargo build --release
    
    if [ $? -ne 0 ]; then
        echo "❌ Error al compilar. Abortando."
        exit 1
    fi
fi

# Iniciar el visualizador en background
echo "🖥️ Iniciando visualizador..."
./target/release/sc_score_visualizer &
VISUALIZER_PID=$!

# Esperar que se inicie
sleep 3

# Verificar que el proceso está corriendo
if kill -0 $VISUALIZER_PID 2>/dev/null; then
    echo "✅ Visualizador iniciado correctamente (PID: $VISUALIZER_PID)"
else
    echo "❌ Error al iniciar el visualizador"
    exit 1
fi

echo ""
echo "🎯 SISTEMA LISTO"
echo ""
echo "📋 SIGUIENTE PASOS:"
echo "   1. Abrir SuperCollider"
echo "   2. Cargar archivo: sc_auto_visualizer.scd"
echo "   3. Ejecutar bloques 1-6 (primera vez) o solo bloque 6 (uso diario)"
echo "   4. ¡Empezar live coding con visualización automática!"
echo ""
echo "🧪 TEST RÁPIDO:"
echo "   • Cargar y ejecutar: test_visualizer.scd"
echo ""
echo "🛑 PARAR SISTEMA:"
echo "   • kill $VISUALIZER_PID"
echo "   • O presionar Ctrl+C en esta terminal"
echo ""

# Mantener el script corriendo para mostrar logs
echo "📡 Visualizador activo. Presiona Ctrl+C para parar..."
wait $VISUALIZER_PID
