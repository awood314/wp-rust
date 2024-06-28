use std::ops::Div;

use num_traits::{Float, ToPrimitive};

use crate::util::linear_interpolation;

#[derive(Default)]
pub struct CircularBuffer<T: Float> {
    buffer: Vec<T>,
    write_index: usize,
    length: usize,
    mask: usize,
}

impl<T: Float> CircularBuffer<T> {
    pub fn create(&mut self, length: usize) {
        self.write_index = 0;

        // nearest power of 2
        self.length = 2_usize.pow((length as f64).log10().div(2.0.log10()).ceil() as u32);

        self.mask = self.length.wrapping_sub(1);
        self.buffer.resize(self.length, T::from(0.0).unwrap());
    }

    pub fn read_buffer(&self, delay: usize) -> T {
        let read_index = (self.write_index.wrapping_sub(delay)) & self.mask;
        self.buffer[read_index]
    }

    pub fn read_buffer_fractional(&self, delay: f64) -> T {
        let truncated_delay = delay as usize;
        let fraction = delay - truncated_delay as f64;
        linear_interpolation(
            self.read_buffer(truncated_delay),
            self.read_buffer(truncated_delay + 1),
            T::from(fraction).unwrap(),
        )
    }

    pub fn write_buffer(&mut self, input: T) {
        self.buffer[self.write_index] = input;
        self.write_index = (self.write_index.wrapping_add(1)) & self.mask;
    }
}
