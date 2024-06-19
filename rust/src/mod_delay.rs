use std::f64::consts::PI;

#[cxx::bridge(namespace = "wprust::rust::mod")]
mod ffi {
    struct LFO {
        frequency: f64,
        counter: f64,
        phase_inc: f64,
        sample_rate: f64,
    }

    struct ModulatedDelayParameters {
        rate: f64,
        depth: f64,
        feedback: f64,
    }

    struct ModulatedDelay {
        params: ModulatedDelayParameters,
        lfo: LFO,
        // delay
    }

    extern "Rust" {
        fn reset(filter: &mut ModulatedDelay, sample_rate: f64);

        fn set_parameters(filter: &mut ModulatedDelay, params: ModulatedDelayParameters);

        fn process(filter: &mut ModulatedDelay, xn: f64) -> f64;
    }
}

fn parabolic_sin(x: f64) -> f64 {
    const B: f64 = 4.0 / PI;
    const C: f64 = -4.0 / (PI * PI);
    const P: f64 = 0.225;

    let y = B * x + C * x * x.abs();
    P * (y * y.abs() - y) + y
}

impl ffi::LFO {
    fn reset(&mut self, sample_rate: f64) {
        self.sample_rate = sample_rate;
        self.phase_inc = self.frequency / sample_rate;
        self.counter = 0.0;
    }

    fn render(&mut self) -> f64 {
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

fn reset(_filter: &mut ffi::ModulatedDelay, _sample_rate: f64) {}

fn set_parameters(_filter: &mut ffi::ModulatedDelay, _params: ffi::ModulatedDelayParameters) {}

fn process(_filter: &mut ffi::ModulatedDelay, _xn: f64) -> f64 {
    0.0
}
