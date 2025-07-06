// test_minimal.scd
// Test mínimo para verificar sintaxis

(
var addr;

"=== TEST MINIMAL ===".postln;

addr = NetAddr.new("127.0.0.1", 57122);

"Enviando un evento simple...".postln;
addr.sendMsg("/event", "point", 440, 0.5, 1.0);

"✓ Evento enviado. Si ves esto, la sintaxis funciona.".postln;
)
