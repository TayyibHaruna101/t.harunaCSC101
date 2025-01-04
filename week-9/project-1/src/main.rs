use std::io::Write; 
use std::fs::File; 

fn main() {
    
    let lager = vec!["33 Export", "Desperados", "Goldberg", "Heineken", "Star"];
    let stout = vec!["Legend", "Tubo King", "Williams"];
    let non_alcoholic = vec!["Maltina", "Amstel Malta", "Malta Gold", "Fayrouz"];

let mut file = File::create("drinks.txt").expect("Failed to create file");

    file.write_all(b"\nStout:\n").expect("Failed to write to file");
    for drink in stout {
        file.write_all(format!("{}\n", drink).as_bytes()).expect("Failed to write to file");
}
     file.write_all(b"Lager:\n").expect("Failed to write to file");
    for drink in lager {
        file.write_all(format!("{}\n", drink).as_bytes()).expect("Failed to write to file");
         }

    file.write_all(b"\nNon-Alcoholic:\n").expect("Failed to write to file");
    for drink in non_alcoholic {
        file.write_all(format!("{}\n", drink).as_bytes()).expect("Failed to write to file");
    }
    println!("Drinks have been saved to drinks.txt");
}
