use std::f64::consts::PI;

#[cxx::bridge(namespace = "wprust::rust::mod")]
mod ffi {
    struct ModulatedDelayParameters {
        rate: f64,
        depth: f64,
        feedback: f64,
    }

    extern "Rust" {
        type ModulatedDelay;

        fn reset(filter: &mut ModulatedDelay, sample_rate: f64);

        fn set_parameters(filter: &mut ModulatedDelay, params: ModulatedDelayParameters);

        fn process(filter: &mut ModulatedDelay, xn: f64) -> f64;
    }
}

struct CircularBuffer<T: Copy + Default> {
    buffer: Vec<T>,
    write_index: usize,
    length: usize,
    mask: usize,
}

impl<T: Copy + Default> CircularBuffer<T> {
    fn create(&mut self, length: usize) {
        self.write_index = 0;

        // nearest power of 2
        self.length = length.ilog2().div_ceil(2_u32.ilog2()).pow(2) as usize;

        self.mask = self.length - 1;
        self.buffer.resize(self.length, Default::default());
    }

    fn read_buffer(&mut self, delay: usize) -> T {
        let read_index = (self.write_index - delay) & self.mask;
        self.buffer[read_index]
    }

    fn write_buffer(&mut self, input: T) {
        self.buffer[self.write_index] = input;
        self.write_index = (self.write_index + 1) & self.mask;
    }

    // TODO: linear interpolation read
}

struct AudioDelay {
    buffer_l: CircularBuffer<f64>,
}

struct LFO {
    frequency: f64,
    counter: f64,
    phase_inc: f64,
    sample_rate: f64,
}

fn parabolic_sin(x: f64) -> f64 {
    const B: f64 = 4.0 / PI;
    const C: f64 = -4.0 / (PI * PI);
    const P: f64 = 0.225;

    let y = B * x + C * x * x.abs();
    P * (y * y.abs() - y) + y
}

impl LFO {
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

struct ModulatedDelay {
    params: ffi::ModulatedDelayParameters,
    lfo: LFO,
    delay: AudioDelay,
}

impl ModulatedDelay {}

fn reset(_filter: &mut ModulatedDelay, _sample_rate: f64) {}

fn set_parameters(_filter: &mut ModulatedDelay, _params: ffi::ModulatedDelayParameters) {}

fn process(_filter: &mut ModulatedDelay, _xn: f64) -> f64 {
    0.0
}
