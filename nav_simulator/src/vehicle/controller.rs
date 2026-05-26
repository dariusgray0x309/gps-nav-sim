#[derive(Clone, Copy)]
#[allow(dead_code)]
pub enum ProportionalGain {
    Slow,
    Moderate,
    Aggressive,
    Fast,
}

impl ProportionalGain {
    pub fn get_gain(turn_speed: ProportionalGain) -> f64 {
        match turn_speed {
            ProportionalGain::Slow => 0.5,
            ProportionalGain::Moderate => 1.0,
            ProportionalGain::Aggressive => 2.0,
            ProportionalGain::Fast => 5.0,
        }
    }
}
