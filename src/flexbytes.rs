pub struct FlexBytes<const MAX: usize> {
    bytes: [u8; MAX],
    len: usize,
}

impl<const MAX: usize> FlexBytes<MAX> {
    pub fn new(len: usize) -> Self {
        FlexBytes {
            bytes: [0; MAX],
            len,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn bytes(&self) -> &[u8] {
        &self.bytes[0..self.len]
    }

    pub fn bytes_mut(&mut self) -> &mut [u8] {
        &mut self.bytes[0..self.len]
    }
}
