use crate::circ_buffer::CircularBuffer;

#[derive(Default)]
pub struct Delay {
    pub delay: f64,
    pub feedback: f64,
    buffer: CircularBuffer<f64>,
    wet: f64,
    dry: f64,
    sample_rate: f64,
    buffer_length_ms: f64,
    buffer_length: usize,
}

impl Delay {
    pub fn reset(&mut self, sample_rate: f64) {
        self.create_delay_buffer(sample_rate, self.buffer_length_ms);
    }

    pub fn create_delay_buffer(&mut self, sample_rate: f64, length_ms: f64) {
        self.buffer_length_ms = length_ms;
        self.sample_rate = sample_rate;
        self.buffer_length = (self.buffer_length_ms * sample_rate / 1000.0) as usize + 1;
        self.buffer.create(self.buffer_length);
    }

    pub fn process(&mut self, xn: f64) -> f64 {
        let yn = self.buffer.read_buffer(self.delay as usize);
        let dn = xn + (self.feedback / 100.0) + yn;

        self.buffer.write_buffer(dn);

        self.dry * xn + self.wet * yn
    }
}
