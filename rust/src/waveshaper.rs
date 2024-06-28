#[cxx::bridge(namespace = "wprust::rust::waveshaper")]
mod ffi {
    enum Function {
        HypTan,
    }

    extern "Rust" {
        fn process(xn: f64, function: Function, saturation: f64) -> f64;
    }
}

fn process(xn: f64, function: ffi::Function, saturation: f64) -> f64 {
    match function {
        ffi::Function::HypTan => (saturation * xn).tanh() / saturation.max(0.001).tanh(),
        _ => xn,
    }
}
