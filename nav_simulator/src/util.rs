use crate::vehicle;
use crate::satellite;
use std::fmt;

#[allow(dead_code)]
pub fn wrap_angle(mut input : f64) -> f64{
    while input > std::f64::consts::PI {
        input -= 2.0 * std::f64::consts::PI 
    }
    while input < -std::f64::consts::PI {
        input += 2.0 * std::f64::consts::PI 
    }
    input
}

#[derive(Clone, Copy, Debug)]
pub enum Telemetry{
    SATELLITE{
        id : u8,
        x  : f64,
        y  : f64,
        t  : f64
    },
    VEHICLE{
        x    : f64,
        y    : f64,
        fuel : f64,
        t    : f64   
    }
}

pub trait Populate{
    fn populate(&self) -> Telemetry;
}

impl Populate for vehicle::Vehicle{
    fn populate(&self) -> Telemetry{
        Telemetry::VEHICLE{
            x    : self.position().0,
            y    : self.position().1,
            fuel : self.fuel(),
            t    : self.timestamp()
        }
    }
}

impl Populate for satellite::Satellite{
    fn populate(&self) -> Telemetry{
        Telemetry::SATELLITE{
            id : self.id(),
            x  : self.position().0,
            y  : self.position().1,
            t  : self.timestamp()
        }
    }
}

impl fmt::Display for Telemetry {
    fn fmt(&self, f : &mut fmt::Formatter) -> fmt::Result {
        match self {
            Telemetry::VEHICLE {x, y, fuel, t} => write!(f, "x={}, y={}, fuel={}, t={}", x, y, fuel, t),
            Telemetry::SATELLITE {id, x, y, t} => write!(f, "id={}, x={}, y={}, t={}", id, x, y, t)
        }
    }
}