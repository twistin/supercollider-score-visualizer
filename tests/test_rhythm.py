#!/usr/bin/env python3
"""
🥁 Test de Onsets Rítmicos - SC Score Visualizer
Simula una secuencia de onsets para probar la visualización estilo Ikeda
"""

import socket
import struct
import time

def create_osc_message(address, args):
    """Crea un mensaje OSC simple"""
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

def send_osc_message(sock, address, args):
    """Envía un mensaje OSC"""
    message = create_osc_message(address, args)
    sock.sendto(message, ('127.0.0.1', 57124))

def simulate_rhythm():
    """Simula un patrón rítmico con onsets marcados (estilo Ikeda)"""
    print("🥁 Iniciando patrón rítmico - 8 onsets cada 0.5 segundos")
    print("   Observa los flashes sincronizados en el visualizador!")
    
    sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
    
    pitches = [220, 330, 440, 550, 660, 770, 880, 330]  # Secuencia ascendente
    
    for i in range(8):
        print(f"  💥 Onset #{i+1} - {pitches[i]}Hz")
        
        # ONSET MARCADO - Alta energía
        send_osc_message(sock, "/realtime_audio", [
            pitches[i],   # pitch variado
            0.9,          # amp ALTA
            1.0,          # onset TRUE - Este es el momento clave!
            1.0,          # hasFreq
            pitches[i] * 3,  # centroid alto (brillo)
            0.9,          # flux ALTO (cambio tímbrico)
            pitches[i] * 4,  # rolloff alto
            0.2,          # flatness baja (tonal)
            0.8,          # harmonicity alta
            0.2,          # noisiness baja
            0.5           # spectralSlope
        ])
        
        time.sleep(0.05)  # 50ms de onset
        
        # DECAY - Energía decayendo
        for decay_step in range(5):
            amp_decay = 0.9 * (1 - decay_step * 0.15)
            send_osc_message(sock, "/realtime_audio", [
                pitches[i],
                amp_decay,    # amp decreciendo
                0.0,          # onset FALSE
                1.0,          # hasFreq
                pitches[i] * 2,  # centroid medio
                0.3,          # flux medio
                pitches[i] * 2,  # rolloff medio  
                0.3,          # flatness media
                0.6,          # harmonicity media
                0.4,          # noisiness media
                0.2           # spectralSlope
            ])
            time.sleep(0.02)  # 20ms cada step
        
        # SILENCIO entre onsets
        for silence_step in range(15):
            send_osc_message(sock, "/realtime_audio", [
                0,            # sin pitch
                0.01,         # amp mínima
                0.0,          # sin onset
                0.0,          # sin hasFreq
                200.0,        # centroid bajo
                0.05,         # flux bajo
                400.0,        # rolloff bajo
                0.95,         # flatness ALTA (ruido)
                0.05,         # harmonicity baja
                0.95,         # noisiness ALTA
                -0.8          # spectralSlope negativo
            ])
            time.sleep(0.02)  # 20ms cada step
    
    sock.close()
    print("✅ Patrón rítmico completado! Deberías haber visto 8 flashes precisos.")

if __name__ == "__main__":
    simulate_rhythm()
