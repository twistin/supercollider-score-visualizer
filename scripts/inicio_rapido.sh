#!/bin/bash
# =============================================================================
# 🚀 INICIO RÁPIDO - SC Score Visualizer
# =============================================================================
# Script para usuarios nuevos - automatiza el primer uso
# =============================================================================

echo "🎵 === SC SCORE VISUALIZER - INICIO RÁPIDO ==="
echo

# Colores para output
RED='\033[0;31m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Función para mostrar pasos
show_step() {
    echo -e "${CYAN}▶️  PASO $1: $2${NC}"
    echo
}

# Función para esperar confirmación del usuario
wait_user() {
    echo -e "${YELLOW}⏸️  Presiona ENTER cuando hayas completado este paso...${NC}"
    read -p ""
    echo
}

# Función para verificar comandos
check_command() {
    if command -v $1 &> /dev/null; then
        echo -e "${GREEN}✅ $1 está instalado${NC}"
        return 0
    else
        echo -e "${RED}❌ $1 no está instalado${NC}"
        return 1
    fi
}

show_step "1" "VERIFICACIÓN DEL SISTEMA"

echo "Verificando dependencias..."
DEPENDENCIES_OK=true

if ! check_command cargo; then
    echo -e "${RED}   Instala Rust desde: https://rustup.rs/${NC}"
    DEPENDENCIES_OK=false
fi

if ! check_command python3; then
    echo -e "${RED}   Python 3 es necesario para los tests${NC}"
    DEPENDENCIES_OK=false
fi

if [ ! -d "/Applications/SuperCollider" ] && [ ! -f "/usr/local/bin/sclang" ]; then
    echo -e "${YELLOW}⚠️  SuperCollider no detectado en ubicaciones comunes${NC}"
    echo -e "${YELLOW}   Descarga desde: https://supercollider.github.io/${NC}"
fi

if [ "$DEPENDENCIES_OK" = false ]; then
    echo -e "${RED}❌ Instala las dependencias faltantes y ejecuta el script nuevamente${NC}"
    exit 1
fi

echo -e "${GREEN}✅ Todas las dependencias están listas${NC}"
echo

show_step "2" "COMPILACIÓN DEL PROYECTO"

echo "Compilando SC Score Visualizer..."
if cargo build --release; then
    echo -e "${GREEN}✅ Compilación exitosa${NC}"
else
    echo -e "${RED}❌ Error en la compilación${NC}"
    exit 1
fi
echo

show_step "3" "PRIMERA EJECUCIÓN"

echo -e "${BLUE}Vamos a iniciar el visualizador en background...${NC}"
echo "Se abrirá una ventana negra con un panel azul en la esquina superior derecha."
echo

# Iniciar visualizador en background
./target/release/sc_score_visualizer &
VISUALIZER_PID=$!

sleep 2

if ps -p $VISUALIZER_PID > /dev/null; then
    echo -e "${GREEN}✅ Visualizador iniciado correctamente (PID: $VISUALIZER_PID)${NC}"
    echo -e "${BLUE}📋 Verifica que veas:${NC}"
    echo "   - Una ventana negra con título 'SC Score Visualizer'"
    echo "   - Un panel azul en la esquina superior derecha"
    echo "   - Información del sistema en el panel"
else
    echo -e "${RED}❌ Error iniciando el visualizador${NC}"
    exit 1
fi

wait_user

show_step "4" "TEST DE COMUNICACIÓN OSC"

echo -e "${BLUE}Ejecutando test de glissando...${NC}"
echo "Deberías ver una línea curva que se dibuja de abajo hacia arriba durante 4 segundos."
echo

if [ -f "test_glissando.py" ]; then
    python3 test_glissando.py
    echo -e "${GREEN}✅ Test de glissando completado${NC}"
else
    echo -e "${YELLOW}⚠️  test_glissando.py no encontrado, creando uno simple...${NC}"
    
    cat > quick_test.py << 'EOF'
import socket
import struct
import time

def create_osc_message(address, args):
    msg = address.encode() + b'\x00'
    while len(msg) % 4 != 0:
        msg += b'\x00'
    
    types = ','
    for arg in args:
        types += 'f'
    
    types += '\x00'
    while len(types) % 4 != 0:
        types += '\x00'
    
    msg += types.encode()
    
    for arg in args:
        msg += struct.pack('>f', float(arg))
    
    return msg

print("🎼 Enviando test básico...")
sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
for i in range(10):
    freq = 220 + i * 66  # 220 -> 880 Hz
    message = create_osc_message("/realtime_audio", [
        freq, 0.5, 1.0 if i == 0 else 0.0, 1.0, freq*2, 0.3, freq*3, 0.2, 0.8, 0.2, 0.5
    ])
    sock.sendto(message, ('127.0.0.1', 57124))
    time.sleep(0.1)
