use crate::bjorklund::bjorklund;
use crate::cutting::CuttingSequence;
use crate::euclidean::euclidean_rhythm;

pub mod cutting;
pub mod euclidean;
mod continued_fraction;
pub mod bjorklund;

/// Return true if `b` is a rotation of `a`.
fn is_equivalent(a: &[bool], b: &[bool]) -> bool {
    if a.len() != b.len() { return false; }
    let mut b = b.to_vec();
    for i in 0..a.len() {
        if a == b { return true; }
        (&mut b).rotate_left(1);
    }
    false
}

#[test]
fn test_equivalence() {
    let n = 17;
    let d = 53;

    let a = euclidean_rhythm(n, d);
    let b = CuttingSequence::new(n as f64 / d as f64).take(d as usize).collect::<Vec<bool>>();
    let c = bjorklund(n, d);
    
    assert!(is_equivalent(&a, &b));
    assert!(is_equivalent(&a, &c));
}