#!/usr/bin/env python3
"""
Test Visual Intensivo - Envía patrones de datos muy visibles
"""

from pythonosc import udp_client
import time
import math

def test_visual_patterns():
    print("🎨 Iniciando test visual intensivo...")
    
    client = udp_client.SimpleUDPClient("127.0.0.1", 57124)
    
    print("1. Patrón de pitch ascendente...")
    for i in range(20):
        pitch = 200 + (i * 40)  # 200Hz a 1000Hz
        onset = 1.0 if i % 4 == 0 else 0.0  # Onset cada 4 mensajes
        centroid = 1000 + (i * 100)  # Centroide ascendente
        
        client.send_message("/realtime_audio", [
            pitch, onset, centroid, 0.8, 0.6, 2000, 0.4, 0.9
        ])
        
        print(f"  Pitch: {pitch}Hz, Onset: {onset}")
        time.sleep(0.1)
    
    print("\n2. Patrón ondulatorio...")
    for i in range(30):
        t = i * 0.2
        pitch = 440 + (200 * math.sin(t))  # Onda senoidal
        onset = 1.0 if math.sin(t * 2) > 0.5 else 0.0  # Onsets periódicos
        centroid = 1500 + (500 * math.cos(t))  # Centroide coseno
        
        client.send_message("/realtime_audio", [
            pitch, onset, centroid, 0.7, 0.5, 2500, 0.3, 0.8
        ])
        
        print(f"  Onda: pitch={pitch:.1f}Hz, onset={onset}")
        time.sleep(0.15)
    
    print("\n3. Patrón de saltos dramáticos...")
    frequencies = [200, 800, 300, 1200, 150, 1000, 400, 900]
    for i, freq in enumerate(frequencies):
        onset = 1.0  # Todos tienen onset
        centroid = freq * 2  # Centroide proporcional
        
        client.send_message("/realtime_audio", [
            freq, onset, centroid, 0.9, 0.8, freq * 3, 0.5, 1.0
        ])
        
        print(f"  Salto: {freq}Hz -> centroide {centroid}Hz")
        time.sleep(0.3)
    
    print("\n✅ Test visual completado")
    print("Si no ves cambios en la ventana del visualizador, puede estar minimizada o fuera de pantalla")

if __name__ == "__main__":
    test_visual_patterns()
