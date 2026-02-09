/// A translation of Björklunds recursive algorithm from "The Theory of Rep-Rate Pattern Generation in the SNS Timing System"
pub fn bjorklund(n: u64, d: u64) -> Vec<bool> {
    let mut counts = vec![];
    let mut remainders = vec![];
    let mut divisor = d - n;
    remainders.push(n);
    let mut level = 0;
    while remainders[level] > 1 {
        counts.push(divisor / remainders[level]);
        remainders.push(divisor % remainders[level]);
        divisor = remainders[level];
        level += 1;
    }
    counts.push(divisor);

    let mut pattern = vec![];
    build(level as i32, &mut pattern, &counts, &remainders);
    let i = pattern.iter().position(|&x| x).unwrap();
    pattern.rotate_left(i);
    pattern
}

fn build(level: i32, pattern: &mut Vec<bool>, counts: &[u64], remainders: &[u64]) {
    match level {
        -1 => pattern.push(false),
        -2 => pattern.push(true),
        _ => {
            for _ in 0..counts[level as usize] {
                build(level - 1, pattern, counts, remainders);
            }
            if remainders[level as usize] != 0 {
                build(level - 2, pattern, counts, remainders)
            }
        }
    }
}

#[test]
fn test_bjorklund() {
    bjorklund(5, 13);
}
