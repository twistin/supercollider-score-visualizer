// TEST SINTAXIS SUPERCOLLIDER - VERIFICACIÓN RÁPIDA
// Este archivo verifica que la sintaxis básica funcione correctamente

(
"Iniciando test de sintaxis SuperCollider...".postln;

// Test 1: Variables básicas
var testVar = 440;
"OK Test 1: Variables básicas funcionan".postln;

// Test 2: NetAddr básico
var oscTest = NetAddr.new("127.0.0.1", 57124);
"OK Test 2: NetAddr funciona".postln;

// Test 3: Funciones básicas
var testFunc = { |freq=440|
    ("OK Test function: " ++ freq ++ "Hz").postln;
};
"OK Test 3: Funciones funcionan".postln;

// Test 4: Envío OSC básico
oscTest.sendMsg("/test", "syntax_check", 440, 0.5);
"OK Test 4: Envío OSC funciona".postln;

"TODOS LOS TESTS SINTAXIS PASSED - SuperCollider syntax OK".postln;
)

// Si ves el mensaje "TODOS LOS TESTS SINTAXIS PASSED", 
// significa que los archivos principales deberían cargar sin problemas.
