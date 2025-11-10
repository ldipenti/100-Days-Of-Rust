fn main() {
    let test_cases:[&[u8]; 4] = [
        &[3, 4, 1, 2],
        &[10, 11, 12, 9, 10],
        &[6, 5, 4, 3, 2, 9],
        &[9, 9],
    ];
    for (run_nr, runs) in test_cases.iter().enumerate() {
        println!("Run #{} has {} days of progress", run_nr, progress_days(runs));
    }
}

fn progress_days(runs: &[u8]) -> usize {
    let mut counter = 0;
    for days_pair in runs.windows(2) {
        let prev = days_pair[0];
        let next = days_pair[1];
        if next > prev { counter += 1};
    }
    return counter;
}