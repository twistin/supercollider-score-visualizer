#!/usr/bin/env python3
from pythonosc import udp_client
import time
import math

print("🎨 Enviando visualización CONTINUA...")
print("👀 AHORA busca la ventana del visualizador usando Cmd+Tab")

client = udp_client.SimpleUDPClient("127.0.0.1", 57124)

for i in range(100):  # 100 mensajes = 20 segundos de visualización
    t = i * 0.2
    pitch = 440 + (300 * math.sin(t))
    onset = 1.0 if i % 5 == 0 else 0.0
    centroid = 1200 + (800 * math.cos(t * 0.7))
    amplitude = 0.8 + (0.2 * math.sin(t * 1.5))
    
    client.send_message("/realtime_audio", [
        pitch, amplitude, centroid, 0.7, 0.5, 2500, 0.3, amplitude
    ])
    
    if i % 10 == 0:
        print(f"🎵 Enviando patrón visual {i//10 + 1}/10...")
    
    time.sleep(0.2)

print("✅ Patrón visual completado")
print("Si no viste nada, la ventana está oculta o fuera de pantalla")
