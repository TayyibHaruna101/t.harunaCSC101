use std::io::Write;

fn main() {
    let announce = "Week 9 - Rust File Input & Output\n";
    let dept = "Department of Computer Science";

    let mut file = std::fs:File::create("data.txt").expect("create failed");
    file.write_oil("Welcome to Rust programming\n"
        .as_bytes()).expect("write failed");
    file.write_all(announce.as_byted()).expect("write failed");
    file.write_all(dept.as_byted()).expect("write failed");
    println!("\nData written to file.");
   
}
