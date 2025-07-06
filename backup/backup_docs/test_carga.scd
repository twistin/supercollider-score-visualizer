// Prueba simple para verificar que el archivo principal funciona
(
"Probando carga del archivo principal...".postln;
"supercollider_examples.scd".loadRelative;
)

// Después de cargar, ejecuta esto para verificar:
(
if (~scvTestBasicEvents.notNil, {
    "✓ ÉXITO: scvTestBasicEvents está disponible".postln;
    "✓ El archivo se cargó correctamente".postln;
    "Ahora puedes ejecutar: ~scvTestBasicEvents.()".postln;
}, {
    "✗ ERROR: scvTestBasicEvents no está disponible".postln;
    "El archivo no se cargó correctamente".postln;
});
)
