mod controller;

use controller::ProportionalGain;

pub static EMPTY: f64 = 1e-6;

#[derive(Default)]
pub struct Vehicle {
    x: f64,
    y: f64,
    v: f64,
    psi: f64,
    fuel: f64,
    fuel_efficiency: f64,
    timestamp: f64,
    frame: u64,
    init: bool,
    complete: bool,
    waypoint_idx: usize,
    waypoints: Vec<(f64, f64)>,
    original_v: f64,
    original_fuel: f64,
    logging: bool,
}

#[allow(dead_code)]
impl Vehicle {
    pub fn position(&self) -> (f64, f64) {
        (self.x, self.y)
    }

    pub fn velocity(&self) -> f64 {
        self.v
    }

    pub fn heading(&self) -> f64 {
        self.psi
    }

    pub fn fuel(&self) -> f64 {
        self.fuel
    }

    pub fn fuel_efficiency(&self) -> f64 {
        self.fuel_efficiency
    }

    pub fn timestamp(&self) -> f64 {
        self.timestamp
    }

    pub fn waypoints(&self) -> Vec<(f64, f64)> {
        self.waypoints.clone()
    }

    pub fn frame(&self) -> u64 {
        self.frame
    }

    pub fn complete(&self) -> bool {
        self.complete
    }

    pub fn set_position(&mut self, input: (f64, f64)) {
        self.x = input.0;
        self.y = input.1;
    }

    pub fn set_velocity(&mut self, input: f64) {
        self.v = input;
    }

    pub fn set_heading(&mut self, input: f64) {
        self.psi = input;
    }

    pub fn set_fuel(&mut self, input: f64) {
        self.fuel = input;
    }

    pub fn set_fuel_efficiency(&mut self, input: f64) {
        self.fuel_efficiency = input;
    }

    pub fn set_timestamp(&mut self, input: f64) {
        self.timestamp = input;
    }

    pub fn set_logging_enabled(&mut self, input: bool) {
        self.logging = input;
    }

    pub fn update_heading(&mut self, heading_rate: f64, dt: f64) {
        self.psi += heading_rate * dt;
    }

    pub fn update_fuel(&mut self, dt: f64) {
        let distance = self.v * dt;
        let fuel_used = distance / self.fuel_efficiency;
        self.fuel -= fuel_used;
    }

    pub fn compute_relative_position(&mut self, goal: &(f64, f64)) -> (f64, f64) {
        let delta_x: f64 = goal.0 - self.x;
        let delta_y: f64 = goal.1 - self.y;
        (delta_x, delta_y)
    }

    pub fn add_waypoint(&mut self, input: &(f64, f64)) {
        self.waypoints.push(*input);
    }

    pub fn initialize(&mut self) {
        if self.waypoints.len() == 0 {
            println!("No waypoints available for guidance!");
            return;
        }

        self.original_v = self.v;

        self.original_fuel = self.fuel;

        self.init = true;

        self.complete = false;
    }

    #[allow(unused_assignments)]
    pub fn update(&mut self, dt: f64) {
        if !self.init {
            println!("The vehicle has not been initialized");
            return;
        }

        if self.waypoint_idx >= self.waypoints.len() {
            if self.logging {
                println!("Complete");
            }
            self.complete = true;
            return;
        }

        let heading_rate_limit: f64 = (80.0 as f64).to_radians();

        let goal = self.waypoints[self.waypoint_idx];

        let rel_pos = self.compute_relative_position(&goal);

        let distance = rel_pos.0.hypot(rel_pos.1);

        if self.logging {
            println!("Total distance from goal = {distance}");
        }

        let mut kp = 0.0;

        let waypoint_radius = (self.original_v * dt).max(1.0);

        if distance < 10.0 {
            self.v = distance;
            kp = ProportionalGain::get_gain(ProportionalGain::FAST);
        } else {
            self.v = self.original_v;
            kp = ProportionalGain::get_gain(ProportionalGain::AGGRESSIVE);
        }

        if distance <= waypoint_radius {
            if self.logging {
                println!(
                    "Reached waypoint {} out of {}\n",
                    self.waypoint_idx + 1,
                    self.waypoints.len()
                );
            }
            self.waypoint_idx += 1;
            return;
        }

        let desired_heading: f64 = rel_pos.1.atan2(rel_pos.0);

        // Proportional controller
        let heading_rate: f64 =
            (kp * (desired_heading - self.psi)).clamp(-heading_rate_limit, heading_rate_limit);

        self.update_heading(heading_rate, dt);

        if self.psi != 0.0 && self.logging {
            println!("Desired heading = {} degrees", desired_heading.to_degrees());
            println!("Current heading = {} degrees", self.psi.to_degrees());
            println!(
                "Turning with heading rate = {} deg/sec",
                heading_rate.to_degrees()
            );
        }

        let vx = self.v * self.psi.cos();
        let vy = self.v * self.psi.sin();

        // Update the position
        self.x += vx * dt;
        self.y += vy * dt;

        self.update_fuel(dt);

        if self.fuel <= 0.0 {
            if self.logging {
                println!("No more fuel available -- ending simulation");
                println!(
                    "Completed {}% of the path\n",
                    ((self.waypoint_idx) as f64 / self.waypoints.len() as f64) * 100.0
                );
            }
            return;
        }

        if self.logging {
            println!(
                "Waypoint {} goal: x = {}, y = {}",
                self.waypoint_idx + 1,
                goal.0,
                goal.1
            );
            println!(
                "Current position @t = {}: x = {}, y = {}",
                self.timestamp, self.x, self.y
            );
            println!("Velocity magnitude = {}", self.v);
            println!(
                "{}% of fuel remaining\n",
                (self.fuel / self.original_fuel) * 100.0
            );
        }

        self.timestamp += dt;

        self.frame += 1;
    }
}
