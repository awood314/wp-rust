#[derive(Default)]
pub struct CircularBuffer<T: Copy + Default> {
    buffer: Vec<T>,
    write_index: usize,
    length: usize,
    mask: usize,
}

impl<T: Copy + Default> CircularBuffer<T> {
    pub fn create(&mut self, length: usize) {
        self.write_index = 0;

        // nearest power of 2
        self.length = length.ilog2().div_ceil(2_u32.ilog2()).pow(2) as usize;

        self.mask = self.length.wrapping_sub(1);
        self.buffer.resize(self.length, Default::default());
    }

    pub fn read_buffer(&self, delay: usize) -> T {
        let read_index = (self.write_index.wrapping_sub(delay)) & self.mask;
        self.buffer[read_index]
    }

    // TODO: interpolation
    pub fn read_buffer_fractional(&self, delay: f64) -> T {
        self.read_buffer(delay as usize)
    }

    pub fn write_buffer(&mut self, input: T) {
        self.buffer[self.write_index] = input;
        self.write_index = (self.write_index.wrapping_add(1)) & self.mask;
    }
}
