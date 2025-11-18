fn main() {
    let test_cases = [1, 2, 12, 24, 11, 123456];

    for (test_nr, test_case) in test_cases.iter().enumerate() {
        println!(
            "Test #{}, next prime of {} is {}",
            test_nr,
            test_case,
            next_prime(*test_case)
        );
    }

    println!("Let's look for the first 100 primes");
    let mut found_primes = 0;
    let mut n = 1;
    while found_primes < 100 {
        let prime = next_prime(n);
        println!("Prime #{} is {}", found_primes + 1, prime);
        n = prime + 1;
        found_primes += 1;
    }
}

fn next_prime(num: u32) -> u32 {
    for n in num..u32::MAX {
        // 2 is even and prime
        if n == 2 {
            return n;
        }
        // other even numbers are not primes
        if n > 2 && n % 2 == 0 {
            continue;
        }
        let mut is_prime = false;
        for factor in 3..n + 1 {
            // skip even factors
            if factor % 2 == 0 {
                continue;
            }
            if n % factor == 0 && factor < num {
                // Definitely not a prime
                break;
            }
            if n % factor == 0 {
                is_prime = true;
            }
        }
        if is_prime {
            return n;
        };
    }
    0
}
