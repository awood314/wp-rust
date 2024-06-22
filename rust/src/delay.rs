use crate::circ_buffer::CircularBuffer;

#[derive(Default)]
pub struct Delay {
    pub delay: f64,    // samples
    pub feedback: f64, // 0-1
    pub wet: f64,      // scalar
    pub dry: f64,      // scalar
    sample_rate: f64,
    buffer_length_ms: f64,
    buffer_length: usize,
    buffer: CircularBuffer<f64>,
}

impl Delay {
    pub fn create_delay_buffer(&mut self, sample_rate: f64, length_ms: f64) {
        self.buffer_length_ms = length_ms;
        self.sample_rate = sample_rate;
        self.buffer_length = (self.buffer_length_ms * sample_rate / 1000.0) as usize + 1;
        self.buffer.create(self.buffer_length);
    }

    pub fn process(&mut self, xn: f64) -> f64 {
        let yn = self.buffer.read_buffer_fractional(self.delay);
        let dn = xn + self.feedback * yn;

        self.buffer.write_buffer(dn);

        self.dry * xn + self.wet * yn
    }
}
