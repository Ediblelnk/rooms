// This function returns the exact number of primes less than or equal to x
pub fn pi(x: usize) -> usize {
    if x < 2 {
        return 0;
    }

    let upperbound = (x as f64 / ((x as f64).ln() - 1.0)).ceil() as usize;

    let mut primes = Vec::with_capacity(upperbound);
    primes.push(2);

    for candidate in (3..x).step_by(2) {
        for p in &primes {
            if candidate % p == 0 {
                break;
            } else if p * p > candidate {
                primes.push(candidate);
                break;
            }
        }
    }
    primes.len()
}
