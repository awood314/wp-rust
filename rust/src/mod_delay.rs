#[cxx::bridge(namespace = "wprust::rust::mod")]
mod ffi {
    struct ModulatedDelayParameters {
        rate: f64,
        depth: f64,
        feedback: f64,
    }

    struct ModulatedDelay {
        params: ModulatedDelayParameters,
        // delay
        // lfo
    }

    extern "Rust" {
        fn reset(filter: &mut ModulatedDelay, sample_rate: f64);

        fn set_parameters(filter: &mut ModulatedDelay, params: ModulatedDelayParameters);

        fn process(filter: &mut ModulatedDelay, xn: f64) -> f64;
    }
}

fn reset(_filter: &mut ffi::ModulatedDelay, _sample_rate: f64) {}

fn set_parameters(_filter: &mut ffi::ModulatedDelay, _params: ffi::ModulatedDelayParameters) {}

fn process(_filter: &mut ffi::ModulatedDelay, _xn: f64) -> f64 {
    0.0
}
