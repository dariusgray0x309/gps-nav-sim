import pandas as pd
import matplotlib.pyplot as plt
import os
from pathlib import Path
import argparse
from datetime import datetime

def plot_data(file_name : str):
    
    now = datetime.now()

    date = now.strftime(format="%Y%m%d_%H%M")

    base_dir : Path = Path(f"{os.getcwd()}/sim_data")

    save_dir : Path = Path(f"{base_dir}/plots/{date}")
    save_dir.mkdir(parents=True, exist_ok=True)

    data_file : Path = Path(f"{base_dir}/logs/{file_name}")

    if not data_file.is_file():
        print(f"{data_file} does not exist")
        return    

    df : pd.DataFrame = pd.read_csv(data_file, delimiter=",")

    sat_1 = df[(df['which'] == 'Satellite') & (df['id'] == 1)]
    sat_2 = df[(df['which'] == 'Satellite') & (df['id'] == 2)]
    sat_3 = df[(df['which'] == 'Satellite') & (df['id'] == 3)]
    car = df[(df['which'] == 'Vehicle')]
    est = df[(df['which'] == 'Estimate')]

    print("Plotting:")
    plt.figure()
    plt.plot(car.x, car.y, color= 'b', label="Truth")
    plt.plot(est.x, est.y, color='r', label = "Trilateration", linestyle='--')
    plt.xlabel("x [m]")
    plt.ylabel("y [m]")
    plt.grid()
    plt.legend()
    plt.title("Vehicle Trajectory")
    img_name : str = "vehicle_trajectory.png"
    print(f"saving {img_name} to {save_dir}")
    plt.savefig(f"{save_dir}/{img_name}")

    plt.figure()
    plt.plot(car.t, car.fuel)
    plt.xlabel("t [s]")
    plt.ylabel("fuel remaining [%]")
    plt.grid()
    plt.title("Remaining fuel percentage")
    img_name = "vehicle_fuel.png"
    print(f"saving {img_name} to {save_dir}")
    plt.savefig(f"{save_dir}/{img_name}")

    plt.figure()
    plt.plot(sat_1.x, sat_1.y)
    plt.xlabel("x [m]")
    plt.ylabel("y [m]")
    plt.grid()
    plt.title("Satellite 1 Trajectory")
    img_name = "sat_1_trajectory.png"
    print(f"saving {img_name} to {save_dir}")
    plt.savefig(f"{save_dir}/{img_name}")

    plt.figure()
    plt.plot(sat_1.t, sat_1.r)
    plt.xlabel("t [s]")
    plt.ylabel("r [m]")
    plt.grid()
    plt.title("Satellite 1 Range to Vehicle")
    img_name = "sat_1_range.png"
    print(f"saving {img_name} to {save_dir}")
    plt.savefig(f"{save_dir}/{img_name}")

    plt.figure()
    plt.plot(sat_2.x, sat_2.y)
    plt.xlabel("x [m]")
    plt.ylabel("y [m]")
    plt.grid()
    plt.title("Satellite 2 Trajectory")
    img_name = "sat_2_trajectory.png"
    print(f"saving {img_name} to {save_dir}")
    plt.savefig(f"{save_dir}/{img_name}")

    plt.figure()
    plt.plot(sat_2.t, sat_2.r)
    plt.xlabel("t [s]")
    plt.ylabel("r [m]")
    plt.grid()
    plt.title("Satellite 2 Range to Vehicle")
    img_name = "sat_2_range.png"
    print(f"saving {img_name} to {save_dir}")
    plt.savefig(f"{save_dir}/{img_name}")

    plt.figure()
    plt.plot(sat_3.x, sat_3.y)
    plt.xlabel("x [m]")
    plt.ylabel("y [m]")
    plt.grid()
    plt.title("Satellite 3 Trajectory")
    img_name = "sat_3_trajectory.png"
    print(f"saving {img_name} to {save_dir}")
    plt.savefig(f"{save_dir}/{img_name}")

    plt.figure()
    plt.plot(sat_3.t, sat_3.r)
    plt.xlabel("t [s]")
    plt.ylabel("r [m]")
    plt.grid()
    plt.title("Satellite 3 Range to Vehicle")
    img_name = "sat_3_range.png"
    print(f"saving {img_name} to {save_dir}")
    plt.savefig(f"{save_dir}/{img_name}")
    
def main():

    cli = argparse.ArgumentParser(None)
    cli.add_argument("-f", "--file_name", default="telemetry_000.txt")

    args = cli.parse_args(None)

    plot_data(args.file_name)

if __name__ == "__main__":
    main()