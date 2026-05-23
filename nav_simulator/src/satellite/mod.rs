pub mod orbit;
use crate::Orbit;

#[derive(Default)]
#[allow(dead_code)]
pub struct Satellite{
    id        : u8,
    x         : f64,
    y         : f64,
    r         : f64,
    vx        : f64,
    vy        : f64,
    timestamp : f64,
    theta     : f64,
    frame     : u64,
    init      : bool
}

#[allow(dead_code)]
impl Satellite{

    pub fn id(&self) -> u8{
        self.id
    }

    pub fn position(&self) -> (f64, f64){
        (self.x, self.y)
    }

    pub fn range(&self) -> f64{
        self.r
    }

    pub fn timestamp(&self) -> f64{
        self.timestamp
    }

    pub fn frame(&self) -> u64{
        self.frame
    }

    pub fn set_position(&mut self, input : (f64, f64)){
        self.x = input.0;
        self.y = input.1;
    }

    pub fn set_range(&mut self, input : f64){
        self.r = input;
    }

    pub fn set_id(&mut self, input : u8){
        self.id = input;
    }

    pub fn set_timestamp(&mut self, input : f64){
        self.timestamp = input;
    }

    pub fn set_frame(&mut self, input : u64){
        self.frame = input;
    }

    pub fn compute_trilateration(sat1 : &Satellite, sat2 : &Satellite, sat3 : &Satellite) -> (f64, f64){
        let x1 = sat1.x;
        let x2 = sat2.x;
        let x3 = sat3.x;

        let y1 = sat1.y;
        let y2 = sat2.y;
        let y3 = sat3.y;

        let r1 = sat1.r;
        let r2 = sat2.r;
        let r3 = sat3.r;

        let a = 2.0*(x2 - x1);
        let b = 2.0*(y2 - y1);
        let c = r1.powi(2) - r2.powi(2) - x1.powi(2) + x2.powi(2) - y1.powi(2) + y2.powi(2);
        let d = 2.0*(x3- x2);
        let e = 2.0*(y3 - y2);
        let f = r2.powi(2) - r3.powi(2) - x2.powi(2) + x3.powi(2) - y2.powi(2) + y3.powi(2);

        let first = (c*e - f*b) / (e*a - b*d);
        let second = (c*d - a*f) / (b*d - a*e);

        (first, second)

    }

    pub fn compute_range(&mut self, input : &(f64, f64)){
        let rel_pos : (f64, f64) = (self.x - input.0, self.y - input.1);
        self.r = rel_pos.0.hypot(rel_pos.1);
    }

    pub fn initialize(&mut self, orbit_type : Orbit, rotation_angle : f64){

        // Angle (out of 360) along the assumed circular orbit
        let angle_rad = rotation_angle.to_radians();

        let v = (0.0, Orbit::compute_orbtial_velocity(orbit_type));

        // Apply the rotation
        let x_prime = self.x * angle_rad.cos() - self.y * angle_rad.sin();
        let y_prime = self.x * angle_rad.sin() + self.y * angle_rad.cos();

        let vx_prime = v.0 * angle_rad.cos() - v.1 * angle_rad.sin();
        let vy_prime = v.0 * angle_rad.sin() + v.1 * angle_rad.cos();

        // Initialize the satellite's starting position
        self.x = x_prime;
        self.y = y_prime;

        // Initialize the satellite's starting velocity
        self.vx = vx_prime;
        self.vy = vy_prime;

        // Initialize the angle along the orbit
        self.theta = self.y.atan2(self.x);

        // Initialization complete
        self.init = true;

        println!("----Initial Values----");
        println!("Position components = x = {}, y = {}", self.x, self.y);
        println!("Velocity components = x = {}, y = {}", self.vx, self.vy);
        println!("Angle = {} degrees\n", self.theta.to_degrees());

    }

    pub fn update(&mut self, dt : f64) {

        println!("Orbit for Satellite #{} @ t = {}", self.id, self.timestamp);

        // States
        let mut theta = (self.y.atan2(self.x)).to_degrees();

        let original_angle = self.theta;

        println!("----Current Values----");
        println!("Position components = x = {}, y = {}", self.x, self.y);
        println!("Velocity components = x = {}, y = {}", self.vx, self.vy);
        println!("Angle = {theta} degrees\n");

        self.timestamp += dt;

        self.frame += 1;

        let acc = Orbit::compute_gravitational_acceleration(&self.position());
        println!("Acceleration components = x = {}, y = {}", acc.0, acc.1);

        self.vx += acc.0*dt;
        self.vy += acc.1*dt;
        println!("Velocity components = x = {}, y = {}", self.vx, self.vy);

        self.x += self.vx*dt;
        self.y += self.vy*dt;
        println!("Position components = x = {}, y = {}", self.x, self.y);

        theta = (self.y.atan2(self.x)).to_degrees();
        println!("Angular difference from starting position = {} degrees\n", (theta - original_angle).abs());

    }

}