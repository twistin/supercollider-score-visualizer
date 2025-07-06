// 🎯 DIAGNÓSTICO VISUAL - Test específico para problemas de renderizado
// Puerto OSC: 57124

"🔍 Iniciando diagnóstico visual...".postln;

// Configuración OSC
~osc = NetAddr.new("127.0.0.1", 57124);
"📡 OSC configurado en puerto 57124".postln;

// ========== TESTS VISUALES ESPECÍFICOS ==========

~testVisualSimple = {
    "🧪 === TEST VISUAL SIMPLE - UN SOLO PUNTO ===".postln;
    // Un punto que dura mucho tiempo para ver si llega al final
    ~osc.sendMsg("/event", "point", 440, 0.8, 10.0, 0.1, 0.1, 0.5, 0.0, 0.0, 220);
    "✅ Punto enviado: 440Hz, 10 segundos de duración".postln;
    "👀 OBSERVA: ¿El punto llega al final de la pantalla?".postln;
};

~testVisualGliss = {
    "🧪 === TEST VISUAL GLISSANDO - TRAYECTORIA COMPLETA ===".postln;
    // Glissando lento para seguir toda la trayectoria
    ~osc.sendMsg("/event", "gliss", 220, 880, 0.7, 8.0, 0, 0.1, 0.1, 0.7, 0.2, 0.1, 120);
    "✅ Glissando enviado: 220→880Hz, 8 segundos".postln;
    "👀 OBSERVA: ¿La línea va de izquierda a derecha completamente?".postln;
};

~testVisualLargo = {
    "🧪 === TEST VISUAL DURACIÓN LARGA ===".postln;
    // Evento muy largo para probar los límites del sistema
    ~osc.sendMsg("/event", "point", 660, 0.6, 15.0, 0.2, 0.3, 0.8, 0.1, 0.1, 300);
    "✅ Evento largo enviado: 15 segundos".postln;
    "👀 OBSERVA: ¿Se mantiene visible durante toda la duración?".postln;
};

~testVisualIntensidad = {
    "🧪 === TEST VISUAL INTENSIDAD ALTA ===".postln;
    // Evento con máxima intensidad visual
    ~osc.sendMsg("/event", "point", 880, 1.0, 5.0, 0.0, 0.0, 1.0, 1.0, 1.0, 500);
    "✅ Evento de alta intensidad enviado".postln;
    "👀 OBSERVA: ¿Se ve brillante y llamativo?".postln;
};

~testVisualSecuencia = {
    "🧪 === TEST VISUAL SECUENCIA TEMPORAL ===".postln;
    "Enviando 5 puntos espaciados cada 2 segundos...".postln;
    
    ~osc.sendMsg("/event", "point", 220, 0.7, 1.5, 0.1, 0.1, 0.5, 0.0, 0.0, 100);
    { 2.wait; ~osc.sendMsg("/event", "point", 330, 0.7, 1.5, 0.1, 0.1, 0.5, 0.2, 0.2, 150); }.fork;
    { 4.wait; ~osc.sendMsg("/event", "point", 440, 0.7, 1.5, 0.1, 0.1, 0.5, 0.4, 0.4, 200); }.fork;
    { 6.wait; ~osc.sendMsg("/event", "point", 660, 0.7, 1.5, 0.1, 0.1, 0.5, 0.6, 0.6, 250); }.fork;
    { 8.wait; ~osc.sendMsg("/event", "point", 880, 0.7, 1.5, 0.1, 0.1, 0.5, 0.8, 0.8, 300); }.fork;
    
    "👀 OBSERVA: ¿Los puntos aparecen en secuencia y se mueven correctamente?".postln;
};

// ========== MENSAJE DE USO ==========

"".postln;
"🎯 TESTS DE DIAGNÓSTICO VISUAL DISPONIBLES:".postln;
"".postln;
"  • ~testVisualSimple.()     - Un punto de 10 segundos".postln;
"  • ~testVisualGliss.()      - Glissando de 8 segundos".postln;
"  • ~testVisualLargo.()      - Evento de 15 segundos".postln;
"  • ~testVisualIntensidad.() - Máxima intensidad visual".postln;
"  • ~testVisualSecuencia.()  - 5 puntos en secuencia".postln;
"".postln;
"🔍 PARA DIAGNOSTICAR EL PROBLEMA:".postln;
"1. Ejecuta: ~testVisualSimple.()".postln;
"2. Observa si el punto llega al borde derecho de la pantalla".postln;
"3. Si NO llega, el problema está en el algoritmo de movimiento".postln;
"4. Si SÍ llega pero se desvanece, el problema está en el fade/alpha".postln;
"".postln;
"📊 DESPUÉS DE CADA TEST, REPORTA:".postln;
"- ¿El evento aparece?".postln;
"- ¿Se mueve de izquierda a derecha?".postln;
"- ¿Llega al final de la pantalla?".postln;
"- ¿Se desvanece gradualmente o desaparece bruscamente?".postln;
"".postln;

"✅ Tests de diagnóstico listos. Ejecuta: ~testVisualSimple.()".postln;
