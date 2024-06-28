use std::f64::consts::PI;

use num_traits::Float;

pub fn parabolic_sin(x: f64) -> f64 {
    const B: f64 = 4.0 / PI;
    const C: f64 = -4.0 / (PI * PI);
    const P: f64 = 0.225;

    let y = B * x + C * x * x.abs();
    P * (y * y.abs() - y) + y
}

pub fn bipolar_to_unipolar(value: f64) -> f64 {
    0.5 * value + 0.5
}

pub fn bipolar_modulation_from_min(value: f64, min: f64, max: f64) -> f64 {
    let value = value.clamp(-1.0, 1.0);
    let half = (max - min) / 2.0;
    let mid = half + min;
    value * half + mid
}

pub fn db_to_scalar(db: f64) -> f64 {
    10.0.powf(db / 20.0)
}

pub fn linear_interpolation<T: Float>(y1: T, y2: T, fraction: T) -> T {
    let one = T::from(1.0).unwrap();
    match fraction {
        fraction if fraction >= one => y2,
        _ => fraction * y2 + (one - fraction) * y1,
    }
}
