use std::io;

fn main() 
{
    println!("Enter the coefficient a: ");
    let mut a = String::new();
    io::stdin().read_line(&mut a).expect("Failed to read input");
    let a:f32 = a.trim().parse().expect("Failed to input"); 

    println!("Enter the coefficient b: ");
    let mut b = String::new();
    io::stdin().read_line(&mut b).expect("Failed to read input");
    let b:f32 = b.trim().parse().expect("Failed to input"); 

    println!("Enter the coefficient c: ");
    let mut c = String::new();
    io::stdin().read_line(&mut c).expect("Failed to read input");
    let c:f32 = c.trim().parse().expect("Failed to input"); 

    let discriminant:f32 = (b * b) - (4.0 * a * c) / (2.0 * a);
    println!("Descriminant is equals to: {}", discriminant);

    if discriminant < 0.0 {
        println!("There are no real roots");
    }
    if discriminant > 0.0 {
        println!("There are real roots");
    }
    if discriminant == 0.0 {
        println!("There is one real root");
    }
}




