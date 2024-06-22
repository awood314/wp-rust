use crate::delay::Delay;
use crate::lfo::LFO;
use crate::util::{self, db_to_scalar};

#[cxx::bridge(namespace = "wprust::rust::mod")]
mod ffi {
    #[derive(Default)]
    struct ModulatedDelayParameters {
        rate: f64,
        depth_pct: f64,
        feedback: f64,
    }

    extern "Rust" {
        type ModulatedDelay;

        fn create_modulated_delay() -> *mut ModulatedDelay;

        fn reset(self: &mut ModulatedDelay, sample_rate: f64);

        fn set_parameters(self: &mut ModulatedDelay, params: ModulatedDelayParameters);

        fn process(self: &mut ModulatedDelay, xn: f64) -> f64;
    }
}

#[derive(Default)]
struct ModulatedDelay {
    sample_rate: f64,
    depth: f64,
    lfo: LFO,
    delay: Delay,
}

fn create_modulated_delay() -> *mut ModulatedDelay {
    Box::into_raw(Box::new(ModulatedDelay::default()))
}

impl ModulatedDelay {
    fn reset(self: &mut ModulatedDelay, sample_rate: f64) {
        self.delay.create_delay_buffer(sample_rate, 100.0);

        self.lfo.reset(sample_rate);

        self.sample_rate = sample_rate;
    }

    fn set_parameters(self: &mut ModulatedDelay, params: ffi::ModulatedDelayParameters) {
        self.depth = params.depth_pct / 100.0;
        self.lfo.frequency(params.rate);
        self.delay.feedback = params.feedback / 100.0;

        // flanger
        self.delay.wet = db_to_scalar(-3.0);
        self.delay.dry = db_to_scalar(-3.0);
    }

    fn process(self: &mut ModulatedDelay, xn: f64) -> f64 {
        // next lfo
        let lfo_signal = self.lfo.render();

        // flanger
        let mod_value = util::bipolar_to_unipolar(self.depth * lfo_signal);
        const MIN_DELAY: f64 = 0.1;
        const MAX_DEPTH: f64 = 7.0;
        let delay_ms =
            util::bipolar_modulation_from_min(mod_value, MIN_DELAY, MIN_DELAY + MAX_DEPTH);
        self.delay.delay = (delay_ms / 1000.0) * self.sample_rate;

        self.delay.process(xn)
    }
}
