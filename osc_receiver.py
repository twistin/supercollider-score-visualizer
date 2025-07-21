import time
import argparse
from pythonosc import dispatcher
from pythonosc import osc_server

def print_message(address, *args):
    print(f"{time.strftime('%H:%M:%S')} [{address}] -> {args}")


def run_osc_server(ip: str, port: int):
    disp = dispatcher.Dispatcher()
    disp.map("/note", print_message)
    disp.map("/drone", print_message)
    disp.map("/cluster", print_message)
    disp.map("/clear", print_message)
    disp.map("/test", print_message)

    server = osc_server.ThreadingOSCUDPServer((ip, port), disp)
    try:
        print(f"🔌 Listening on {ip}:{port}")
        server.serve_forever()
    except KeyboardInterrupt:
        print("🛑 Servidor OSC detenido por el usuario.")

if __name__ == "__main__":
    print("Script de Python iniciado!")

    parser = argparse.ArgumentParser()
    parser.add_argument("--ip", default="127.0.0.1", help="The ip to listen on")
    parser.add_argument("--port", type=int, default=7777, help="The port to listen on")
    args = parser.parse_args()

    run_osc_server(args.ip, args.port)
