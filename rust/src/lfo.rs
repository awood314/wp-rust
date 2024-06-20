use std::f64::consts::PI;

fn parabolic_sin(x: f64) -> f64 {
    const B: f64 = 4.0 / PI;
    const C: f64 = -4.0 / (PI * PI);
    const P: f64 = 0.225;

    let y = B * x + C * x * x.abs();
    P * (y * y.abs() - y) + y
}

#[derive(Default)]
pub struct LFO {
    pub frequency: f64,
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

        parabolic_sin(-angle)
    }
}
