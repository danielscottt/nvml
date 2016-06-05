pub struct PmemObj<T> {
    size: usize,
    d: T,
}

impl<T> PmemObj<T> {
    pub fn new(d: T, size: usize) -> Self {
        PmemObj { size: size, d: d }
    }

    pub fn direct(&self) -> &T {
        &self.d
    }

    pub fn direct_size(&self) -> usize {
        self.size
    }
}
