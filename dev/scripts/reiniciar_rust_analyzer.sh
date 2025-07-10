#!/bin/bash

# =================================================================
# 🔧 SCRIPT PARA REINICIAR RUST ANALYZER
# =================================================================

echo "🔧 Reiniciando Rust Analyzer..."

# Limpiar target para forzar recompilación
echo "🧹 Limpiando directorio target..."
cargo clean

# Regenerar Cargo.lock si es necesario
echo "📋 Verificando Cargo.lock..."
cargo generate-lockfile

# Compilar proyecto para verificar estado
echo "🔨 Compilando proyecto..."
cargo check

if [ $? -eq 0 ]; then
    echo "✅ Proyecto compila correctamente"
else
    echo "❌ Error en compilación"
    exit 1
fi

echo ""
echo "🎯 Pasos para reiniciar Rust Analyzer en VS Code:"
echo "   1. Presiona Cmd+Shift+P (macOS) o Ctrl+Shift+P (Linux/Windows)"
echo "   2. Busca: 'rust-analyzer: Restart server'"
echo "   3. Selecciona la opción y presiona Enter"
echo ""
echo "📝 Alternativamente:"
echo "   1. Cierra VS Code completamente"
echo "   2. Espera 5 segundos"
echo "   3. Abre VS Code de nuevo"
echo ""
echo "🔧 Si el problema persiste:"
echo "   1. Verifica que rust-analyzer esté actualizado"
echo "   2. Revisa la configuración de VS Code"
echo "   3. Considera reiniciar el sistema"
echo ""
echo "✅ El proyecto está listo para trabajar"
