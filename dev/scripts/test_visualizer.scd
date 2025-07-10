// =================================================================
// TEST SUPER SIMPLE DEL SISTEMA DE AUTO-VISUALIZACIÓN
// =================================================================
// 
// Ejecutar este archivo para probar que el sistema funciona
// REQUISITO: El visualizador debe estar ejecutándose
// 
// =================================================================

// PASO 1: Configuración mínima
(
~testAddr = NetAddr("127.0.0.1", 57124);
"✅ Dirección OSC configurada".postln;
)

// PASO 2: Test básico de conectividad
(
~testAddr.sendMsg("/event", "test", 440.0, 0.5, 1.0, 0.5, 0.0, 440.0);
"🧪 Mensaje de prueba enviado".postln;
)

// PASO 3: Test con secuencia de notas
(
fork {
    var freqs = [220, 330, 440, 550, 660];
    freqs.do { |freq, i|
        ~testAddr.sendMsg("/event", "sequence", freq.asFloat, 0.6, 0.5, 0.3, 0.0, freq.asFloat);
        "🎵 Nota enviada: %Hz".format(freq).postln;
        0.3.wait;
    };
    "🎊 Secuencia de test completada!".postln;
};
)
