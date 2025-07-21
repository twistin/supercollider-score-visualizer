#!/bin/bash

# =================================================================
# INICIO RÁPIDO - SC Score Visualizer
# =================================================================

echo "🎵 SC Score Visualizer - Inicio Rápido"
echo "====================================="
echo ""

# Verificar que el proyecto esté compilado
if [ ! -f "./target/release/sc_score_visualizer" ]; then
    echo "🔧 Compilando proyecto..."
    cargo build --release --quiet
    if [ $? -eq 0 ]; then
        echo "✅ Compilación exitosa"
    else
        echo "❌ Error en compilación"
        exit 1
    fi
fi

# Verificar puerto OSC
# El puerto se lee de config.toml, por defecto 57124 según README.md
PORT=$(grep 'listen_port' config.toml | grep -o '[0-9]*' 2>/dev/null || echo "57124")
if lsof -i :$PORT > /dev/null 2>&1; then
    echo "⚠️  Puerto $PORT ya está en uso"
    echo "🛑 Terminando procesos previos..."
    pkill -f sc_score_visualizer
    sleep 2
fi

# Ejecutar visualizador
echo "🚀 Iniciando visualizador en puerto $PORT..."
echo ""
echo "🎹 Controles disponibles:"
echo "   Espacio - Pausar/reanudar visualización"
echo "   R - Reiniciar visualización"
echo "   D - Toggle información de debug"
echo "   G - Toggle grilla"
echo "   H - Mostrar/ocultar ayuda"
echo "   P - Captura manual (guarda eventos actuales)"
echo "   F - Toggle pantalla completa"
echo "   Esc - Salir"
echo ""
echo "📋 Siguiente paso: Abrir SuperCollider y ejecutar:"
echo "   \"setup_visualizer.scd\".load;"
echo ""
echo "🧪 Para probar comunicación rápida:"
echo "   \"test_conexion.scd\".load;"
echo ""
echo "📚 Para ver ejemplos completos:"
echo "   \"ejemplos_live_coding.scd\".load;"
echo ""
echo "📖 Documentación completa: docs/GUIA_LIVE_CODING.md"
echo ""

# Ejecutar visualizador
./target/release/sc_score_visualizer
