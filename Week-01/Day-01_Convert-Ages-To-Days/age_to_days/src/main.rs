use std::io;

fn main() {
    loop {
        println!("Please enter the amount of years to convert to days");
        let mut buffer = String::new();
        let stdin = io::stdin();
        match stdin.read_line(&mut buffer) {
            Ok(input) => input,
            Err(_) => {
                println!("Error reading input, try again!");
                continue;
            }
        };
        let years: u32 = match buffer.trim().parse::<u32>() {
            Ok(num) => num,
            Err(_) => {
                println!("Not a valid number");
                continue;
            }
        };
        println!("{} years -> {} days", years, calc_age(years));
    }
}

fn calc_age(age:u32) -> u32 {
    let days_per_year = 365;
    age*days_per_year
}