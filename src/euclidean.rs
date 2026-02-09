use crate::continued_fraction::RationalContinuedFraction;

pub fn euclidean_rhythm(n: u64, d: u64) -> Vec<bool> {
    assert!(n < d);
    let c: Vec<u64> = RationalContinuedFraction::new(n, d).collect();
    // Skip 1 because it's the integer part which is always zero
    c.into_iter()
        .skip(1)
        .rev()
        .fold(vec![false], |v, m| sigma(m as usize, v))
}

fn sigma(m: usize, v: Vec<bool>) -> Vec<bool> {
    v.into_iter()
        .flat_map(|b| {
            let mut result = vec![false; m - 1];
            result.push(true);
            if b {
                result.push(false);
            }
            result
        })
        .collect()
}
