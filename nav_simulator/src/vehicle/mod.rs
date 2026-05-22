mod controller;

use controller::ProportionalGain;

#[derive(Default)]
pub struct Vehicle{
    x         : f64,
    y         : f64,
    v         : f64,
    psi       : f64,
    fuel      : f64,
    fuel_rate : f64,
    timestamp : f64,
    waypoints : Vec<(f64, f64)>
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

    pub fn waypoints(&self) -> Vec<(f64, f64)>{
        self.waypoints.clone()
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
        let distance = self.v * dt;
        let fuel_used = distance / self.fuel_rate;
        self.fuel -= fuel_used;
    }

    pub fn compute_relative_position(&mut self, goal : &(f64, f64)) -> (f64, f64){
        let delta_x : f64 = goal.0 - self.x;
        let delta_y : f64 = goal.1 - self.y;
        (delta_x, delta_y)
    }

    pub fn add_waypoint(&mut self, input : &(f64, f64)){
        self.waypoints.push(*input);
    }

    pub fn simulate_motion(&mut self){

        let original_v = self.v;

        let mut time : f64 = 0.0;
        
        let dt : f64 = 0.01;

        let total_fuel : f64 = self.fuel;

        let heading_rate_limit : f64 = (80.0 as f64).to_radians();

        let number_of_waypoints = self.waypoints.len();

        let waypoint_radius = (self.v * dt).max(1.0);

        if number_of_waypoints == 0 {
            println!("No waypoints for guidance!");
            return;
        }

        for (index, goal) in self.waypoints().iter().enumerate(){

            self.v = original_v;
            let mut kp = ProportionalGain::get_gain(ProportionalGain::AGGRESSIVE);

            loop{

                let rel_pos = self.compute_relative_position(goal);

                let distance = rel_pos.0.hypot(rel_pos.1);
                println!("Total distance from goal = {distance}");

                if distance < 10.0{
                    self.v = distance.min(original_v);
                    kp = ProportionalGain::get_gain(ProportionalGain::FAST);
                }

                if distance <= waypoint_radius{
                    println!("Reached waypoint {} out of {}\n", index+1, number_of_waypoints);
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

                let vx = self.v * self.psi.cos();
                let vy = self.v * self.psi.sin();

                // Update the position
                self.x += vx * dt;
                self.y += vy * dt;

                self.update_fuel(dt);

                if self.fuel <= 0.0{
                    println!("No more fuel available -- ending simulation");
                    println!("Completed {}% of the path\n", ((index) as f64 / number_of_waypoints as f64) * 100.0);
                    return;
                }

                println!("Waypoint {} goal: x = {}, y = {}", index+1, goal.0, goal.1);
                println!("Current position @t = {}: x = {}, y = {}", time, self.x, self.y);
                println!("Velocity magnitude = {}", self.v);
                println!("{}% of fuel remaining\n", (self.fuel / total_fuel) * 100.0);

                time += dt;

                self.timestamp = time;

            }

        }

    }

}