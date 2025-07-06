#!/usr/bin/env python3
"""
Test de depuración OSC - Verifica envío y recepción de mensajes
"""

import time
import threading
from pythonosc import udp_client
from pythonosc import dispatcher
from pythonosc import osc_server
import argparse

def audio_handler(unused_addr, *args):
    """Handler para mensajes de audio"""
    print(f"Mensaje OSC recibido en {unused_addr}: {args}")

def send_test_message(client):
    """Envía un mensaje de prueba específico"""
    # Mensaje compatible con el formato esperado
    client.send_message("/realtime_audio", [
        0.5,    # pitch (normalized)
        1.0,    # onset_detected
        0.3,    # spectral_centroid
        0.7,    # mfcc_0
        0.4,    # mfcc_1
        0.6,    # spectral_rolloff
        0.8,    # zero_crossing_rate
        0.2     # loudness
    ])
    print("Mensaje OSC enviado a /realtime_audio con 8 parámetros")

def test_osc_reception():
    """Test de recepción OSC en puerto separado"""
    print("Iniciando servidor OSC de prueba en puerto 5556...")
    
    dispatcher_obj = dispatcher.Dispatcher()
    dispatcher_obj.map("/realtime_audio", audio_handler)
    
    server = osc_server.ThreadingOSCUDPServer(("127.0.0.1", 5556), dispatcher_obj)
    server_thread = threading.Thread(target=server.serve_forever, daemon=True)
    server_thread.start()
    
    # Cliente para enviar a nuestro servidor de prueba
    client = udp_client.SimpleUDPClient("127.0.0.1", 5556)
    
    print("Enviando mensaje de prueba a nuestro servidor...")
    send_test_message(client)
    
    time.sleep(2)
    server.shutdown()
    print("Test de recepción completado")

def test_visualizer_osc():
    """Test de envío al visualizador"""
    print("Enviando mensajes al visualizador en puerto 5555...")
    
    client = udp_client.SimpleUDPClient("127.0.0.1", 5555)
    
    for i in range(10):
        # Enviar diferentes patrones para verificar la visualización
        pitch = 0.1 + (i * 0.1)
        onset = 1.0 if i % 3 == 0 else 0.0
        centroid = 0.2 + (i * 0.08)
        
        client.send_message("/realtime_audio", [
            pitch,      # pitch
            onset,      # onset_detected
            centroid,   # spectral_centroid
            0.5,        # mfcc_0
            0.4,        # mfcc_1
            0.6,        # spectral_rolloff
            0.3,        # zero_crossing_rate
            0.7         # loudness
        ])
        
        print(f"Mensaje {i+1}/10 enviado: pitch={pitch:.1f}, onset={onset}, centroid={centroid:.1f}")
        time.sleep(0.5)
    
    print("Todos los mensajes enviados al visualizador")

if __name__ == "__main__":
    print("=== Test de Depuración OSC ===")
    
    # Test 1: Verificar que podemos recibir OSC
    print("\n1. Probando recepción OSC...")
    test_osc_reception()
    
    # Test 2: Enviar al visualizador
    print("\n2. Enviando mensajes al visualizador...")
    test_visualizer_osc()
    
    print("\nTest completado. Verifica la ventana del visualizador para cambios visuales.")
