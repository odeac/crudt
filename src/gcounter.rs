#[derive(Clone, Debug)]
pub struct GCounter {
    value: u64,
}

impl GCounter {
    pub fn new() -> Self {
        Self { value: 0 }
    }

    pub fn value(&self) -> u64 {
        self.value
    }

    pub fn inc(&mut self, v: u64) {
        self.value += v
    }

    // pub fn merge(&mut self, other : &GCounter ) {

    // }
}

impl Default for GCounter {
    fn default() -> Self {
        Self::new()
    }
}
