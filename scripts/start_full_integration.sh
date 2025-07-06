#!/bin/bash

# =============================================================================
# 🎵 INTEGRACIÓN COMPLETA SC SCORE VISUALIZER
# =============================================================================
# Script para ejecutar tanto SuperCollider como el visualizador Rust
# y probar la integración audiovisual en tiempo real
# =============================================================================

echo "🎵 INTEGRACIÓN COMPLETA SC SCORE VISUALIZER"
echo "==========================================="

# Verificar que el visualizador esté compilado
if [ ! -f "./target/release/sc_score_visualizer" ]; then
    echo "📦 Compilando visualizador..."
    cargo build --release
    if [ $? -ne 0 ]; then
        echo "❌ Error compilando el visualizador"
        exit 1
    fi
fi

echo "✅ Visualizador compilado"

# Detener cualquier proceso previo
echo "🧹 Limpiando procesos previos..."
pkill -f sc_score_visualizer 2>/dev/null

# Ejecutar el visualizador en background
echo "🚀 Iniciando visualizador Rust..."
./target/release/sc_score_visualizer &
VISUALIZER_PID=$!

# Esperar un momento para que se inicie
sleep 2

# Verificar que el visualizador esté corriendo
if ps -p $VISUALIZER_PID > /dev/null; then
    echo "✅ Visualizador ejecutándose (PID: $VISUALIZER_PID)"
else
    echo "❌ Error iniciando el visualizador"
    exit 1
fi

echo ""
echo "🎯 VISUALIZADOR LISTO PARA RECIBIR DATOS OSC"
echo "============================================"
echo "📡 Puerto OSC: 57124"
echo "🎯 Dirección: /realtime_audio"
echo ""
echo "📋 PRÓXIMOS PASOS EN SUPERCOLLIDER:"
echo "================================="
echo "1. Abrir SuperCollider IDE"
echo "2. Ejecutar:"
echo '   thisProcess.interpreter.executeFile("realtime_analysis_alt.scd".resolveRelative);'
echo "3. Para test con tono generado:"
echo "   ~testAnalysisAlt.();"
echo "4. Para análisis de micrófono:"
echo "   ~startAnalysisAlt.();"
echo "5. Para detener:"
echo "   ~stopTestAlt.(); // o ~stopAnalysisAlt.();"
echo ""
echo "🎨 CONTROLES DEL VISUALIZADOR:"
echo "============================="
echo "• Espacio - Pausar/Reanudar"
echo "• R - Reiniciar"
echo "• 1,2,3 - Cambiar estilos visuales"
echo "• ESC - Salir"
echo ""
echo "⚠️  Para detener todo:"
echo "   kill $VISUALIZER_PID"
echo "   o ejecutar: pkill -f sc_score_visualizer"
echo ""

# Función para detener todo al recibir Ctrl+C
cleanup() {
    echo ""
    echo "🛑 Deteniendo integración..."
    kill $VISUALIZER_PID 2>/dev/null
    echo "✅ Visualizador detenido"
    exit 0
}

trap cleanup INT

echo "🎵 SISTEMA LISTO - Presiona Ctrl+C para detener todo"
echo "=================================================="

# Mantener el script corriendo y mostrar estado
while ps -p $VISUALIZER_PID > /dev/null; do
    sleep 5
    echo "🟢 Visualizador activo ($(date '+%H:%M:%S'))"
done

echo "❌ El visualizador se detuvo inesperadamente"
