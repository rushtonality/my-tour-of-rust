/// Return true if the number is prime 
///
/// # Arguments
/// 
/// * `number` - The number to check
pub fn is_prime(number : i64) -> bool {
    let mut candidate : i64 = 2;
    let mut limit = number / 2;

    while candidate <= limit {
        if number % candidate == 0 {
            return false;
        }
        limit = number / candidate;
        candidate += 1;
    }
    true
}

/// Return the largest factor for the number that is also prime.
///
/// # Arguments
/// 
/// * `number` - The number to check
pub fn largest_prime_factor(number : i64) -> i64 {
    let mut prime = 1;
    for factor in 2..number {
        if number % factor == 0 && is_prime(number / factor) {
            prime = number / factor;
            break;
        }
    }
    prime
}

fn main() {
    let number = 13195;
    // Largest prime factor of 13195: 29
    println!("Largest prime factor of {}: {}", number, largest_prime_factor(number));

    let number : i64 = 60085143;
    // Largest prime factor of 60085143: 13597
    println!("Largest prime factor of {}: {}", number, largest_prime_factor(number));

    let number : i64 = 600851475143;
    // Largest prime factor of 600851475143: 6857
    println!("Largest prime factor of {}: {}", number, largest_prime_factor(number));
}
