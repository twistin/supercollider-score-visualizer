(
// =============================================================================
// 🧪 VALIDADOR DE CLASES - SuperCollider
// =============================================================================

"🔍 Validando clases utilizadas en realtime_analysis.scd...".postln;

// Lista de clases a validar
~classesToValidate = [
    \SoundIn,
    \Amplitude, 
    \Pitch,
    \Onsets,
    \FFT,
    \LocalBuf,
    \SpecCentroid,
    \LPZ1,
    \SpecPcile,
    \SpecFlatness,
    \SendReply,
    \Impulse,
    \Out,
    \Silent
];

~validationResults = [];

~classesToValidate.do { |className|
    var classObj = className.asClass;
    if (classObj.notNil) {
        ("✅ " ++ className ++ " - Disponible").postln;
        ~validationResults = ~validationResults.add([className, true]);
    } {
        ("❌ " ++ className ++ " - NO ENCONTRADA").postln;
        ~validationResults = ~validationResults.add([className, false]);
    };
};

"".postln;
"📊 RESUMEN DE VALIDACIÓN:".postln;
"========================".postln;

~validCount = ~validationResults.select { |item| item[1] == true }.size;
~totalCount = ~validationResults.size;

("✅ Clases válidas: " ++ ~validCount ++ "/" ++ ~totalCount).postln;

if (~validCount == ~totalCount) {
    "🎉 TODAS LAS CLASES ESTÁN DISPONIBLES".postln;
    "🟢 realtime_analysis.scd debería compilar sin errores".postln;
} {
    "⚠️ ALGUNAS CLASES NO ESTÁN DISPONIBLES".postln;
    "🔴 Necesitas instalar extensiones o verificar la instalación de SuperCollider".postln;
};

"".postln;
"🔧 Para probar la compilación completa, ejecuta:".postln;
"   thisProcess.interpreter.executeFile(\"realtime_analysis.scd\".resolveRelative);".postln;
"".postln;
)
