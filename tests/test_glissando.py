#!/usr/bin/env python3
"""
🎼 Test de Glissando Simple - SC Score Visualizer
Simula un glissando de 220Hz a 880Hz durante 4 segundos
"""

import socket
import struct
import time
import math

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

def simulate_glissando():
    """Simula un glissando Xenakis-style"""
    print("🎼 Iniciando glissando 220Hz -> 880Hz durante 4 segundos...")
    print("   Observa el visualizador para ver la línea curva aparecer!")
    
    sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
    
    start_time = time.time()
    duration = 4.0
    start_freq = 220.0
    end_freq = 880.0
    
    while (time.time() - start_time) < duration:
        progress = (time.time() - start_time) / duration
        
        # Frecuencia que asciende logarítmicamente (más natural)
        freq = start_freq * ((end_freq / start_freq) ** progress)
        
        # Amplitud que decrece suavemente
        amp = 0.8 * (1 - progress * 0.7)
        
        # Onset solo al principio
        onset = 1.0 if progress < 0.05 else 0.0
        
        # Centroide que sigue la frecuencia
        centroid = freq * 2.5
        
        send_osc_message(sock, "/realtime_audio", [
            freq,         # pitch aumentando
            amp,          # amp disminuyendo gradualmente
            onset,        # onset solo al inicio
            1.0,          # hasFreq siempre (tono puro)
            centroid,     # centroid proporcional
            0.1,          # flux bajo (tono estable)
            freq * 3,     # rolloff
            0.15,         # flatness baja (muy tonal)
            0.9,          # harmonicity alta
            0.1,          # noisiness baja
            0.3           # spectralSlope
        ])
        
        # Mostrar progreso
        if int(progress * 20) != int((progress - 0.02/duration) * 20):
            print(f"  {progress*100:.0f}% - {freq:.0f}Hz")
        
        time.sleep(0.02)  # 50 Hz
    
    sock.close()
    print("✅ Glissando completado! Deberías haber visto una línea curva ascendente.")

if __name__ == "__main__":
    simulate_glissando()
