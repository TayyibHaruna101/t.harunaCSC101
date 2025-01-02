use std::io;

fn main() 
{
    let p:f32 = 3200.0;
    let f:f32 = 3000.0;
    let a:f32 = 2500.0;
    let e:f32 = 2000.0;
    let w:f32 = 2500.0;

 println!("\nYour food menu and their prices is as follows: ");
 println!("\np = Pounded Yam/Edinkaiko soup: {} ",p);   
 println!("f = Fried rice and chicken: {} ",f);
 println!("a = Amala & Ewedu soup: {} ",a);
 println!("e = Eba & Egusi soup: {} ",e);
 println!("w = White Rice & Stew: {} ",w);

println!("Enter the code of the food you want (p, f, a, e, w,)");

let mut food_code = String::new();

io::stdin().read_line(&mut food_code).expect("Failed to read input");
let food_code = food_code.trim();

if food_code == "p" {
    println!("You selected Pounded Yam/Edinkaiko soup and your price is: {} ",p);
}
else if food_code == "f"{
   println!("You selected Fried ried rice and chicken and your price is: {} ",f);
}
else if food_code == "a"{
    println!("You selected Amala & Ewedu soup and your price is: {} ",a);
}
else if food_code == "e"{
    println!("You selected Eba & Egusi soup and your price is: {} ",e);
}
else if food_code == "w"{
    println!("You selected White Rice & Stew and your price is: {} ",w);
}
else {
        println!("Invalid selection. Please choose a valid option.");
    }


}

