import argparse
import zmq # pyright: ignore[reportMissingImports]
import json
import csv
from pathlib import Path
import re
import os

def next_log_file(directory : Path, prefix : str = "telemetry", ext : str = ".csv", num_digits : int = 3) -> Path:
    
    directory.mkdir(parents=True, exist_ok=True)

    # Matches a file like: telemetry_008.csv
    pattern_to_match = re.compile(rf"^{re.escape(prefix)}_(\d{{{num_digits}}}){re.escape(ext)}$")

    max_file_num : int = -1

    for path in directory.iterdir():

        match = pattern_to_match.match(path.name)
        if match:
            max_file_num = max(max_file_num, int(match.group(1)))

    next_file_num : int = max_file_num + 1

    file_num : str = str(next_file_num).zfill(num_digits)

    return directory / f"{prefix}_{file_num}{ext}"

def main():
    cli = argparse.ArgumentParser(None)
    cli.add_argument("-s", "--sub_addr", default="tcp://localhost:8081")
    cli.add_argument("-e", "--ext", default=".txt")

    args = cli.parse_args(None)

    print(f"subscriber address = {args.sub_addr}\n")

    context : zmq.Context = zmq.Context() # type: ignore
    socket : zmq.Socket = context.socket(zmq.SUB) # type: ignore
    socket.connect(args.sub_addr) # type: ignore
    socket.subscribe("") # type: ignore

    folder : Path = Path(f"{os.getcwd()}/../sim_data/logs")
    output_file : Path = next_log_file(folder, "telemetry", args.ext)
    print(f"creating {output_file}")

    column_names : list[str] = ["id", "x", "y", "r", "t", "which", "frame", "fuel"]

    try:
        with open(output_file, "a", newline="") as file:
            writer : csv.DictWriter[str] = csv.DictWriter(file, fieldnames=column_names)
            writer.writeheader()

            while True:
                msg = socket.recv_string() # type: ignore
                data = json.loads(msg) # type: ignore
                #print(f"{data}")
                writer.writerow(data)

    except KeyboardInterrupt:
        print("\nKeyboardInterrupt received, shutting down\n")
    finally:
        socket.close() # type: ignore
        context.term() # type: ignore

if __name__ == "__main__":
    main()