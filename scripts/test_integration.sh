#!/bin/bash
# =============================================================================
# 🎼 TEST INTEGRACIÓN COMPLETA - SC Score Visualizer
# =============================================================================
# Script para probar la integración en tiempo real entre SuperCollider y Rust
# =============================================================================

echo "🎼 === INICIANDO TEST DE INTEGRACIÓN COMPLETA ==="
echo

# Colores para output
RED='\033[0;31m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Verificar que SuperCollider esté disponible
if ! command -v sclang &> /dev/null; then
    echo -e "${RED}❌ SuperCollider no encontrado. Por favor instala SuperCollider.${NC}"
    exit 1
fi

echo -e "${GREEN}✅ SuperCollider encontrado${NC}"

# Compilar el visualizador
echo -e "${BLUE}🔧 Compilando visualizador Rust...${NC}"
cargo build --release
if [ $? -ne 0 ]; then
    echo -e "${RED}❌ Error compilando el visualizador${NC}"
    exit 1
fi
echo -e "${GREEN}✅ Visualizador compilado exitosamente${NC}"

# Crear archivo temporal de SuperCollider
SC_TEMP_FILE="/tmp/sc_realtime_test.scd"
cat > "$SC_TEMP_FILE" << 'EOF'
// Cargar el archivo de análisis
load("/Users/sdcarr/Documents/github/sc-score/sc_score_visualizer/realtime_analysis.scd");

// Esperar un poco para que se cargue
2.wait;

// Iniciar el servidor
Server.default.boot;

// Esperar a que arranque
Server.default.waitForBoot {
    "🎵 Servidor SuperCollider listo!".postln;
    
    // Esperar 2 segundos más
    2.wait;
    
    // Iniciar análisis en tiempo real
    "🚀 Iniciando análisis de audio en tiempo real...".postln;
    ~analyzer = Synth(\RealtimeAnalyzer);
    
    "📡 Enviando datos OSC a puerto 57124...".postln;
    "🎨 El visualizador debería estar recibiendo datos ahora!".postln;
    "";
    "⚡ INSTRUCCIONES:".postln;
    "   - Reproduce música o haz ruido cerca del micrófono".postln;
    "   - Observa las visualizaciones en tiempo real".postln;
    "   - Presiona Ctrl+C en cualquier terminal para terminar".postln;
    "";
};
EOF

echo
echo -e "${YELLOW}🚀 INICIANDO SISTEMA COMPLETO${NC}"
echo -e "${BLUE}📋 INSTRUCCIONES:${NC}"
echo "1. Se abrirá SuperCollider en una terminal"
echo "2. Se abrirá el visualizador en otra ventana"
echo "3. Reproduce música o haz sonidos para ver la visualización"
echo "4. Presiona Ctrl+C en cualquier ventana para terminar"
echo

# Función para limpiar procesos al salir
cleanup() {
    echo
    echo -e "${YELLOW}🧹 Limpiando procesos...${NC}"
    killall sclang 2>/dev/null
    killall sc_score_visualizer 2>/dev/null
    rm -f "$SC_TEMP_FILE"
    echo -e "${GREEN}✅ Limpieza completada${NC}"
    exit 0
}

# Configurar limpieza al recibir señales
trap cleanup SIGINT SIGTERM

echo -e "${BLUE}▶️  Iniciando SuperCollider...${NC}"

# Iniciar SuperCollider en background
osascript << EOF &
tell application "Terminal"
    activate
    do script "cd /Users/sdcarr/Documents/github/sc-score/sc_score_visualizer && echo '🎼 Iniciando SuperCollider...' && sclang '$SC_TEMP_FILE'"
end tell
EOF

# Esperar un poco para que SuperCollider arranque
sleep 3

echo -e "${BLUE}🎨 Iniciando visualizador...${NC}"

# Iniciar el visualizador
./target/release/sc_score_visualizer

# Si llegamos aquí, el visualizador se cerró
cleanup
