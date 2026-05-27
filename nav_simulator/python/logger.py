import argparse
import zmq
import json

cli = argparse.ArgumentParser(None)
cli.add_argument("-s", "--sub_addr", default="tcp://localhost:8081")

args = cli.parse_args(None)

if args.sub_addr:
    print(f"subscriber address = {args.sub_addr}\n")

context = zmq.Context()
socket = context.socket(zmq.SUB)
socket.connect(args.sub_addr)
socket.setsockopt_string(zmq.SUBSCRIBE, "")
socket.subscribe("")

try:
    while True:
        msg = socket.recv_string()
        data = json.loads(msg)
        print(f"{data}")
except KeyboardInterrupt:
    print("\nKeyboardInterrupt received, shutting down\n")
finally:
    socket.close()
    context.term