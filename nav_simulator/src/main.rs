mod satellite;
mod vehicle;
mod util;

use satellite::orbit::*;

use crate::satellite::{Satellite, orbit};

use crate::util::*;

#[allow(unused_imports)]
use std::{thread, time::Duration, collections::HashMap};
use std::sync::{Arc, Barrier, mpsc, atomic::{AtomicBool, Ordering}};

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

    let stop = Arc::new(AtomicBool::new(false));
    let stop_1 = stop.clone();
    let stop_2 = stop.clone();
    let stop_3 = stop.clone();
    let stop_4 = stop.clone();

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

            println!("Sat 1 updating");
            sat1.update(step_size);

            println!("Sat 1 populating tm");
            let tm = sat1.populate();
            //thread::sleep(Duration::from_millis(1000));

            println!("Barrier 1 (Sat 1) waiting");
            barrier_1.wait();

            println!("Sender 1 (Sat 1) tm sending");
            sender_1.send(tm).unwrap();

            println!("Stop 1 (Sat 1) storing true");
            if sat1.timestamp() >= stop_time {
                stop_1.store(true, Ordering::Relaxed);
            }

            println!("Stop 1 (Sat 1) breaking loop");
            if stop_1.load(Ordering::Relaxed){
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

            println!("Sat 2 updating");
            sat2.update(step_size);

            println!("Sat 2 populating tm");
            let tm = sat2.populate();
            //thread::sleep(Duration::from_millis(1000));

            println!("Barrier 2 (Sat 2) waiting");
            barrier_2.wait();

            println!("Sender 2 (Sat 2) tm sending");
            sender_2.send(tm).unwrap();

            println!("Stop 2 (Sat 2) storing true");
            if sat2.timestamp() >= stop_time {
                stop_2.store(true, Ordering::Relaxed);
            }

            println!("Stop 2 (Sat 2) breaking loop");
            if stop_2.load(Ordering::Relaxed){
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

            println!("Sat 3 updating");
            sat3.update(step_size);

            println!("Sat 3 populating tm");
            let tm = sat3.populate();
            //thread::sleep(Duration::from_millis(1000));

            println!("Barrier 3 (Sat 3) waiting");
            barrier_3.wait();

            println!("Sender 3 (Sat 3) tm sending");
            sender_3.send(tm).unwrap();

            if sat3.timestamp() >= stop_time{
                println!("Stop 3 (Sat 3) storing true");
                stop_3.store(true, Ordering::Relaxed);
            }

            println!("Stop 3 (Sat 3) breaking loop");
            if stop_3.load(Ordering::Relaxed){
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

            println!("Car updating");
            car.update(step_size);

            println!("Car populating tm");
            let tm = car.populate();
            //thread::sleep(Duration::from_millis(1000));

            println!("Barrier 4 (Car) waiting");
            barrier_4.wait();

            println!("Sender 4 (Car) tm sending");
            sender_4.send(tm).unwrap();

            if car.complete(){
                println!("Stop 4 (Car) storing true");
                stop_4.store(true, Ordering::Relaxed);
            }

            println!("Stop 4 (Car) breaking loop");
            if stop_4.load(Ordering::Relaxed){
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
                //println!("TM from Satellite {}", msg);
                sat_frames.entry(frame).or_default().push(msg)
            },
            util::Telemetry::VEHICLE { x , y , fuel , t , frame } => {
                //println!("TM from Vehicle {}", msg);
                let global_tm = util::Telemetry::VEHICLE { x: x + orbit::EARTH_RADIUS_AVG, y, fuel, t, frame };
                car_frames.insert(frame, global_tm);
            }
        }
    }

    for (frame, sats) in &sat_frames{

        if sats.len() != 3{
            println!("This doesn't have 3 elements!");
        }

        println!("sat_1={} ", sats[0]);
        println!("sat_2={} ", sats[1]);
        println!("sat_3={} ", sats[2]);
        //let (x, y) = util::Telemetry::compute_trilateration(sats);

        //println!("Frame:{}, Trilateration resulted in x={x}, y={y}", frame);

        let car = car_frames.get(frame);
        if car.is_some(){
            println!("car tm = {}", car.unwrap());

            let sat_1_pos : (f64, f64);
            if let util::Telemetry::SATELLITE { id , x , y , t , r , frame  } = sats[0]{
                sat_1_pos = (x, y);
                println!("Sat {}, x = {}, y = {}, frame = {}", id, x, y, frame);
            };

            let sat_2_pos : (f64, f64);
            if let util::Telemetry::SATELLITE { id , x , y , t , r , frame  } = sats[1]{
                sat_2_pos = (x, y);
                println!("Sat {}, x = {}, y = {}, frame = {}", id, x, y, frame);
            };

            let sat_3_pos : (f64, f64);
            if let util::Telemetry::SATELLITE { id , x , y , t , r , frame  } = sats[2]{
                sat_3_pos = (x, y);
                println!("Sat {}, x = {}, y = {}, frame = {}", id, x, y, frame);
            };

            let car_pos : (f64, f64);
            if let util::Telemetry::VEHICLE { x, y, fuel, t, frame } = car.unwrap(){
                car_pos = (*x, *y);
                println!("Car x = {}, y = {}, frame = {}", x, y, frame);
            };

            //let r_1 : f64 = util::compute_2_d_range(&sat_1_pos, &car_pos);
//
            //let r_2 : f64 = util::compute_2_d_range(&sat_2_pos, &car_pos);
//
            //let r_3 : f64 = util::compute_2_d_range(&sat_3_pos, &car_pos);

            // need to set r_1, r_2, & r_3 back to sats[0], sats[1], & sats[3]


            //println!("Frame:{}, Trilateration resulted in x={x}, y={y}", frame);

        }

    }

}