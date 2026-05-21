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
