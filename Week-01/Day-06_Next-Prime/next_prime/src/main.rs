fn main() {
    let test_cases = [12, 24, 11];

    for (test_nr, test_case) in test_cases.iter().enumerate() {
        println!("Test #{}, next prime of {} is {}", test_nr, test_case, next_prime(*test_case));
    }
}

fn next_prime(_num: i32) -> i32 {
    0
}