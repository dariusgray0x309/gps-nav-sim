pub const UNIVERSAL_GRAV_CONSTANT: f64 = 6.67430e-11; // [m^3 kg^-1 s^-2]
pub const EARTH_MASS: f64 = 5.97219e24; // [kg]
pub const MU_EARTH: f64 = (UNIVERSAL_GRAV_CONSTANT * EARTH_MASS) * 1e-9; // [km^3 s^-2]
pub const EARTH_RADIUS_AVG: f64 = 6371.0088; // [km] == avg(equitorial & polar)

#[derive(Clone, Copy)]
#[allow(dead_code)]
pub enum Orbit {
    LEO,
    MEO,
    GEO,
    HEO,
}

#[allow(dead_code)]
impl Orbit {
    pub fn geodedic_altitude(orbit_type: Orbit) -> f64 {
        match orbit_type {
            Orbit::LEO => 1080.0,  // [km]
            Orbit::MEO => 15000.0, // [km]
            Orbit::GEO => 35786.0, // [km]
            Orbit::HEO => 0.0, // not true at all, but it varies a lot. Don't intend to impelement
        }
    }

    pub fn geocentric_altitude(orbit_type: Orbit) -> f64 {
        let alt = Orbit::geodedic_altitude(orbit_type);
        alt + EARTH_RADIUS_AVG
    }

    pub fn compute_orbtial_velocity(orbit_type: Orbit) -> f64 {
        let alt = Orbit::geocentric_altitude(orbit_type);
        (MU_EARTH / alt).sqrt() // [km/s]
    }

    pub fn compute_gravitational_acceleration(position: &(f64, f64)) -> (f64, f64) {
        let mag = position.0.hypot(position.1);
        let accel = -MU_EARTH / (mag * mag * mag);
        (accel * position.0, accel * position.1)
    }

    pub fn compute_orbital_period(orbit_type: Orbit) -> f64 {
        let r = Orbit::geocentric_altitude(orbit_type);
        (2.0 * std::f64::consts::PI * r) / Orbit::compute_orbtial_velocity(orbit_type)
    }
}
