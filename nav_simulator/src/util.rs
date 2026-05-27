use crate::satellite;
use crate::vehicle;
use serde::{Deserialize, Serialize};
use std::fmt;

#[allow(dead_code)]
pub fn wrap_angle(mut input: f64) -> f64 {
    while input > std::f64::consts::PI {
        input -= 2.0 * std::f64::consts::PI
    }
    while input < -std::f64::consts::PI {
        input += 2.0 * std::f64::consts::PI
    }
    input
}

pub fn compute_2_d_range(first: &(f64, f64), second: &(f64, f64)) -> f64 {
    let x = first.0 - second.0;
    let y = first.1 - second.1;
    x.hypot(y)
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum Telemetry {
    SATELLITE {
        id: u8,
        x: f64,
        y: f64,
        t: f64,
        r: f64,
        frame: u64,
    },
    VEHICLE {
        x: f64,
        y: f64,
        fuel: f64,
        t: f64,
        frame: u64,
    },
}

pub static NULL: (f64, f64) = (0.0, 0.0);

impl Telemetry {
    pub fn compute_trilateration(input: &Vec<Telemetry>) -> (f64, f64) {
        if input.len() != 3 {
            return NULL;
        }

        let mut sats: Vec<satellite::Satellite> = Vec::new();

        for tm in input {
            if let Telemetry::SATELLITE {
                id,
                x,
                y,
                t,
                r,
                frame,
            } = tm
            {
                let mut temp = satellite::Satellite::default();
                temp.set_id(*id);
                temp.set_position((*x, *y));
                temp.set_timestamp(*t);
                temp.set_range(*r);
                temp.set_frame(*frame);
                sats.push(temp);
            }
        }

        if sats.is_empty() {
            return NULL;
        }

        satellite::Satellite::compute_trilateration(&sats[0], &sats[1], &sats[2])
    }
}

pub trait Populate {
    fn populate(&self) -> Telemetry;
}

impl Populate for vehicle::Vehicle {
    fn populate(&self) -> Telemetry {
        Telemetry::VEHICLE {
            x: self.position().0,
            y: self.position().1,
            fuel: self.fuel_percentage(),
            t: self.timestamp(),
            frame: self.frame(),
        }
    }
}

impl Populate for satellite::Satellite {
    fn populate(&self) -> Telemetry {
        Telemetry::SATELLITE {
            id: self.id(),
            x: self.position().0,
            y: self.position().1,
            t: self.timestamp(),
            r: self.range(),
            frame: self.frame(),
        }
    }
}

impl fmt::Display for Telemetry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Telemetry::VEHICLE {
                x,
                y,
                fuel,
                t,
                frame,
            } => write!(
                f,
                "x={}, y={}, fuel={}, t={}, frame={}",
                x, y, fuel, t, frame
            ),
            Telemetry::SATELLITE {
                id,
                x,
                y,
                t,
                r,
                frame,
            } => write!(
                f,
                "id={}, x={}, y={}, t={}, r={}, frame={}",
                id, x, y, t, r, frame
            ),
        }
    }
}

#[derive(Default, Serialize, Deserialize)]
pub struct OutputMessage {
    id: u8,
    x: f64,
    y: f64,
    r: f64,
    t: f64,
    which: String,
    frame: u64,
    fuel: f64,
}

#[derive(Default)]
pub struct OutputMessageBuilder {
    id: u8,
    x: f64,
    y: f64,
    r: f64,
    t: f64,
    which: String,
    frame: u64,
    fuel: f64,
}

impl OutputMessageBuilder {
    pub fn id(mut self, input: u8) -> Self {
        self.id = input;
        self
    }

    pub fn x(mut self, input: f64) -> Self {
        self.x = input;
        self
    }

    pub fn y(mut self, input: f64) -> Self {
        self.y = input;
        self
    }

    pub fn r(mut self, input: f64) -> Self {
        self.r = input;
        self
    }

    pub fn t(mut self, input: f64) -> Self {
        self.t = input;
        self
    }

    pub fn which(mut self, input: impl Into<String>) -> Self {
        self.which = input.into();
        self
    }

    pub fn frame(mut self, input: u64) -> Self {
        self.frame = input;
        self
    }

    pub fn fuel(mut self, input: f64) -> Self {
        self.fuel = input;
        self
    }

    pub fn build(self) -> OutputMessage {
        OutputMessage {
            id: self.id,
            x: self.x,
            y: self.y,
            r: self.r,
            t: self.t,
            which: self.which,
            frame: self.frame,
            fuel: self.fuel,
        }
    }
}
