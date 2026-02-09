/// A cutting sequence for an arbitraty slope, `d`.
pub struct CuttingSequence {
    y: f64,
    d: f64,
}

impl CuttingSequence {
    pub fn new(d: f64) -> Self {
        Self { y: d, d }
    }
}

impl Iterator for CuttingSequence {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        let y_prime = self.y + self.d;
        let s = (y_prime.floor() - self.y.floor() - self.d.floor()) > 0f64;
        self.y = y_prime;
        Some(s)
    }
}
