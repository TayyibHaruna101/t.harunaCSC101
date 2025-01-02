use std::io;

fn main() {
    println!("Enter your years_of_service: ");
    let mut years_of_service = String::new();
    io::stdin().read_line(&mut years_of_service).expect("Failed to read input");
    let years_of_service:f32 = years_of_service.trim().parse().expect("Failed to input");

    if years_of_service >= 40.0 {
        println!("Your incentive for the month is 1_560_000.0");
    }

    if years_of_service > 30.0 {
        println!("Your incentive for the month is 1_480_000.0");
    }

    if years_of_service < 28.0 {
        println!("Your incentive for the month is 1_300_000.0");
    }

    if years_of_service == 0.0 {
        println!("Your incentive for the month is just 100_000.0");
    }
}
