#!/usr/bin/env python3
"""
Test de envío de eventos OSC al SC Score Visualizer Universal
Este script envía eventos de prueba para verificar la conexión
"""

import time
from python_osc.udp_client import SimpleUDPClient

def main():
    print("🧪 Test de conexión OSC con SC Score Visualizer")
    
    # Conectar al puerto correcto
    client = SimpleUDPClient("127.0.0.1", 57123)
    
    print("📡 Enviando eventos de prueba...")
    
    # Test 1: Evento puntual básico
    client.send_message("/event", ["point", 440.0, 0.8, 2.0, 0.1, 0.1, 0.5, 0.0, 0.0, 220.0])
    print("✅ Enviado: evento puntual (440Hz)")
    time.sleep(0.5)
    
    # Test 2: Glissando
    client.send_message("/event", ["gliss", 220.0, 880.0, 0.7, 3.0, 0.2, 0.3, 0.7, 0.2, 0.1, 120.0])
    print("✅ Enviado: glissando (220-880Hz)")
    time.sleep(0.5)
    
    # Test 3: Cluster
    client.send_message("/event", ["cluster", 500.0, 0.6, 1.5, 0.05, 0.2, 0.8, 0.3, 0.4, 180.0])
    print("✅ Enviado: cluster (500Hz)")
    time.sleep(0.5)
    
    # Test 4: Ruido
    client.send_message("/event", ["noise", 1000.0, 2000.0, 0.5, 2.0, 0.1, 0.4, 0.9, 0.5, 0.2, 60.0])
    print("✅ Enviado: ruido (1000-2000Hz)")
    time.sleep(0.5)
    
    # Test 5: Masa sonora
    client.send_message("/event", ["soundmass", 300.0, 0.9, 4.0, 0.3, 0.5, 1.0, 0.8, 0.6, 280.0])
    print("✅ Enviado: masa sonora (300Hz)")
    
    print("🎉 ¡Test completado! Verifica el visualizador para ver los eventos.")

if __name__ == "__main__":
    main()
