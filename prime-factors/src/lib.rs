pub fn factors(n: u64) -> Vec<u64> {
    let mut factors = vec![];
    for x in 2..n + 1 {
        if n % x == 0 {
            factors.push(x);
            if (n / x != 1) & (x != n) {
                factors.push(n / x);
            }
        }
    }

    println!("Factors: {:?}", factors);
    factors
}
