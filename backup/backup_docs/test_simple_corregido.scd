// test_simple_corregido.scd
// Script de prueba simple para verificar parsing OSC flexible (SIN .wait)

(
var addr = NetAddr.new("127.0.0.1", 57122);

"=== PRUEBA DE PARSING FLEXIBLE ===".postln;

// Enviar mensajes con tipos mixtos inmediatamente (sin esperas)
"Enviando evento con flotantes...".postln;
addr.sendMsg("/event", "point", 440.0, 0.7, 2.0);

"Enviando evento con enteros...".postln;
addr.sendMsg("/event", "point", 880, 1, 1);

"Enviando evento con tipos mixtos...".postln;
addr.sendMsg("/event", "point", 220.5, 0, 3);

"Enviando glissando con tipos mixtos...".postln;
addr.sendMsg("/event", "gliss", 100, 1000.0, 0.8, 4);

"Enviando cluster con tipos mixtos...".postln;
addr.sendMsg("/event", "cluster", 660, 150.5, 6, 0.3, 4.0);

"Todos los mensajes enviados. Verifica el visualizador.".postln;
)
