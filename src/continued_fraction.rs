/// The continued fraction of a rational number.
pub struct RationalContinuedFraction {
    n: u64,
    d: u64
}

impl RationalContinuedFraction {
    pub fn new(n: u64, d: u64) -> Self {
        Self { n, d }
    }
}

impl Iterator for RationalContinuedFraction {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.d == 0 { return None; }
        let (q, r) = (self.n.div_euclid(self.d), self.n.rem_euclid(self.d));
        (self.n, self.d) = (self.d, r);
        Some(q)
    }
}
