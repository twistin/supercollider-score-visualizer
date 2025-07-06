// Test Rápido para Panel de UI
// Prueba algunos eventos para verificar que el panel se ve correctamente

(
var osc = NetAddr.new("127.0.0.1", 57124);

~sendPoint = { |freq=440, amp=0.5, dur=1.0|
    osc.sendMsg("/event", "point", freq, amp, dur, 0.1, 0.1, 0.5, 0.0, 0.0, 220);
    ("Punto enviado: " ++ freq ++ "Hz").postln;
};

~testUI = {
    "TEST UI - Generando eventos para verificar el panel...".postln;
    
    // Enviar algunos eventos para que aparezcan números en el panel
    ~sendPoint.(220, 0.6, 2.0);
    { 0.5.wait; ~sendPoint.(440, 0.7, 2.0); }.fork;
    { 1.0.wait; ~sendPoint.(880, 0.8, 2.0); }.fork;
    { 1.5.wait; ~sendPoint.(1760, 0.5, 2.0); }.fork;
    
    "Panel debe mostrar 'Eventos: 4' y tiempo corriendo...".postln;
};

"UI Test cargado. Ejecuta: ~testUI.value".postln;
)
