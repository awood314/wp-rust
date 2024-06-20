use crate::delay::Delay;
use crate::lfo::LFO;

#[cxx::bridge(namespace = "wprust::rust::mod")]
mod ffi {
    #[derive(Default)]
    struct ModulatedDelayParameters {
        rate: f64,
        depth: f64,
        feedback: f64,
    }

    extern "Rust" {
        type ModulatedDelay;

        fn create_modulated_delay() -> Box<ModulatedDelay>;

        fn reset(self: &mut ModulatedDelay, sample_rate: f64);

        fn set_parameters(self: &mut ModulatedDelay, params: ModulatedDelayParameters);

        fn process(self: &mut ModulatedDelay, xn: f64) -> f64;
    }
}

fn bipolar_to_unipolar(value: f64) -> f64 {
    0.5 * value + 0.5
}

fn bipolar_modulation_from_min(value: f64, min: f64, max: f64) -> f64 {
    let value = value.clamp(-1.0, 1.0);
    let half = (max - min) / 2.0;
    let mid = half + min;
    value * half + mid
}

#[derive(Default)]
struct ModulatedDelay {
    params: ffi::ModulatedDelayParameters,
    lfo: LFO,
    delay: Delay,
}

fn create_modulated_delay() -> Box<ModulatedDelay> {
    Box::new(ModulatedDelay::default())
}

impl ModulatedDelay {
    fn reset(self: &mut ModulatedDelay, sample_rate: f64) {
        self.delay.reset(sample_rate);

        self.delay.create_delay_buffer(sample_rate, 100.0);

        self.lfo.reset(sample_rate);
    }

    fn set_parameters(self: &mut ModulatedDelay, params: ffi::ModulatedDelayParameters) {
        self.params = params;
        self.lfo.frequency = self.params.rate;
        self.delay.feedback = self.params.feedback;
    }

    fn process(self: &mut ModulatedDelay, xn: f64) -> f64 {
        // next lfo
        let lfo_signal = self.lfo.render();

        // flanger
        let min_delay = 0.1;
        let max_depth = 7.0;
        let depth = self.params.depth / 100.0;
        let mod_value = bipolar_to_unipolar(depth * lfo_signal);
        self.delay.delay = bipolar_modulation_from_min(mod_value, min_delay, min_delay + max_depth);
        self.delay.process(xn)
    }
}
