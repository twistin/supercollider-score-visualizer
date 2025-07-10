#!/bin/bash

# =================================================================
# 🎵 SC SCORE VISUALIZER - DATOS DE PRUEBA OSC
# =================================================================

echo "📡 Enviando datos de prueba OSC al puerto 57124..."

# Verificar si oscsend está disponible
if ! command -v oscsend &> /dev/null; then
    echo "⚠️ oscsend no está disponible. Instala liblo-tools:"
    echo "   brew install liblo"
    echo ""
    echo "🎵 La aplicación seguirá funcionando sin datos OSC"
    echo "   Puedes probar las funcionalidades de menú, tema y grilla"
    echo ""
    exit 0
fi

echo "✅ oscsend encontrado, enviando datos de prueba..."

# Función para enviar eventos musicales
send_note() {
    local freq=$1
    local amp=$2
    oscsend localhost 57124 /note ff $freq $amp 2>/dev/null
    echo "🎵 Nota: ${freq}Hz, ${amp}amp"
}

# Enviar algunas notas de prueba
echo "🎶 Enviando secuencia de notas..."
send_note 440.0 0.8  # A4
sleep 0.5
send_note 523.25 0.7 # C5
sleep 0.5
send_note 659.25 0.6 # E5
sleep 0.5
send_note 783.99 0.5 # G5
sleep 0.5

echo "✅ Datos de prueba enviados"
echo "🎯 Si ves eventos en la visualización, OSC está funcionando"
