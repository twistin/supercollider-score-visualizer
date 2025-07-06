// 🧪 PRUEBA MÍNIMA - Variables SuperCollider
// Este archivo demuestra la solución correcta

// ========== DECLARACIONES GLOBALES ==========
var scvTestMinimo;

(
// Variable local
var funcionLocal;

"🚀 Cargando prueba mínima...".postln;

// Definir función local
funcionLocal = {
    "✅ ¡Función de prueba ejecutada correctamente!".postln;
    "📡 Enviando evento de prueba...".postln;
};

// Asignar a variable global
scvTestMinimo = funcionLocal;

// Verificar
if(scvTestMinimo.notNil, {
    "✅ scvTestMinimo exportada correctamente".postln;
}, {
    "❌ ERROR: scvTestMinimo no se exportó".postln;
});

"🎉 Prueba completada. Ejecuta: scvTestMinimo()".postln;

) // Fin del bloque

// INSTRUCCIONES:
// 1. Ejecuta todo este archivo (Cmd+A, Cmd+Return)
// 2. Luego ejecuta: scvTestMinimo()
