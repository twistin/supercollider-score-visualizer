#!/usr/bin/env python3
"""
🧪 Test de validación post-corrección
Simula datos de análisis en tiempo real para verificar el funcionamiento
"""

import asyncio
from pythonosc import udp_client
import time
import math

async def test_corrected_integration():
    """Test de integración tras corrección de sintaxis"""
    
    client = udp_client.SimpleUDPClient("127.0.0.1", 57124)
    
    print("🧪 Iniciando test de validación post-corrección...")
    print("📡 Enviando datos simulados de análisis de audio...")
    
    for i in range(50):  # 50 mensajes de prueba
        # Simular análisis de audio realista
        time_factor = i * 0.1
        
        # Pitch con glissando sutil
        pitch = 440 + 100 * math.sin(time_factor * 0.5)
        
        # Amplitud con envolvente
        amp = 0.5 + 0.3 * math.sin(time_factor * 2)
        
        # Onset ocasional
        onset = 1.0 if (i % 10 == 0) else 0.0
        
        # hasFreq alta para pitch estable
        has_freq = 0.9 + 0.1 * math.sin(time_factor)
        
        # Parámetros espectrales variables
        centroid = 1000 + 500 * math.sin(time_factor * 0.8)
        flux = 0.3 + 0.2 * math.sin(time_factor * 1.2)
        rolloff = 2000 + 800 * math.sin(time_factor * 0.6)
        flatness = 0.2 + 0.1 * math.sin(time_factor * 1.5)
        
        # Parámetros derivados
        harmonicity = has_freq * (1 - flatness)
        noisiness = flatness * (1 - has_freq)
        spectral_slope = -0.1 + 0.05 * math.sin(time_factor * 0.9)
        
        # Enviar mensaje OSC con el formato corregido
        client.send_message("/realtime_audio", [
            pitch,          # [0] Frecuencia fundamental
            amp,            # [1] Amplitud RMS
            onset,          # [2] Detección de onset
            has_freq,       # [3] Confianza de pitch
            centroid,       # [4] Centroide espectral
            flux,           # [5] Flujo espectral
            rolloff,        # [6] Rolloff espectral
            flatness,       # [7] Planitud espectral
            harmonicity,    # [8] Harmonicidad
            noisiness,      # [9] Ruidosidad
            spectral_slope  # [10] Pendiente espectral
        ])
        
        if i % 10 == 0:
            print(f"📊 Enviado paquete {i}/50 - Pitch: {pitch:.1f}Hz, Amp: {amp:.2f}")
        
        await asyncio.sleep(0.02)  # 50 Hz como el SynthDef
    
    print("✅ Test de validación completado")
    print("🎯 Verifica que el visualizador muestre los datos correctamente")

if __name__ == "__main__":
    asyncio.run(test_corrected_integration())
