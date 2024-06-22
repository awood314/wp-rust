use std::f64::consts::PI;

use crate::util;

#[derive(Default)]
pub struct LFO {
    frequency: f64,
    counter: f64,
    phase_inc: f64,
    sample_rate: f64,
}

impl LFO {
    pub fn reset(&mut self, sample_rate: f64) {
        self.sample_rate = sample_rate;
        self.phase_inc = self.frequency / sample_rate;
        self.counter = 0.0;
    }

    pub fn frequency(&mut self, frequency: f64) {
        self.frequency = frequency;
        self.phase_inc = self.frequency / self.sample_rate;
    }

    pub fn render(&mut self) -> f64 {
        // Sin wave
        let angle = self.counter * 2.0 * PI - PI;

        // increment counter and wrap
        self.counter += self.phase_inc;
        self.counter = match self.counter {
            counter if counter >= 1.0 => counter - 1.0,
            counter if counter <= 0.0 => counter + 1.0,
            _ => self.counter,
        };

        util::parabolic_sin(-angle)
    }
}
