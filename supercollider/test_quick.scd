// =============================================================================
// 🧪 TEST RÁPIDO - Comunicación OSC
// =============================================================================
// Test mínimo para verificar que funciona la comunicación básica
// =============================================================================

(
"🚀 Iniciando test rápido de comunicación OSC...".postln;

// Configurar cliente OSC hacia el visualizador
~visualizer = NetAddr("127.0.0.1", 57124);

// Test 1: Enviar mensaje de prueba directo
"📡 Enviando mensaje de prueba...".postln;
~visualizer.sendMsg("/realtime_audio", 
    440.0,    // pitch
    0.5,      // amp  
    1.0,      // onset
    1.0,      // hasFreq
    1000.0,   // centroid
    0.3,      // flux
    2000.0,   // rolloff
    0.2,      // flatness
    0.8,      // harmonicity
    0.2,      // noisiness
    -0.5      // spectralSlope
);

"✅ Mensaje enviado. Deberías ver actividad en el visualizador.".postln;
"";
)

// Test 2: Simulación de glissando
(
"🎼 Simulando glissando 220Hz -> 880Hz en 4 segundos...".postln;

Routine({
    var startTime = SystemClock.seconds;
    var duration = 4.0;
    
    while { (SystemClock.seconds - startTime) < duration } {
        var progress = (SystemClock.seconds - startTime) / duration;
        var freq = 220 + (660 * progress); // 220 -> 880 Hz
        var amp = 0.8 * (1 - progress); // Fade out
        
        ~visualizer.sendMsg("/realtime_audio",
            freq,     // pitch aumentando
            amp,      // amp disminuyendo  
            if(progress < 0.1, 1.0, 0.0), // onset solo al inicio
            1.0,      // hasFreq siempre
            freq * 2, // centroid proporcional
            0.1,      // flux bajo (tono puro)
            freq * 3, // rolloff
            0.1,      // flatness baja (tonal)
            0.9,      // harmonicity alta
            0.1,      // noisiness baja
            0.2       // spectralSlope
        );
        
        0.02.wait; // 50 Hz (cada 20ms)
    };
    
    "✅ Glissando simulado completado!".postln;
}).play;
)

// Test 3: Simulación de onsets rítmicos
(
"🥁 Simulando patrón rítmico...".postln;

Routine({
    8.do { |i|
        "Onset #".post; (i+1).postln;
        
        // Onset marcado
        ~visualizer.sendMsg("/realtime_audio",
            [220, 330, 440, 550].wrapAt(i), // Pitch variado
            0.8,      // amp alta
            1.0,      // onset TRUE
            1.0,      // hasFreq
            1200.0,   // centroid
            0.8,      // flux alto (cambio)
            2500.0,   // rolloff
            0.3,      // flatness media
            0.7,      // harmonicity
            0.3,      // noisiness
            0.1       // spectralSlope
        );
        
        0.1.wait; // Esperar 100ms
        
        // Silencio entre onsets
        ~visualizer.sendMsg("/realtime_audio",
            0,        // sin pitch
            0.01,     // amp mínima
            0.0,      // sin onset
            0.0,      // sin hasFreq
            500.0,    // centroid bajo
            0.1,      // flux bajo
            800.0,    // rolloff bajo
            0.9,      // flatness alta (ruido)
            0.1,      // harmonicity baja
            0.9,      // noisiness alta
            -0.8      // spectralSlope negativo
        );
        
        0.4.wait; // Pausa entre notas
    };
    
    "✅ Patrón rítmico completado!".postln;
}).play;
)

// Test 4: Audio real con análisis
(
"🎵 Iniciando análisis de audio real...".postln;
"   Carga primero realtime_analysis.scd si no está cargado.".postln;

// Verificar si el servidor está corriendo
if(Server.default.serverRunning.not) {
    "❌ Servidor no está corriendo. Ejecuta: Server.default.boot".postln;
} {
    "✅ Servidor corriendo, iniciando analizador...".postln;
    ~analyzer = Synth(\RealtimeAnalyzer);
    "📡 Analizador activo. Haz sonidos para ver visualización!".postln;
};
)

// Para detener el analizador
(
~analyzer.free;
"🛑 Analizador detenido.".postln;
)
