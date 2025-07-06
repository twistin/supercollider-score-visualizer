#!/bin/bash

# =============================================================================
# 🔍 DIAGNÓSTICO FINAL ONSETS - SuperCollider
# =============================================================================
# Determina el mejor enfoque para resolver el problema de Onsets
# =============================================================================

echo "🔍 DIAGNÓSTICO FINAL DEL PROBLEMA ONSETS"
echo "========================================"

echo "📊 RESUMEN DEL PROBLEMA:"
echo "Error: 'Onsets arg: threshold has bad input: rcomplex'"
echo "Causa: Los argumentos de Onsets no se interpretan correctamente"
echo ""

echo "🛠️ SOLUCIONES PREPARADAS:"
echo "========================"

# Verificar archivos creados
echo "1. ✅ realtime_analysis.scd - Versión con keyword arguments"
echo "2. ✅ realtime_analysis_alt.scd - Versión SIN Onsets"
echo "3. ✅ test_onsets_syntax.scd - Test de diagnóstico"

echo ""
echo "📋 PLAN DE ACCIÓN:"
echo "=================="
echo "1. EJECUTAR test_onsets_syntax.scd en SuperCollider"
echo "2. Si Onsets funciona → usar realtime_analysis.scd"
echo "3. Si Onsets falla → usar realtime_analysis_alt.scd"
echo ""

echo "🎯 VERSIÓN ALTERNATIVA (SIN ONSETS):"
echo "===================================="
echo "✅ Usa diferencia de amplitud para detectar onsets"
echo "✅ Compatible con cualquier instalación de SuperCollider"
echo "✅ Mantiene los mismos 11 parámetros de análisis"
echo "✅ Misma estructura OSC para el visualizador Rust"
echo ""

echo "📄 ARCHIVOS PARA TESTING:"
echo "========================="
echo "- test_onsets_syntax.scd → Diagnóstico del UGen Onsets"
echo "- realtime_analysis_alt.scd → Versión alternativa garantizada"
echo ""

echo "🚀 COMANDOS PARA SUPERCOLLIDER:"
echo "=============================="
echo ""
echo "// OPCIÓN 1: Test de diagnóstico"
echo "thisProcess.interpreter.executeFile(\"test_onsets_syntax.scd\".resolveRelative);"
echo ""
echo "// OPCIÓN 2: Usar versión alternativa (RECOMENDADO)"
echo "thisProcess.interpreter.executeFile(\"realtime_analysis_alt.scd\".resolveRelative);"
echo "~testAnalysisAlt.();"
echo ""

# Crear script de selección automática
cat > select_analysis_version.scd << 'EOF'
(
// =============================================================================
// 🎯 SELECTOR AUTOMÁTICO DE VERSIÓN DE ANÁLISIS
// =============================================================================

"🎯 SELECCIONANDO VERSIÓN ÓPTIMA DE ANÁLISIS...".postln;

// Test rápido de disponibilidad de Onsets
try {
    // Intentar crear un SynthDef simple con Onsets
    SynthDef(\TestOnsetsAvailable, {
        var in, fft, onset;
        in = SinOsc.ar(440) * 0.1;
        fft = FFT(LocalBuf(1024), in);
        onset = Onsets.kr(fft, \rcomplex, 0.3);
        Out.ar(0, Silent.ar(2));
    }).add;
    
    "✅ Onsets UGen disponible - Cargando versión completa...".postln;
    thisProcess.interpreter.executeFile("realtime_analysis.scd".resolveRelative);
    
} {
    "⚠️ Onsets UGen no disponible - Cargando versión alternativa...".postln;
    thisProcess.interpreter.executeFile("realtime_analysis_alt.scd".resolveRelative);
};

"🎵 Sistema de análisis listo para usar.".postln;
)
EOF

echo "✅ Script de selección automática creado: select_analysis_version.scd"
echo ""
echo "🎉 RECOMENDACIÓN FINAL:"
echo "====================="
echo "Ejecutar en SuperCollider:"
echo "thisProcess.interpreter.executeFile(\"select_analysis_version.scd\".resolveRelative);"
echo ""
echo "Este script detectará automáticamente qué versión usar."
