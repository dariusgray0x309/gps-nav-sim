use clap::Parser;
use std::{thread, time::{Duration, Instant}};
use nav_simulator::satellite::{Satellite, orbit::Orbit};
use nav_simulator::util::Populate;

#[derive(Parser, Debug)]
struct Args{
    #[arg(short, long)] // refers to CLI syntax "short -i" or "long --id"
    id : u8,

    #[arg(long, default_value_t = 0.0)]
    phase_angle : f64,

    #[arg(short, long, default_value_t = false)]
    logging : bool,

    #[arg(short, long, default_value_t = 0.01)]
    dt : f64,

    #[arg(short, long)]
    stop_time : Option<f64>,

    #[arg(short, long, default_value_t = String::from("tcp://127.0.0.1:8080"))]
    pub_addr : String
}

// Example command: cargo run --bin sat_node -- --id 1 --phase-angle 0.0 --logging --stop-time 20.0

fn main() -> anyhow::Result<()>{

    let cli = Args::parse();

    let context = zmq::Context::new();
    let socket = context.socket(zmq::PUB)?;
    socket.connect(&cli.pub_addr)?;

    let default_alt = Orbit::geocentric_altitude(Orbit::GEO);

    let mut sat = Satellite::default();
    sat.set_id(cli.id);
    sat.set_position((default_alt, 0.0));
    sat.set_logging_enabled(cli.logging);
    sat.initialize(Orbit::GEO, cli.phase_angle);

    let period = Duration::from_secs_f64(cli.dt * 10.0);

    loop{
        let start = Instant::now();

        sat.update(cli.dt);

        let tm = sat.populate();

        let json = serde_json::to_string(&tm)?;
        socket.send(json.as_bytes(), 0)?;

        if let Some(stop_time) = cli.stop_time{
            if sat.timestamp() >= stop_time {
                break;
            }
        }

        let elapsed = start.elapsed();
        if elapsed < period{
            thread::sleep(period - elapsed);
        }
    }

    Ok(())

}