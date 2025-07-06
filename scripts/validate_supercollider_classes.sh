#!/bin/bash

# =============================================================================
# 🔍 VALIDADOR DE CLASES SUPERCOLLIDER
# =============================================================================
# Verifica que todas las clases utilizadas en realtime_analysis.scd 
# existan y estén disponibles en SuperCollider
# =============================================================================

echo "🔍 VALIDANDO CLASES SUPERCOLLIDER"
echo "================================="

# Crear script temporal de validación SuperCollider
cat > validate_classes.scd << 'EOF'
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
EOF

echo "📄 Script de validación SuperCollider creado: validate_classes.scd"

# Verificar que realtime_analysis.scd existe
if [ -f "realtime_analysis.scd" ]; then
    echo "✅ Archivo realtime_analysis.scd encontrado"
    
    # Buscar las clases utilizadas en el archivo
    echo ""
    echo "🔍 CLASES DETECTADAS EN EL ARCHIVO:"
    echo "=================================="
    
    grep -o '\b[A-Z][a-zA-Z]*\.' realtime_analysis.scd | sed 's/\.//' | sort | uniq | while read class; do
        echo "📌 $class"
    done
    
    echo ""
    echo "📋 PRÓXIMOS PASOS:"
    echo "1. Abrir SuperCollider"
    echo "2. Ejecutar: validate_classes.scd"
    echo "3. Verificar que todas las clases estén disponibles"
    echo "4. Si hay errores, instalar las extensiones necesarias"
    echo ""
    echo "🧪 Para test completo:"
    echo "1. Compilar: cargo build --release"
    echo "2. SuperCollider: thisProcess.interpreter.executeFile(\"realtime_analysis.scd\".resolveRelative);"
    echo "3. Verificar que no haya errores de compilación"
    
else
    echo "❌ Archivo realtime_analysis.scd no encontrado"
    exit 1
fi

echo ""
echo "✅ Validación preparada. Ejecuta validate_classes.scd en SuperCollider."