sock.close()
print("✅ Test básico enviado!")
EOF

    python3 quick_test.py
    rm quick_test.py
fi

echo
echo -e "${BLUE}📋 ¿Viste la línea curva o cambios visuales?${NC}"
echo "   - Sí: ¡Perfecto! La comunicación OSC funciona"
echo "   - No: Revisa que el visualizador esté en primer plano"

wait_user

show_step "5" "TEST DE ONSETS RÍTMICOS"

if [ -f "test_rhythm.py" ]; then
    echo -e "${BLUE}Ejecutando test de onsets...${NC}"
    echo "Deberías ver 8 flashes precisos cada 0.5 segundos."
    echo
    python3 test_rhythm.py
    echo -e "${GREEN}✅ Test de onsets completado${NC}"
else
    echo -e "${YELLOW}⚠️  Creando test de onsets simple...${NC}"
    
    cat > quick_rhythm.py << 'EOF'
import socket
import struct
import time

def create_osc_message(address, args):
    msg = address.encode() + b'\x00'
    while len(msg) % 4 != 0:
        msg += b'\x00'
    
    types = ','
    for arg in args:
        types += 'f'
    
    types += '\x00'
    while len(types) % 4 != 0:
        types += '\x00'
    
    msg += types.encode()
    
    for arg in args:
        msg += struct.pack('>f', float(arg))
    
    return msg

print("🥁 Enviando 5 onsets rítmicos...")
sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
pitches = [220, 330, 440, 550, 660]

for i, pitch in enumerate(pitches):
    print(f"  💥 Onset #{i+1} - {pitch}Hz")
    
    # Onset marcado
    message = create_osc_message("/realtime_audio", [
        pitch, 0.9, 1.0, 1.0, pitch*3, 0.9, pitch*4, 0.2, 0.8, 0.2, 0.5
    ])
    sock.sendto(message, ('127.0.0.1', 57124))
    time.sleep(0.05)
    
    # Decay
    for j in range(3):
        amp = 0.9 * (1 - j * 0.3)
        message = create_osc_message("/realtime_audio", [
            pitch, amp, 0.0, 1.0, pitch*2, 0.3, pitch*2, 0.3, 0.6, 0.4, 0.2
        ])
        sock.sendto(message, ('127.0.0.1', 57124))
        time.sleep(0.02)
    
    time.sleep(0.4)

sock.close()
print("✅ Test de onsets completado!")
EOF

    python3 quick_rhythm.py
    rm quick_rhythm.py
fi

echo
echo -e "${BLUE}📋 ¿Viste los flashes o pulsos visuales?${NC}"

wait_user

show_step "6" "INTEGRACIÓN CON SUPERCOLLIDER"

echo -e "${BLUE}Ahora vamos a conectar con SuperCollider real:${NC}"
echo
echo "1. Abre SuperCollider.app"
echo "2. Abre el archivo: realtime_analysis.scd"
echo "3. Ejecuta los bloques de código paso a paso (Cmd+Return)"
echo "4. Prueba con sonidos reales"
echo
echo -e "${YELLOW}📖 Consulta GUIA_PRIMER_USO.md para instrucciones detalladas${NC}"

wait_user

show_step "7" "LIMPIEZA Y FINALIZACIÓN"

echo -e "${BLUE}¿Quieres mantener el visualizador corriendo? (y/n)${NC}"
read -p "" keep_running

if [[ $keep_running != "y" && $keep_running != "Y" ]]; then
    echo "Cerrando visualizador..."
    kill $VISUALIZER_PID 2>/dev/null
    echo -e "${GREEN}✅ Visualizador cerrado${NC}"
else
    echo -e "${GREEN}✅ Visualizador sigue ejecutándose (PID: $VISUALIZER_PID)${NC}"
    echo -e "${BLUE}   Para cerrarlo: kill $VISUALIZER_PID${NC}"
fi

echo
echo -e "${GREEN}🎉 === CONFIGURACIÓN INICIAL COMPLETADA ===${NC}"
echo
echo -e "${CYAN}📚 PRÓXIMOS PASOS:${NC}"
echo "   • Lee GUIA_PRIMER_USO.md para uso avanzado"
echo "   • Consulta VALIDACION_COMPLETADA.md para capacidades completas"
echo "   • Experimenta con ARQUITECTURA_AUDIOVISUAL.md para conceptos artísticos"
echo
echo -e "${CYAN}🎵 ¡Disfruta creando arte audiovisual con SC Score Visualizer!${NC}"
