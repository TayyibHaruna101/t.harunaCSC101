use std::fs::File;
use std::io::{self, Write};

struct Student {
    name: String,
    matric_number: String,
    department: String,
    level: u32,
}

fn main() -> io::Result<()> {
    let students = vec![
        Student {
            name: "Oluchi Mordi".to_string(),
            matric_number: "ACC1021111".to_string(),
            department: "Accounting".to_string(),
            level: 100,
        },
        Student {
            name: "Adams Aliyu".to_string(),
            matric_number: "ECO1011011".to_string(),
            department: "Economics".to_string(),
            level: 100,
        },
        Student {
            name: "Shania Bolade".to_string(),
            matric_number: "CSC10328828".to_string(),
            department: "Computer".to_string(),
            level: 100,
        },
        Student {
            name: "Adekunle Gold".to_string(),
            matric_number: "EEE20210101".to_string(),
            department: "Electrical".to_string(),
            level: 200,
        },
        Student {
            name: "Blanca Edemoh".to_string(),
            matric_number: "MEE10200102".to_string(),
            department: "Mechanical".to_string(),
            level: 100,
        },
    ];

    // Print to console
    println!("{:<20} {:<15} {:<15} {:<5}", "Student Name", "Matric Number", "Department", "Level");
    println!("{}", "-".repeat(60));
    for student in &students {
        println!(
            "{:<20} {:<15} {:<15} {:<5}",
            student.name, student.matric_number, student.department, student.level
        );
    }

    // Write to file
    let mut file = File::create("student_details.txt")?;
    writeln!(file, "{:<20} {:<15} {:<15} {:<5}", "Student Name", "Matric Number", "Department", "Level")?;
    writeln!(file, "{}", "-".repeat(60))?;
    for student in &students {
        writeln!(
            file,
            "{:<20} {:<15} {:<15} {:<5}",
            student.name, student.matric_number, student.department, student.level
        )?;
    }

    println!("Student details have been saved to 'student_details.txt'.");
    Ok(())
}
