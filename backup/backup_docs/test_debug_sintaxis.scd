// test_debug_sintaxis.scd
// Para debuggear exactamente qué está causando el problema

(
var addr;

"=== DEBUG SINTAXIS ===".postln;
"Si ves este mensaje, el bloque básico funciona".postln;

addr = NetAddr.new("127.0.0.1", 57122);
"Conexión OSC creada".postln;

addr.sendMsg("/event", "point", 440, 0.5, 1.0);
"Mensaje OSC enviado".postln;

"✓ Todo OK - sintaxis funciona perfectamente".postln;
)
