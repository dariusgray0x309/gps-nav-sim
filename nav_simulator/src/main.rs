mod satellite;
mod vehicle;
mod util;

use satellite::orbit::*;

use crate::satellite::Satellite;

use crate::util::*;

use std::{thread, time::Duration, collections::HashMap};
use std::sync::{mpsc, Arc, Barrier};


fn main(){

    let (sender, receiver) = mpsc::channel();
    let sender_1 = sender.clone();
    let sender_2 = sender.clone();
    let sender_3 = sender.clone();

    // Arc : shared ownership between threads
    // Barrier : wait for (n) threads
    let barrier = Arc::new(Barrier::new(3));
    let barrier_1 = barrier.clone();
    let barrier_2 = barrier.clone();
    let barrier_3 = barrier.clone();

    let mut frames : HashMap<u64, Vec<Telemetry>> = HashMap::new();
    
    let mut sat1 = Satellite::default();
    let mut sat2 = Satellite::default();
    let mut sat3 = Satellite::default();

    sat1.set_id(1);
    sat2.set_id(2);
    sat3.set_id(3);

    sat1.set_position((0.0, 100.0));
    sat2.set_position((100.0, 0.0));
    sat3.set_position((100.0, 100.0));

    sat1.set_range(67.08);
    sat2.set_range(80.62);
    sat3.set_range(92.20);

    println!("TEST 1: Trilateration");
    let car_position = Satellite::compute_trilateration(&sat1, &sat2, &sat3);
    println!("Based on the satellite's measurements, the vehicle is at x={}, y={}\n", car_position.0, car_position.1);

    let default_alt = Orbit::geocentric_altitude(Orbit::GEO);

    sat1.set_position((default_alt, 0.0));
    sat2.set_position((default_alt, 0.0));
    sat3.set_position((default_alt, 0.0));


    println!("TEST 2: Vehicle simulation");
    let mut car = vehicle::Vehicle::default();
    let goal_position : (f64, f64) = (100.0, 150.0);
    let starting_heading = goal_position.1.atan2(goal_position.0);
    car.set_heading(starting_heading);
    car.set_velocity(35.0);  // [m/s]
    car.set_fuel_rate(7.0);  // [m/L]
    car.set_fuel(55.0);      // [L]
    car.add_waypoint(&goal_position);
    car.add_waypoint(&(105.0, 155.0));
    car.add_waypoint(&(105.0, 185.0));
    car.add_waypoint(&(135.0, 185.0));
    car.simulate_motion();

    let last_car_position = car.position();

    println!("TEST 3: Orbit simulation");

    let step_size = 0.01;

    let stop_time = 20.0;

    // Need to pass ownership to the thread because it's possible that sat1
    // can get deallocated prior to the thread finishing what it needs to do
    // resulting in a dangling reference. The fix is the "move" keyword
    let sim1 = thread::spawn(move ||{
        println!("Thread 1");
        sat1.initialize(Orbit::GEO, 0.0);
        loop{
            sat1.update(step_size);
            sat1.compute_range(&last_car_position);
            let tm = sat1.populate();
            //thread::sleep(Duration::from_millis(1000));
            sender_1.send(tm).unwrap();
            barrier_1.wait();

            if sat1.timestamp() >= stop_time {
                break;
            }
        }
    });

    let sim2 = thread::spawn(move ||{
        println!("Thread 2");
        sat2.initialize(Orbit::GEO, 90.0);
        loop {
            sat2.update(step_size);
            sat2.compute_range(&last_car_position);
            let tm = sat2.populate();
            //thread::sleep(Duration::from_millis(1000));
            sender_2.send(tm).unwrap();
            barrier_2.wait();

            if sat2.timestamp() >= stop_time {
                break;
            }
        }
    });

    let sim3 = thread::spawn(move ||{
        println!("Thread 3");
        sat3.initialize(Orbit::GEO, 180.0);
        loop{
            sat3.update(step_size);
            sat3.compute_range(&last_car_position);
            let tm = sat3.populate();
            //thread::sleep(Duration::from_millis(1000));
            sender_3.send(tm).unwrap();
            barrier_3.wait();

            if sat3.timestamp() >= stop_time{
                break;
            }
        }
    });

    // Wait for each thread to finish
    sim1.join().unwrap();
    sim2.join().unwrap();
    sim3.join().unwrap();

    println!("TEST 4: Output telemetry");

    // receiver.recv waits forever until the next message
    // receiver.try_recv will exit immediately if there isn't a new message
    while let Ok(msg) = receiver.try_recv(){
        if let util::Telemetry::SATELLITE { id: _, x: _, y: _, t: _, r: _, frame} = msg{
            //println!("TM from Satellite {}", msg);
            frames.entry(frame).or_default().push(msg);
        }
    }

    for (key, val) in &frames{

        if val.len() != 3{
            println!("This doesn't have 3 elements!");
        }

        //println!("val_1={}, val_2={}, val_3={}", val[0], val[1], val[2]);
        let (x, y) = util::Telemetry::compute_trilateration(val);

        println!("Frame:{}, Trilateration resulted in x={x}, y={y}", key);
    }

}