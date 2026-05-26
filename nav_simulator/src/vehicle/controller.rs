#[derive(Clone, Copy)]
#[allow(dead_code)]
pub enum ProportionalGain {
    SLOW,
    MODERATE,
    AGGRESSIVE,
    FAST,
}

impl ProportionalGain {
    pub fn get_gain(turn_speed: ProportionalGain) -> f64 {
        match turn_speed {
            ProportionalGain::SLOW => 0.5,
            ProportionalGain::MODERATE => 1.0,
            ProportionalGain::AGGRESSIVE => 2.0,
            ProportionalGain::FAST => 5.0,
        }
    }
}
