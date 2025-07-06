#!/usr/bin/env python3
"""
Script para probar la grilla de frecuencias mejorada
Envía eventos en diferentes octavas para mostrar la nueva grilla
"""

import socket
import struct
import time

def send_osc_message(host, port, address, *args):
    """Envía un mensaje OSC básico"""
    addr_bytes = address.encode('utf-8') + b'\x00'
    while len(addr_bytes) % 4 != 0:
        addr_bytes += b'\x00'
    
    types = ','
    arg_bytes = b''
    
    for arg in args:
        if isinstance(arg, str):
            types += 's'
            encoded = arg.encode('utf-8') + b'\x00'
            while len(encoded) % 4 != 0:
                encoded += b'\x00'
            arg_bytes += encoded
        elif isinstance(arg, float):
            types += 'f'
            arg_bytes += struct.pack('>f', arg)
        elif isinstance(arg, int):
            types += 'i'
            arg_bytes += struct.pack('>i', arg)
    
    types_bytes = types.encode('utf-8') + b'\x00'
    while len(types_bytes) % 4 != 0:
        types_bytes += b'\x00'
    
    message = addr_bytes + types_bytes + arg_bytes
    
    sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
    sock.sendto(message, (host, port))
    sock.close()

def test_frequency_grid():
    """Prueba la grilla de frecuencias con eventos en diferentes octavas"""
    host = "127.0.0.1"
    port = 57124
    
    print("🎵 Probando la grilla de frecuencias mejorada...")
    print("📊 Enviando eventos en diferentes octavas...")
    
    # Frecuencias que coinciden con la grilla nueva
    test_frequencies = [
        (55.0, "A1 - 55Hz"),
        (110.0, "A2 - 110Hz"), 
        (220.0, "A3 - 220Hz"),
        (261.6, "C4 - Do central"),
        (440.0, "A4 - La central"),
        (523.3, "C5 - 523Hz"),
        (880.0, "A5 - 880Hz"),
        (1046.5, "C6 - 1046Hz"),
        (1760.0, "A6 - 1760Hz"),
        (2093.0, "C7 - 2093Hz"),
        (3520.0, "A7 - 3520Hz"),
        (4186.0, "C8 - 4186Hz"),
    ]
    
    for i, (freq, description) in enumerate(test_frequencies):
        amp = 0.6 + (i % 3) * 0.1  # Variación de amplitud
        dur = 2.0 + (i % 2) * 1.0  # Variación de duración
        
        send_osc_message(host, port, "/event", "point", freq, amp, dur)
        print(f"✅ {description} - amp={amp:.1f}, dur={dur:.1f}s")
        
        time.sleep(0.8)  # Envío más rápido para ver el patrón
    
    print("🎯 ¡Grilla de frecuencias testeada! Verifica que los eventos se alineen con las líneas.")

if __name__ == "__main__":
    test_frequency_grid()
