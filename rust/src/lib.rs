use std::f64::consts::PI;

#[cxx::bridge(namespace = "wprust::rust")]
mod ffi {
    struct BiquadCoefficients {
        a0: f64,
        a1: f64,
        a2: f64,
        b1: f64,
        b2: f64,

        // wet/dry
        c0: f64,
        d0: f64,
    }

    struct Biquad {
        // state
        x_z1: f64,
        x_z2: f64,
        y_z1: f64,
        y_z2: f64,

        // coefficients
        coeffs: BiquadCoefficients,
    }

    struct AudioFilterParameters {
        frequency: f64,
    }

    struct AudioFilter {
        params: AudioFilterParameters,
        biquad: Biquad,
        sample_rate: f64,
    }

    extern "Rust" {
        fn reset(filter: &mut AudioFilter, sample_rate: f64);

        fn set_parameters(filter: &mut AudioFilter, params: AudioFilterParameters);

        fn process(filter: &mut AudioFilter, xn: f64) -> f64;
    }
}

impl ffi::Biquad {
    fn reset(&mut self) {
        self.x_z1 = 0.0;
        self.x_z2 = 0.0;
        self.y_z1 = 0.0;
        self.y_z2 = 0.0;
    }

    fn process(&mut self, xn: f64) -> f64 {
        // Direct algo

        // compute output sample
        let yn = {
            let yn = self.coeffs.a0 * xn + self.coeffs.a1 * self.x_z1 + self.coeffs.a2 * self.x_z2
                - self.coeffs.b1 * self.y_z1
                - self.coeffs.b2 * self.y_z2;

            // underflow check
            if yn.is_normal() {
                yn
            } else {
                0.0
            }
        };

        // update states
        self.x_z2 = self.x_z1;
        self.x_z1 = xn;
        self.y_z2 = self.y_z1;
        self.y_z1 = yn;

        yn
    }
}

fn set_parameters(filter: &mut ffi::AudioFilter, params: ffi::AudioFilterParameters) {
    // pass-through
    filter.biquad.coeffs.c0 = 1.0;
    filter.biquad.coeffs.d0 = 0.0;

    // set parameters
    filter.params = params;

    // update coeffs
    let theta_c = 2.0 * PI * filter.params.frequency / filter.sample_rate;
    let gamma = theta_c.cos() / (1.0 + theta_c.sin());

    // LPF1
    filter.biquad.coeffs.a0 = (1.0 - gamma) / 2.0;
    filter.biquad.coeffs.a1 = (1.0 - gamma) / 2.0;
    filter.biquad.coeffs.a2 = 0.0;
    filter.biquad.coeffs.b1 = -gamma;
    filter.biquad.coeffs.b2 = 0.0;
}

fn reset(filter: &mut ffi::AudioFilter, sample_rate: f64) {
    filter.sample_rate = sample_rate;
    filter.biquad.reset()
}

fn process(filter: &mut ffi::AudioFilter, xn: f64) -> f64 {
    filter.biquad.coeffs.d0 * xn + filter.biquad.coeffs.c0 * filter.biquad.process(xn)
}
