use clap::Parser;
use nav_simulator::satellite::orbit;
use nav_simulator::util;
use nav_simulator::util::Telemetry;

use std::collections::HashMap;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long, default_value_t = String::from("tcp://127.0.0.1:8080"))]
    sub_addr: String,
}

#[allow(unreachable_code)]
fn main() -> anyhow::Result<()> {
    let cli = Args::parse();

    let context = zmq::Context::new();
    let socket = context.socket(zmq::SUB)?;
    socket.bind(&cli.sub_addr)?;
    println!(
        "{}:: binding subscriber to {}\n",
        env!("CARGO_BIN_NAME"),
        cli.sub_addr
    );

    // Subscribe to all messages
    socket.set_subscribe(b"")?;

    let mut sat_frames: HashMap<u64, HashMap<u8, Telemetry>> = HashMap::new();
    let mut car_frames: HashMap<u64, Telemetry> = HashMap::new();

    loop {
        // Get the raw zmq bytes and deserialize into the Telemetry enum
        let bytes = socket.recv_bytes(0)?;
        let msg: Telemetry = serde_json::from_slice(&bytes)?;

        let frame = match msg {
            Telemetry::SATELLITE {
                id,
                x: _,
                y: _,
                t: _,
                r: _,
                frame,
            } => {
                //println!("TM from Satellite {}", msg);
                sat_frames.entry(frame).or_default().insert(id, msg);
                frame
            }
            Telemetry::VEHICLE {
                x,
                y,
                fuel,
                t,
                frame,
            } => {
                //println!("TM from Vehicle {}", msg);
                let global_tm = Telemetry::VEHICLE {
                    x: x + orbit::EARTH_RADIUS_AVG,
                    y,
                    fuel,
                    t,
                    frame,
                };
                car_frames.insert(frame, global_tm);
                frame
            }
        };

        // Check whether this frame is ready
        // frame comes from the matched message
        if let (Some(sats), Some(car)) = (sat_frames.get(&frame), car_frames.get(&frame)) {
            if sats.len() == 3 {
                let car_pos = if let Telemetry::VEHICLE { x, y, .. } = car {
                    (*x, *y)
                } else {
                    continue;
                };

                let mut trilateration_inputs: Vec<Telemetry> = Vec::new();

                for (id, sat) in sats {
                    if let Telemetry::SATELLITE {
                        id: _,
                        x,
                        y,
                        t,
                        r: _,
                        frame,
                    } = sat
                    {
                        let sat_pos = (*x, *y);
                        let r_calc = util::compute_2_d_range(&sat_pos, &car_pos);
                        trilateration_inputs.push(Telemetry::SATELLITE {
                            id: *id,
                            x: *x,
                            y: *y,
                            t: *t,
                            r: r_calc,
                            frame: *frame,
                        });
                    }
                }

                if trilateration_inputs.len() == 3 {
                    let (mut x_est, y_est) =
                        Telemetry::compute_trilateration(&trilateration_inputs);
                    x_est -= orbit::EARTH_RADIUS_AVG;
                    println!("Frame:{frame}, Trilateration resulted in x={x_est}, y={y_est}");
                }

                // Done
                sat_frames.remove(&frame);
                car_frames.remove(&frame);
            }
        }
    }

    Ok(())
}
