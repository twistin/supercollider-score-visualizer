#!/bin/bash

# =================================================================
# INICIO RÁPIDO - SC Score Visualizer
# =================================================================

echo "🎵 SC Score Visualizer - Inicio Rápido"
echo "====================================="
echo ""

# Verificar que el proyecto esté compilado
if [ ! -f "./target/release/sc_score_visualizer" ]; then
    echo "� Compilando proyecto..."
    cargo build --release --quiet
    if [ $? -eq 0 ]; then
        echo "✅ Compilación exitosa"
    else
        echo "❌ Error en compilación"
        exit 1
    fi
fi

# Verificar puerto OSC
PORT=$(grep 'port' config.toml | grep -o '[0-9]*' 2>/dev/null || echo "57124")
if lsof -i :$PORT > /dev/null 2>&1; then
    echo "⚠️ Puerto $PORT ya está en uso"
    echo "🛑 Terminando procesos previos..."
    pkill -f sc_score_visualizer
    sleep 2
fi

# Ejecutar visualizador
echo "🚀 Iniciando visualizador en puerto $PORT..."
echo ""
echo "🎹 Controles disponibles:"
echo "   G - Mostrar/ocultar grilla"
echo "   S - Mostrar/ocultar estadísticas"
echo "   P - Pausar/reanudar"
echo "   TAB - Menú completo"
echo "   1-4 - Cambiar temas"
echo "   ESC - Salir"
echo ""
echo "📋 Siguiente paso: Abrir SuperCollider y cargar 'sc_auto_visualizer.scd'"
echo "🧪 Para probar comunicación: cargar 'test_communication.scd'"
echo ""

# Ejecutar visualizador
./target/release/sc_score_visualizer
echo "Para verificar la integración, ejecuta: python3 verify_integration.py"
echo
echo "Presiona Ctrl+C para detener"
echo "=" * 60

# Ejecutar el visualizador
exec "$BINARY"
