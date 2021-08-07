use std::borrow::Borrow;

pub struct FlexBytes<const MAX: usize> {
    bytes: [u8; MAX],
    len: usize,
}

impl<const MAX: usize> FlexBytes<MAX> {
    pub fn new(len: usize) -> Self {
        assert!(len <= MAX);

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

impl<'a, T: Borrow<[u8]>, const MAX: usize> From<T> for FlexBytes<MAX> {
    fn from(slice: T) -> Self {
        let mut bytes = FlexBytes::new(slice.borrow().len());
        bytes.bytes_mut().copy_from_slice(slice.borrow());
        bytes
    }
}
