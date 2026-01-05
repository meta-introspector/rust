fn main() {
    let mut primes = Vec::new();
    let mut n = 2;
    
    while primes.len() < 10 {
        let mut is_prime = true;
        for &p in &primes {
            if p * p > n {
                break;
            }
            if n % p == 0 {
                is_prime = false;
                break;
            }
        }
        if is_prime {
            primes.push(n);
        }
        n += 1;
    }
    
    println!("First 10 primes: {:?}", primes);
}
