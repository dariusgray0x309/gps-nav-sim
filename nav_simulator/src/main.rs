mod satellite;
mod vehicle;
mod util;

use satellite::orbit::*;

use crate::satellite::{Satellite, orbit};

use crate::util::*;

#[allow(unused_imports)]
use std::{thread, time::Duration, collections::HashMap};
use std::sync::{Arc, Barrier, mpsc};

fn main(){

    let (sender, receiver) = mpsc::channel();
    let sender_1 = sender.clone();
    let sender_2 = sender.clone();
    let sender_3 = sender.clone();
    let sender_4 = sender.clone();

    // Arc : shared ownership between threads
    // Barrier : wait for (n) threads
    let num_threads : usize = 4;
    let barrier = Arc::new(Barrier::new(num_threads));
    let barrier_1 = barrier.clone();
    let barrier_2 = barrier.clone();
    let barrier_3 = barrier.clone();
    let barrier_4 = barrier.clone();

    let mut sat_frames : HashMap<u64, Vec<Telemetry>> = HashMap::new();
    let mut car_frames : HashMap<u64, Telemetry> = HashMap::new();

    let default_alt = Orbit::geocentric_altitude(Orbit::GEO);

    let step_size = 0.01;

    // long pause for debugging
    //thread::sleep(Duration::from_millis(10000));

    println!("TEST 3: Orbit simulation");

    let stop_time = 20.0;

    // Need to pass ownership to the thread because it's possible that sat1
    // can get deallocated prior to the thread finishing what it needs to do
    // resulting in a dangling reference. The fix is the "move" keyword
    let sim1 = thread::spawn(move ||{
        println!("Thread 1");
        let mut sat1 = Satellite::default();
        sat1.set_id(1);
        sat1.initialize(Orbit::GEO, 0.0);
        sat1.set_position((default_alt, 0.0));
        loop{
            sat1.update(step_size);
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
        let mut sat2 = Satellite::default();
        sat2.set_id(2);
        sat2.set_position((default_alt, 0.0));
        sat2.initialize(Orbit::GEO, 30.0);
        loop {
            sat2.update(step_size);
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
        let mut sat3 = Satellite::default();
        sat3.set_id(3);
        sat3.set_position((default_alt, 0.0));
        sat3.initialize(Orbit::GEO, 60.0);
        loop{
            sat3.update(step_size);
            let tm = sat3.populate();
            //thread::sleep(Duration::from_millis(1000));
            sender_3.send(tm).unwrap();
            barrier_3.wait();

            if sat3.timestamp() >= stop_time{
                break;
            }
        }
    });

    let sim4 = thread::spawn(move ||{
        println!("Thread 4");
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
        car.initialize();        

        loop{
            car.update(step_size);
            let tm = car.populate();
            //thread::sleep(Duration::from_millis(1000));
            sender_4.send(tm).unwrap();
            barrier_4.wait();

            if car.complete(){
                break;
            }
        }
    });

    // Wait for each thread to finish
    sim1.join().unwrap();
    sim2.join().unwrap();
    sim3.join().unwrap();
    sim4.join().unwrap();

    println!("TEST 4: Output telemetry");

    // receiver.recv waits forever until the next message
    // receiver.try_recv will exit immediately if there isn't a new message
    while let Ok(msg) = receiver.try_recv(){
        match msg{ 
            util::Telemetry::SATELLITE { id: _, x: _, y: _, t: _, r: _, frame} =>{
                println!("TM from Satellite {}", msg);
                sat_frames.entry(frame).or_default().push(msg)
            },
            util::Telemetry::VEHICLE { mut x, y: _, fuel: _, t: _, frame } => {
                println!("TM from Vehicle {}", msg);
                x += orbit::EARTH_RADIUS_AVG; // convert from local to global
                car_frames.insert(frame, msg);
            }
        }
    }

    for (frame, sats) in &sat_frames{

        if sats.len() != 3{
            println!("This doesn't have 3 elements!");
        }

        println!("val_1={}, val_2={}, val_3={}", sats[0], sats[1], sats[2]);
        //let (x, y) = util::Telemetry::compute_trilateration(sats);

        //println!("Frame:{}, Trilateration resulted in x={x}, y={y}", frame);

        let car = car_frames.get(frame);
        println!("car tm = {}", car.unwrap());
    }

}