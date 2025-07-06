#!/usr/bin/env python3
"""
Script de prueba simple para enviar eventos OSC al SC Score Visualizer
sin necesidad de SuperCollider. Útil para probar la aplicación.
"""

import socket
import time
import random
import struct

class SimpleOSCSender:
    def __init__(self, host="127.0.0.1", port=57124):
        self.host = host
        self.port = port
        self.sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
    
    def send_message(self, address, *args):
        """Envía un mensaje OSC simple"""
        # Construir mensaje OSC básico
        msg = self._build_message(address, args)
        self.sock.sendto(msg, (self.host, self.port))
    
    def _build_message(self, address, args):
        """Construye un mensaje OSC básico"""
        # Dirección OSC
        addr_bytes = address.encode('utf-8') + b'\x00'
        # Padding para alinear a 4 bytes
        while len(addr_bytes) % 4 != 0:
            addr_bytes += b'\x00'
        
        # Tipos de argumentos
        type_tag = ','
        arg_bytes = b''
        
        for arg in args:
            if isinstance(arg, str):
                type_tag += 's'
                arg_str = arg.encode('utf-8') + b'\x00'
                while len(arg_str) % 4 != 0:
                    arg_str += b'\x00'
                arg_bytes += arg_str
            elif isinstance(arg, float):
                type_tag += 'f'
                arg_bytes += struct.pack('>f', arg)
            elif isinstance(arg, int):
                type_tag += 'i'
                arg_bytes += struct.pack('>i', arg)
        
        # Tag de tipos
        type_bytes = type_tag.encode('utf-8') + b'\x00'
        while len(type_bytes) % 4 != 0:
            type_bytes += b'\x00'
        
        return addr_bytes + type_bytes + arg_bytes

def test_events():
    """Función principal de prueba"""
    sender = SimpleOSCSender()
    
    print("Enviando eventos de prueba al SC Score Visualizer...")
    print("Asegúrate de que la aplicación esté ejecutándose.")
    print("Presiona Ctrl+C para detener.")
    
    try:
        # Prueba 1: Eventos puntuales
        print("\n1. Enviando eventos puntuales...")
        for i in range(10):
            freq = 200 + i * 100 + random.uniform(-50, 50)
            amp = 0.3 + random.uniform(0, 0.5)
            duration = 1.0 + random.uniform(0, 2.0)
            attack = 0.05 + random.uniform(0, 0.1)
            decay = 0.1 + random.uniform(0, 0.2)
            density = random.uniform(0.3, 0.8)
            texture = random.uniform(0, 0.5)
            spread = random.uniform(0, 0.3)
            hue = random.uniform(0, 360)
            
            sender.send_message("/event", "point", freq, amp, duration, attack, decay, density, texture, spread, hue)
            time.sleep(0.3)
        
        time.sleep(2)
        
        # Prueba 2: Glissandi
        print("2. Enviando glissandi...")
        for i in range(5):
            start_freq = 300 + random.uniform(-100, 100)
            end_freq = 800 + random.uniform(-200, 200)
            amp = 0.4 + random.uniform(0, 0.4)
            duration = 2.0 + random.uniform(0, 2.0)
            curvature = random.uniform(-0.8, 0.8)
            density = 0.7
            texture = random.uniform(0, 0.4)
            spread = random.uniform(0, 0.2)
            hue = 30 + random.uniform(-20, 20)
            
            sender.send_message("/event", "gliss", start_freq, end_freq, amp, duration, curvature, density, texture, spread, hue)
            time.sleep(0.5)
        
        time.sleep(2)
        
        # Prueba 3: Clusters
        print("3. Enviando clusters...")
        for i in range(3):
            center_freq = 400 + i * 200
            freq_spread = 80 + random.uniform(20, 100)
            num_voices = random.randint(4, 12)
            amp = 0.5 + random.uniform(0, 0.3)
            duration = 3.0 + random.uniform(0, 2.0)
            density = 0.8
            texture = 0.3
            spread = 0.5
            hue = 120 + random.uniform(-30, 30)
            
            sender.send_message("/event", "cluster", center_freq, freq_spread, num_voices, amp, duration, density, texture, spread, hue)
            time.sleep(1.0)
        
        time.sleep(2)
        
        # Prueba 4: Ruido
        print("4. Enviando texturas de ruido...")
        for i in range(4):
            freq_center = 800 + random.uniform(-200, 400)
            freq_bandwidth = 200 + random.uniform(100, 300)
            amp = 0.3 + random.uniform(0, 0.3)
            duration = 2.5 + random.uniform(0, 1.5)
            grain_size = random.uniform(0.5, 2.0)
            spectral_tilt = random.uniform(-0.5, 0.5)
            density = 0.6
            texture = 0.8
            spread = 0.4
            hue = 300 + random.uniform(-60, 60)
            
            sender.send_message("/event", "noise", freq_center, freq_bandwidth, amp, duration, grain_size, spectral_tilt, density, texture, spread, hue)
            time.sleep(0.8)
        
        time.sleep(2)
        
        # Prueba 5: Masas sonoras
        print("5. Enviando masas sonoras...")
        for i in range(2):
            num_components = random.randint(4, 8)
            amp = 0.4 + random.uniform(0, 0.3)
            duration = 4.0 + random.uniform(0, 2.0)
            evolution_rate = random.uniform(0.2, 0.8)
            turbulence = random.uniform(0.1, 0.5)
            density = 0.9
            texture = 0.5
            spread = 0.7
            hue = 60 + random.uniform(-30, 30)
            
            # Generar componentes espectrales
            freqs_amps = []
            base_freq = 300 + i * 200
            for j in range(num_components):
                freq = base_freq * (1 + j * 0.3 + random.uniform(-0.1, 0.1))
                comp_amp = 1.0 / (j + 1) * random.uniform(0.7, 1.3)
                freqs_amps.extend([freq, comp_amp])
            
            args = ["mass", num_components, amp, duration, evolution_rate, turbulence, density, texture, spread, hue] + freqs_amps
            sender.send_message("/event", *args)
            time.sleep(2.0)
        
        print("\n¡Prueba completada! El visualizador debería mostrar diferentes tipos de eventos.")
        print("Puedes repetir la prueba ejecutando este script nuevamente.")
        
    except KeyboardInterrupt:
        print("\nPrueba interrumpida por el usuario.")
    except Exception as e:
        print(f"Error durante la prueba: {e}")
    finally:
        sender.sock.close()

if __name__ == "__main__":
    test_events()
