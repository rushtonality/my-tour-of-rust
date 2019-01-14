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

/// Return value in the Factorial Sequence for the given position.
/// 1, 1, 2, 6, 24, 120, 720, ...
///
/// # Arguments
/// 
/// * `number` - The position to evaluate
pub fn factorial_ver1(number : i64) -> i64 {
    if number == 0 {
        return 1;
    }
    return number * factorial_ver1(number-1);
}

/// Return value in the Factorial Sequence for the given position.
/// 1, 1, 2, 6, 24, 120, 720, ...
///
/// # Arguments
/// 
/// * `number` - The position to evaluate
pub fn factorial_ver2(number : i64) -> i64 {
    factorial_ver2_internal(number, 1)
}

fn factorial_ver2_internal(number : i64, acc : i64) -> i64 {
    if number == 0 {
        return acc;
    }
    factorial_ver2_internal(number - 1, acc * number)
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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_factorial_ver1_0() {
        let number = 0;
        let answer = 1;
        assert_eq!(answer, factorial_ver1(number));
    }

    #[test]
    fn test_factorial_ver1_1() {
        let number = 1;
        let answer = 1;
        assert_eq!(answer, factorial_ver1(number));
    }

    #[test]
    fn test_factorial_ver1_3() {
        let number = 2;
        let answer = 2;
        assert_eq!(answer, factorial_ver1(number));
    }

    #[test]
    fn test_factorial_ver1_10() {
        let number = 10;
        let answer = 3628800;
        assert_eq!(answer, factorial_ver1(number));
    }

   #[test]
    fn test_factorial_ver2_0() {
        let number = 0;
        let answer = 1;
        assert_eq!(answer, factorial_ver2(number));
    }

    #[test]
    fn test_factorial_ver2_1() {
        let number = 1;
        let answer = 1;
        assert_eq!(answer, factorial_ver2(number));
    }

    #[test]
    fn test_factorial_ver2_2() {
        let number = 2;
        let answer = 2;
        assert_eq!(answer, factorial_ver2(number));
    }

    #[test]
    fn test_factorial_ver2_10() {
        let number = 10;
        let answer = 3628800;
        assert_eq!(answer, factorial_ver2(number));
    }

    #[test]
    fn test_factorial_ver2_large() {
        let number = i64::max_value();
        let result = std::panic::catch_unwind(|| factorial_ver2(number));
        assert!(result.is_err());
    }

    #[test]
    fn test_prime_1() {
        assert_eq!(true, is_prime(1));
    }
    #[test]
    fn test_prime_2() {
        assert_eq!(true, is_prime(2));
    }
    #[test]
    fn test_prime_3() {
        assert_eq!(true, is_prime(3));
    }
    #[test]
    fn test_prime_4() {
        assert_eq!(false, is_prime(4));
    }
    #[test]
    fn test_prime_5() {
        assert_eq!(true, is_prime(5));
    }
    #[test]
    fn test_prime_large() {
        assert_eq!(true, is_prime(67_280_421_310_721));
    }
    
    #[test]
    fn test_largest_prime_factor_13195() {
        let number = 13195;
        let answer = 29;
        assert_eq!(answer, largest_prime_factor(number));
    }
    #[test]
    fn test_largest_prime_factor_60085143() {
        let number = 60085143;
        let answer = 13597;
        assert_eq!(answer, largest_prime_factor(number));
    }
    #[test]
    fn test_largest_prime_factor_600851475143() {
        let number = 600851475143;
        let answer = 6857;
        assert_eq!(answer, largest_prime_factor(number));
    }
}