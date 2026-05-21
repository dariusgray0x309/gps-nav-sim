mod satellite;
mod vehicle;
mod util;

use satellite::orbit::*;

#[allow(unused_imports)]
use crate::satellite::Satellite;

#[allow(unused_imports)]
use std::{thread, time::Duration};

fn main(){
    
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

    println!("TEST 2: Orbit simulation");

    // Need to pass ownership to the thread because it's possible that sat1
    // can get deallocated prior to the thread finishing what it needs to do
    // resulting in a dangling reference. The fix is the "move" keyword
    let sim1 = thread::spawn(move ||{
        println!("Thread 1");
        sat1.simulate_orbit(Orbit::GEO, 0.0);
        thread::sleep(Duration::from_millis(5));
    });

    let sim2 = thread::spawn(move ||{
        println!("Thread 2");
        sat2.simulate_orbit(Orbit::GEO, 90.0);
    });

    let sim3 = thread::spawn(move ||{
        println!("Thread 3");
        sat3.simulate_orbit(Orbit::GEO, 180.0);
    });

    println!("TEST 3: Vehicle simulation");

    let mut car = vehicle::Vehicle::default();
    let goal_position : (f64, f64) = (100.0, 150.0);
    //let starting_heading = goal_position.1.atan2(goal_position.0);
    //car.set_heading(starting_heading);
    car.set_velocity(35.0);  // [m/s]
    car.set_fuel_rate(7.0);  // [m/L]
    car.set_fuel(55.0);      // [L]

    let sim4 = thread::spawn(move ||{
        println!("Thread 4");
        car.simulate_motion(&goal_position);
    });

    // Wait for each thread to finish
    sim1.join().unwrap();
    sim2.join().unwrap();
    sim3.join().unwrap();
    sim4.join().unwrap();

}