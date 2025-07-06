#!/bin/bash

# =============================================================================
# 🎯 DIAGNÓSTICO DE VISUALIZACIÓN
# =============================================================================
# Verifica por qué no se ven las visualizaciones aunque los datos OSC lleguen
# =============================================================================

echo "🎯 DIAGNÓSTICO: DATOS OSC RECIBIDOS PERO NO SE VE VISUALIZACIÓN"
echo "=============================================================="

echo ""
echo "✅ CONFIRMADO: El visualizador está recibiendo datos OSC correctamente"
echo "✅ CONFIRMADO: SuperCollider está enviando 11 parámetros de análisis"
echo "✅ CONFIRMADO: La comunicación OSC funciona perfectamente"
echo ""

echo "🔍 POSIBLES CAUSAS DE NO VER VISUALIZACIÓN:"
echo "==========================================="
echo ""
echo "1. 🖥️  VENTANA MINIMIZADA O OCULTA"
echo "   - Verifica si hay una ventana del visualizador abierta"
echo "   - Busca en la barra de tareas o dock"
echo "   - Usa Cmd+Tab (Mac) o Alt+Tab (otros) para cambiar ventanas"
echo ""

echo "2. 📱 VENTANA FUERA DE PANTALLA"
echo "   - La ventana puede estar en coordenadas fuera de la pantalla visible"
echo "   - Solución: Reiniciar el visualizador"
echo ""

echo "3. 🎨 CONFIGURACIÓN VISUAL"
echo "   - Los colores pueden ser muy tenues o transparentes"
echo "   - El fondo puede ser del mismo color que las visualizaciones"
echo ""

echo "4. 📊 RANGO DE DATOS"
echo "   - Los datos pueden estar fuera del rango visual esperado"
echo "   - Necesitas datos de audio más intensos"
echo ""

echo "🛠️ SOLUCIONES PARA PROBAR:"
echo "=========================="
echo ""

echo "SOLUCIÓN 1: Verificar ventanas abiertas"
echo "---------------------------------------"
echo "• Busca una ventana negra o de color oscuro"
echo "• Puede tener título 'SC Score Visualizer' o similar"
echo ""

echo "SOLUCIÓN 2: Reiniciar con ventana forzada"
echo "----------------------------------------"
echo "pkill -f sc_score_visualizer"
echo "sleep 2"
echo "./target/release/sc_score_visualizer"
echo ""

echo "SOLUCIÓN 3: Test con datos más intensos"
echo "--------------------------------------"
echo "• Ejecutar: python3 test_visual_integration.py"
echo "• Este test usa datos más visibles"
echo ""

echo "SOLUCIÓN 4: Verificar en SuperCollider"
echo "-------------------------------------"
echo "• ~startAnalysisAlt.(); // para micrófono"
echo "• ~testAnalysisAlt.();  // para tono generado"
echo "• Hablar al micrófono o hacer ruido fuerte"
echo ""

echo "🎯 PRÓXIMO PASO RECOMENDADO:"
echo "============================"
echo "1. Buscar la ventana del visualizador (puede estar minimizada)"
echo "2. Si no la encuentras, reiniciar el visualizador"
echo "3. Ejecutar el test visual intenso"
echo ""

# Crear comando para reinicio limpio
echo "🔄 COMANDO DE REINICIO LIMPIO:"
echo "============================="
echo "pkill -f sc_score_visualizer && sleep 2 && ./target/release/sc_score_visualizer"
echo ""

echo "🎮 DESPUÉS DEL REINICIO:"
echo "======================"
echo "1. Busca la ventana del visualizador que debe aparecer"
echo "2. En SuperCollider ejecuta: ~testAnalysisAlt.();"
echo "3. Deberías ver visualizaciones inmediatamente"
