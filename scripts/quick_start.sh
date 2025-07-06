#!/bin/bash
# Quick Start Script para SC Score Visualizer Universal
# Este script facilita el inicio rápido del sistema

echo "🎵 SC Score Visualizer Universal - Quick Start"
echo "=============================================="
echo ""

# Verificar que estamos en el directorio correcto
if [ ! -f "Cargo.toml" ]; then
    echo "❌ Error: Ejecuta este script desde el directorio del proyecto"
    echo "   cd /Users/sdcarr/Documents/github/sc-score/sc_score_visualizer"
    exit 1
fi

echo "📁 Directorio: $(pwd)"
echo ""

# Mostrar opciones
echo "Selecciona una opción:"
echo "1) 🚀 Iniciar visualizador"
echo "2) 🧪 Ejecutar test OSC"
echo "3) 🔧 Verificar compilación"
echo "4) 📄 Mostrar configuración"
echo "5) 📚 Mostrar instrucciones SuperCollider"
echo ""

read -p "Opción (1-5): " choice

case $choice in
    1)
        echo "🚀 Iniciando SC Score Visualizer..."
        echo "   Usando puerto definido en config.toml (debería ser 57124)"
        echo "   Presiona Ctrl+C para detener"
        echo ""
        cargo run --release
        ;;
    2)
        echo "🧪 Ejecutando test de conexión OSC..."
        python3 test_osc.py
        echo ""
        echo "✅ Test completado. Si el visualizador está corriendo,"
        echo "   deberías ver eventos en la ventana gráfica."
        ;;
    3)
        echo "🔧 Verificando compilación..."
        cargo check
        if [ $? -eq 0 ]; then
            echo ""
            echo "✅ Compilación exitosa"
        else
            echo ""
            echo "❌ Errores de compilación encontrados"
        fi
        ;;
    4)
        echo "📄 Configuración actual:"
        echo ""
        grep -A 2 -B 2 "port = " config.toml
        echo ""
        grep -A 2 -B 2 "mode = " config.toml
        ;;
    5)
        echo "📚 Instrucciones para SuperCollider:"
        echo ""
        echo "1. Inicia SuperCollider"
        echo "2. Ejecuta: s.boot;"
        echo "3. Ejecuta: (\"supercollider_examples.scd\").loadRelative;"
        echo "4. Espera el mensaje: '✓ TODAS LAS FUNCIONES CARGADAS EXITOSAMENTE'"
        echo "5. Prueba con: scvTestBasicEvents.();"
        echo ""
        echo "Funciones disponibles:"
        echo "- scvTestBasicEvents.()          // Prueba básica"
        echo "- scvXenakisComposition.()       // Composición completa"
        echo "- scvMetastasisPoints.()         // Puntos dispersos"
        echo "- scvPithopraktaGliss.()         // Glissandi"
        echo "- scvStochasticClouds.()         // Clusters estocásticos"
        echo "- scvNoiseTextures.()            // Texturas de ruido"
        echo "- scvSpectralMasses.()           // Masas sonoras"
        echo "- scvRhythmicPatterns.()         // Patrones rítmicos"
        echo ""
        echo "Funciones directas:"
        echo "- scvSendPoint.(freq, amp, dur)      // Evento puntual"
        echo "- scvSendGliss.(f1, f2, amp, dur)    // Glissando"
        echo "- scvSendCluster.(freq, amp, dur)    // Cluster"
        echo "- scvSendNoise.(fc, bw, amp, dur)    // Ruido"
        echo "- scvSendSoundMass.(freq, amp, dur)  // Masa sonora"
        ;;
    *)
        echo "❌ Opción inválida"
        exit 1
        ;;
esac

echo ""
echo "📖 Para más información, consulta:"
echo "   - README.md"
echo "   - ESTADO_VERIFICADO_FINAL.md"
echo "   - docs/SISTEMA_UNIVERSAL_FINAL.md"
