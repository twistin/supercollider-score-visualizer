#!/usr/bin/env python3
"""
🧪 Test de integración audiovisual completa
Simula datos de SuperCollider y verifica que el visualizador responda
"""

import asyncio
from pythonosc import udp_client
import time
import math

async def test_complete_integration():
    """Test completo de integración audiovisual"""
    
    client = udp_client.SimpleUDPClient("127.0.0.1", 57124)
    
    print("🎵 TEST COMPLETO DE INTEGRACIÓN AUDIOVISUAL")
    print("==========================================")
    print("")
    print("📡 Enviando datos simulados de análisis de audio...")
    print("🎯 Verifica que aparezcan visualizaciones en la ventana del visualizador")
    print("")
    
    # Test con diferentes tipos de audio simulado
    test_phases = [
        ("🎵 Fase 1: Tono constante", "constant_tone"),
        ("🌊 Fase 2: Glissando", "glissando"), 
        ("💥 Fase 3: Onsets rítmicos", "rhythmic_onsets"),
        ("🎨 Fase 4: Análisis espectral complejo", "complex_spectral"),
        ("🔥 Fase 5: Señal compleja completa", "full_complex")
    ]
    
    for phase_name, phase_type in test_phases:
        print(f"\n{phase_name}")
        print("=" * len(phase_name))
        
        for i in range(40):  # 2 segundos por fase a 20 Hz
            time_factor = i * 0.05
            
            if phase_type == "constant_tone":
                # Tono constante con pequeñas variaciones
                pitch = 440 + 10 * math.sin(time_factor * 8)
                amp = 0.7 + 0.1 * math.sin(time_factor * 3)
                onset = 1.0 if i == 0 else 0.0
                
            elif phase_type == "glissando":
                # Glissando ascendente
                pitch = 220 + 600 * (time_factor / 2)
                amp = 0.6 + 0.2 * math.sin(time_factor * 4)
                onset = 0.0
                
            elif phase_type == "rhythmic_onsets":
                # Onsets rítmicos regulares
                pitch = 330 + 50 * math.sin(time_factor * 2)
                amp = 0.5 + 0.4 * (1 if i % 8 < 2 else 0)
                onset = 1.0 if i % 8 == 0 else 0.0
                
            elif phase_type == "complex_spectral":
                # Análisis espectral complejo
                pitch = 440 + 200 * math.sin(time_factor * 0.7)
                amp = 0.4 + 0.3 * math.sin(time_factor * 1.5)
                onset = 1.0 if i % 12 == 0 else 0.0
                
            else:  # full_complex
                # Todo combinado
                pitch = 300 + 400 * math.sin(time_factor * 0.3) + 100 * math.sin(time_factor * 2.1)
                amp = 0.3 + 0.5 * abs(math.sin(time_factor * 1.2))
                onset = 1.0 if (i % 6 == 0 or i % 13 == 0) else 0.0
            
            # Parámetros comunes para todas las fases
            has_freq = 0.85 + 0.15 * math.sin(time_factor * 1.1)
            
            # Centroide espectral variable
            centroid = 800 + 800 * math.sin(time_factor * 0.8) + 200 * math.sin(time_factor * 3.2)
            
            # Flujo espectral
            if i > 0:
                flux = abs(centroid - prev_centroid) / 200
            else:
                flux = 0.0
            prev_centroid = centroid
            
            # Otros parámetros espectrales
            rolloff = centroid * 1.5 + 400 * math.sin(time_factor * 0.6)
            flatness = 0.2 + 0.3 * abs(math.sin(time_factor * 2.3))
            
            # Parámetros derivados
            harmonicity = has_freq * (1 - flatness)
            noisiness = flatness * (1 - has_freq)
            spectral_slope = (centroid - 1000) / 1000
            
            # Enviar mensaje OSC
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
            
            # Mostrar progreso cada 10 frames
            if i % 10 == 0:
                print(f"  📊 Frame {i}/40: Pitch={pitch:.0f}Hz, Amp={amp:.2f}, Onset={onset}")
            
            await asyncio.sleep(0.05)  # 20 Hz
        
        print(f"  ✅ {phase_name} completada")
        await asyncio.sleep(0.5)  # Pausa entre fases
    
    print("\n🎉 TEST COMPLETO FINALIZADO")
    print("===========================")
    print("🎯 Si viste visualizaciones durante el test:")
    print("   ✅ La integración funciona perfectamente")
    print("   ✅ Puedes proceder a usar SuperCollider")
    print("")
    print("❌ Si NO viste visualizaciones:")
    print("   🔍 Verifica que el visualizador esté ejecutándose")
    print("   🔍 Revisa la ventana del visualizador")
    print("   🔍 Comprueba que no haya errores en la terminal")

if __name__ == "__main__":
    print("🎮 INICIANDO EN 3 SEGUNDOS...")
    print("   Asegúrate de que la ventana del visualizador esté visible")
    time.sleep(3)
    asyncio.run(test_complete_integration())
