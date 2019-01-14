fn is_prime(number : i64) -> bool {
    for candidate in 2..=(number/2) {
        if number % candidate == 0 {
            return false;
        }
    }
    true
}

fn largest_prime_factor(number : i64) -> i64 {
    let mut prime = 1;
    for factor in 1..number {
        if number % factor == 0 && is_prime(factor){
            prime = factor
        }
    }
    prime
}

fn main() {
    let number = 13195;
    println!("Largest prime factor of {}: {}", number, largest_prime_factor(number));

    let number = 600851475143;
    println!("Largest prime factor of {}: {}", number, largest_prime_factor(number));
}
