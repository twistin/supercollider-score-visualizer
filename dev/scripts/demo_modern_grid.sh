#!/bin/bash

# =================================================================
# 🎵 SC SCORE VISUALIZER - DEMO REJILLA MODERNA
# =================================================================
# Demostración de la nueva rejilla con estilo azul moderno

echo "🎵 SC Score Visualizer - Rejilla Moderna Azul"
echo "=============================================="
echo ""

# Verificar que el binario existe
if [[ ! -f "./target/release/sc_score_visualizer" ]]; then
    echo "⚠️  Binario no encontrado. Compilando..."
    cargo build --release
fi

echo "✅ Binario listo con nueva rejilla moderna"
echo ""

echo "🎨 Características de la Nueva Rejilla:"
echo "======================================"
echo "• 🔵 Fondo azul moderno con gradiente radial"
echo "• ✨ Efectos de resplandor (glow) en líneas principales"
echo "• 🎼 Indicadores visuales para notas musicales"
echo "• 📊 Etiquetas con fondos semi-transparentes"
echo "• 🌟 Puntos de luz sutiles en esquinas"
echo "• 💫 Líneas centrales con efecto luminoso"
echo "• 🏷️ Información de estado en tiempo real"
echo ""

echo "🎛️ Controles Interactivos:"
echo "========================="
echo "G     - Activar/desactivar rejilla"
echo "M     - Cambiar modo musical/lineal"
echo "L     - Activar/desactivar etiquetas"
echo "F     - Activar/desactivar etiquetas de frecuencia"
echo "+/-   - Incrementar/decrementar resolución"
echo "1     - Preset rango vocal (80-800 Hz)"
echo "2     - Preset rango instrumental (200-2000 Hz)"
echo "3     - Preset rango completo (20-20000 Hz)"
echo ""

echo "🎨 Paleta de Colores:"
echo "==================="
echo "• Fondo base: Azul oscuro profundo"
echo "• Líneas principales: Azul brillante con glow"
echo "• Líneas secundarias: Azul suave translúcido"
echo "• Líneas centrales: Azul cyan luminoso"
echo "• Etiquetas: Azul claro con fondos semi-transparentes"
echo "• Efectos: Resplandor radial y puntos de luz"
echo ""

# Crear script de demo con patrones visuales atractivos
cat > /tmp/modern_grid_demo.py << 'EOF'
import socket
import time
import math
import random

def send_osc_message(host, port, address, args):
    """Envía un mensaje OSC simple"""
    try:
        sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
        msg = address.encode() + b'\x00'
        while len(msg) % 4 != 0:
            msg += b'\x00'
        
        for arg in args:
            if isinstance(arg, float):
                msg += b',f\x00\x00'
                import struct
                msg += struct.pack('>f', arg)
        
        sock.sendto(msg, (host, port))
        sock.close()
        return True
    except Exception as e:
        return False

def demo_modern_grid():
    """Genera patrones específicos para mostrar la rejilla moderna"""
    print("🎵 Iniciando demo de rejilla moderna...")
    
    # Secuencia de demostraciones
    demos = [
        {"name": "Armonías C Mayor", "pattern": "harmony", "duration": 25},
        {"name": "Glissandos Cromáticos", "pattern": "chromatic", "duration": 20},
        {"name": "Patrones Rítmicos", "pattern": "rhythmic", "duration": 20},
        {"name": "Texturas Ambientales", "pattern": "ambient", "duration": 30},
    ]
    
    for demo in demos:
        print(f"🎼 {demo['name']} - {demo['duration']}s")
        
        start_time = time.time()
        while time.time() - start_time < demo['duration']:
            if demo['pattern'] == "harmony":
                # Armonías en C Mayor para mostrar las divisiones musicales
                base_freq = 261.63  # C4
                harmonics = [1, 5/4, 3/2, 2]  # Acorde mayor
                
                for i, harmonic in enumerate(harmonics):
                    freq = base_freq * harmonic
                    amp = 0.6 - (i * 0.1)
                    send_osc_message('127.0.0.1', 57124, '/music/note', [freq, amp, 2.0, 0.7])
                    time.sleep(0.1)
                
            elif demo['pattern'] == "chromatic":
                # Glissandos cromáticos para mostrar las líneas de frecuencia
                start_freq = 220  # A3
                end_freq = 880   # A5
                steps = 50
                
                for i in range(steps):
                    freq = start_freq + (end_freq - start_freq) * (i / steps)
                    amp = 0.5 + 0.3 * math.sin(i * 0.2)
                    send_osc_message('127.0.0.1', 57124, '/music/note', [freq, amp, 0.5, 0.8])
                    time.sleep(0.05)
                
            elif demo['pattern'] == "rhythmic":
                # Patrones rítmicos para mostrar la rejilla temporal
                frequencies = [130, 165, 196, 247, 294, 349, 392, 440]  # Escala menor natural
                
                for freq in frequencies:
                    amp = random.uniform(0.4, 0.8)
                    send_osc_message('127.0.0.1', 57124, '/music/note', [freq, amp, 0.8, 0.6])
                    time.sleep(0.15)
                
            elif demo['pattern'] == "ambient":
                # Texturas ambientales para mostrar el fondo y efectos
                freq = random.uniform(80, 800) * random.choice([1, 2, 4])
                amp = random.uniform(0.2, 0.6)
                duration = random.uniform(1.0, 3.0)
                timbre = random.uniform(0.3, 0.9)
                
                send_osc_message('127.0.0.1', 57124, '/music/note', [freq, amp, duration, timbre])
                time.sleep(random.uniform(0.1, 0.5))
    
    print("✅ Demo de rejilla moderna completado")

if __name__ == "__main__":
    demo_modern_grid()
EOF

echo "🔧 Iniciando demo visual en segundo plano..."
python3 /tmp/modern_grid_demo.py &
DEMO_PID=$!

echo "🎯 Demo PID: $DEMO_PID"
echo ""

echo "🎵 Lanzando visualizador con rejilla moderna..."
echo "💡 Observa los nuevos efectos visuales:"
echo "   • Fondo azul con gradiente radial"
echo "   • Efectos de resplandor en líneas principales"
echo "   • Etiquetas con fondos semi-transparentes"
echo "   • Puntos indicadores y efectos de luz"
echo ""

# Lanzar el visualizador
./target/release/sc_score_visualizer

# Limpiar procesos
echo ""
echo "🧹 Limpiando procesos..."
kill $DEMO_PID 2>/dev/null
rm -f /tmp/modern_grid_demo.py

echo "✅ Demo completado"
echo ""
echo "🎨 Características destacadas de la rejilla moderna:"
echo "• Estética profesional con tema azul"
echo "• Efectos visuales sutiles pero impactantes"
echo "• Mejor legibilidad con fondos semi-transparentes"
echo "• Indicadores visuales intuitivos"
echo "• Diseño optimizado para presentaciones y conciertos"
