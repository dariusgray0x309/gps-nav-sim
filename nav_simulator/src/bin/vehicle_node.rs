use clap::Parser;
use std::{thread, time::{Duration, Instant}};
use nav_simulator::vehicle::{self, Vehicle};
use nav_simulator::util::Populate;

#[derive(Parser, Debug)]
struct Args{

    #[arg(short, long, default_value_t = false)]
    logging : bool,

    #[arg(short, long, default_value_t = 35.0)]
    velocity : f64,

    #[arg(short, long, default_value_t = 7.0)]
    efficiency : f64,

    #[arg(short, long, default_value_t = 55.0)]
    fuel : f64,

    #[arg(short, long, default_value_t = 0.01)]
    dt : f64,

    #[arg(short, long, default_value_t = String::from("tcp://127.0.0.1:8080"))]
    pub_addr : String
}

// Example command: cargo run --bin vehicle_node -- --logging --velocity 35.0 --efficiency 7.0 --fuel 55.0 --dt 0.01

fn main() -> anyhow::Result<()>{

    let cli = Args::parse();

    let context = zmq::Context::new();
    let socket = context.socket(zmq::PUB)?;
    socket.connect(&cli.pub_addr)?;

    let mut car = Vehicle::default();
    let goal_position : (f64, f64) = (100.0, 150.0);
    let starting_heading = goal_position.1.atan2(goal_position.0);
    car.set_heading(starting_heading);
    car.set_velocity(cli.velocity);          // [m/s]
    car.set_fuel_efficiency(cli.efficiency); // [m/L]
    car.set_fuel(cli.fuel);                  // [L]
    car.add_waypoint(&goal_position);
    car.add_waypoint(&(105.0, 155.0));
    car.add_waypoint(&(105.0, 185.0));
    car.add_waypoint(&(135.0, 185.0));
    car.set_logging_enabled(cli.logging);
    car.initialize(); 

    let period = Duration::from_secs_f64(cli.dt * 5.0);

    loop{

        let start = Instant::now();

        car.update(cli.dt);

        let tm = car.populate();

        let json = serde_json::to_string(&tm)?;
        socket.send(json.as_bytes(), 0)?;

        if car.complete() || car.fuel() <= vehicle::EMPTY{
            break;
        }

        let elapsed = start.elapsed();
        if elapsed < period{
            thread::sleep(period - elapsed);
        }

    }

    Ok(())
}