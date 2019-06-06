fn is_prime(number: u64) -> bool {
    let mut is_prime = true;
    if number == 1 { return false; }
    for n in 2..number {
        if (number % n == 0) & (number != n) {
            is_prime = false;
            break;
        }
    }
    is_prime
}

pub fn factors(n: u64) -> Vec<u64> {
    let mut prime_factors = vec![];

    for x in 2..n + 1 {
        if n % x == 0 {
            let cuotient = n / x;
            if is_prime(x) { prime_factors.push(x); }
            if is_prime(cuotient) { prime_factors.push(cuotient); }
            if cuotient > 2 {
                factors(cuotient);
            }
        }
    }

    prime_factors
}
