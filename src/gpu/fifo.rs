pub(crate) struct Fifo<T: Copy, const N: usize> {
    pub(crate) contents: Vec<T>,
}

impl<T: Copy, const N: usize> Fifo<T, N> {
    pub(crate) const fn new() -> Self {
        Self {
            contents: Vec::new(),
        }
    }

    pub(crate) fn clear(&mut self) {
        self.contents.clear();
    }

    pub(crate) fn append(&mut self, array: &[T]) {
        for element in array {
            if self.contents.len() < N {
                self.contents.push(*element);
            }
        }
    }
}
