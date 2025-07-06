#!/bin/bash

echo "🎵 === INTEGRACIÓN COMPLETA SuperCollider + Visualizador ==="

# Verificar que SuperCollider esté disponible
if ! command -v sclang &> /dev/null; then
    echo "❌ SuperCollider no encontrado. Asegúrate de que esté instalado."
    exit 1
fi

echo "✅ SuperCollider encontrado"

# Verificar que el visualizador esté compilado
if [ ! -f "./target/release/sc_score_visualizer" ]; then
    echo "🔧 Compilando visualizador..."
    cargo build --release
fi

echo "✅ Visualizador compilado"

# Detener procesos anteriores
echo "🛑 Deteniendo procesos anteriores..."
pkill -f sc_score_visualizer
pkill -f sclang

# Esperar un momento
sleep 2

# Iniciar el visualizador
echo "🚀 Iniciando visualizador en background..."
./target/release/sc_score_visualizer &
VISUALIZER_PID=$!

# Esperar que el visualizador se inicialice
sleep 3

# Verificar que el visualizador esté ejecutándose
if ! ps -p $VISUALIZER_PID > /dev/null; then
    echo "❌ Error: El visualizador no pudo iniciarse"
    exit 1
fi

echo "✅ Visualizador ejecutándose (PID: $VISUALIZER_PID)"

# Crear script temporal de SuperCollider que envía mensajes de prueba
cat > temp_test.scd << 'EOF'
(
"🎵 Iniciando test de integración visual...".postln;

// Configurar dirección OSC
~target = NetAddr("127.0.0.1", 57124);

// Función para enviar datos de prueba
~sendTestData = {
    var pitch, onset, centroid, mfcc0, mfcc1, rolloff, zcr, loudness;
    
    // Generar datos de prueba variados
    pitch = 200 + (800 * rand);  // 200-1000 Hz
    onset = [0, 1].choose;       // Onset aleatorio
    centroid = 500 + (2000 * rand); // Centroide espectral
    mfcc0 = rand;
    mfcc1 = rand;
    rolloff = 1000 + (3000 * rand);
    zcr = rand;
    loudness = rand;
    
    // Enviar mensaje
    ~target.sendMsg("/realtime_audio", pitch, onset, centroid, mfcc0, mfcc1, rolloff, zcr, loudness);
    
    // Mostrar lo que se envió
    ("Enviado -> pitch:" + pitch.round(1) + " onset:" + onset + " centroid:" + centroid.round(1)).postln;
};

// Enviar mensajes cada 200ms durante 10 segundos
~testTask = Task({
    50.do {
        ~sendTestData.();
        0.2.wait;
    };
    "✅ Test de 10 segundos completado".postln;
}).start;
)
EOF

echo "📡 Enviando datos de prueba desde SuperCollider..."

# Ejecutar el script de SuperCollider
sclang temp_test.scd

echo "🏁 Test completado. El visualizador sigue ejecutándose."
echo "Para detenerlo: pkill -f sc_score_visualizer"

# Limpiar archivo temporal
rm -f temp_test.scd

echo "✅ Integración completa terminada"
