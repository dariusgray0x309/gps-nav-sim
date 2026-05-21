mod controller;

use controller::ProportionalGain;

//use super::util;

#[derive(Default)]
pub struct Vehicle{
    x         : f64,
    y         : f64,
    v         : f64,
    psi       : f64,
    fuel      : f64,
    fuel_rate : f64,
    timestamp : f64
}

#[allow(dead_code)]
impl Vehicle{

    pub fn position(&self) -> (f64, f64){
        (self.x, self.y)
    }

    pub fn velocity(&self) -> f64{
        self.v
    }

    pub fn heading(&self) -> f64{
        self.psi
    }

    pub fn fuel(&self) -> f64{
        self.fuel
    }

    pub fn fuel_rate(&self) -> f64{
        self.fuel_rate
    }

    pub fn timestamp(&self) -> f64{
        self.timestamp
    }

    pub fn set_position(&mut self, input : (f64, f64)){
        self.x = input.0;
        self.y = input.1;
    }

    pub fn set_velocity(&mut self, input : f64){
        self.v = input;
    }

    pub fn set_heading(&mut self, input : f64){
        self.psi = input;
    }

    pub fn set_fuel(&mut self, input : f64){
        self.fuel = input;
    }

    pub fn set_fuel_rate(&mut self, input : f64){
        self.fuel_rate = input;
    }

    pub fn set_timestamp(&mut self, input : f64){
        self.timestamp = input;
    }

    pub fn update_heading(&mut self, heading_rate : f64, dt : f64){
        self.psi += heading_rate * dt;
    }

    pub fn update_fuel(&mut self, dt : f64) {
        self.fuel -= self.fuel_rate * dt;
    }

    pub fn compute_relative_position(&mut self, goal : &(f64, f64)) -> (f64, f64){
        let delta_x : f64 = goal.0 - self.x;
        let delta_y : f64 = goal.1 - self.y;
        (delta_x, delta_y)
    }

    pub fn simulate_motion(&mut self, goal : &(f64, f64)){
        // Initial conditions
        let mut vx = self.v * self.psi.cos();
        let mut vy = self.v * self.psi.sin();

        let mut time : f64 = 0.0;
        let dt : f64 = 0.01;

        let kp = ProportionalGain::get_gain(ProportionalGain::AGGRESSIVE);

        let total_fuel : f64 = self.fuel;

        let heading_rate_limit : f64 = (30.0 as f64).to_radians();

        let epsilon : f64 = 1e-4;

        loop{

            // Update the position
            self.x += vx * dt;
            self.y += vy * dt;

            let rel_pos = self.compute_relative_position(goal);

            if rel_pos.0 < epsilon && rel_pos.1 < epsilon{
                println!("Reached the destination");
                break;
            }

            let desired_heading : f64 = rel_pos.1.atan2(rel_pos.0);

            // Proportional controller
            let heading_rate : f64 = (kp * (desired_heading - self.psi)).clamp(-heading_rate_limit, heading_rate_limit);

            self.update_heading(heading_rate, dt);

            if self.psi != 0.0{
                println!("Desired heading = {} degrees", desired_heading.to_degrees());
                println!("Current heading = {} degrees", self.psi.to_degrees());
                println!("Turning with heading rate = {} deg/sec", heading_rate.to_degrees());
            }

            vx = self.v * self.psi.cos();
            vy = self.v * self.psi.sin();

            self.update_fuel(dt);

            if self.fuel <= 0.0{
                println!("No more fuel available -- ending simulation");
                break;
            }

            println!("Goal: x = {}, y = {}", goal.0, goal.1);
            println!("Current position @t = {}: x = {}, y = {}", time, self.x, self.y);
            println!("{}% of fuel remaining\n", (self.fuel / total_fuel) * 100.0);

            time += dt;

            self.timestamp = time;

            if time > 7.0{
                break;
            }

        }

    }

}