use std::fs::File;
use std::io::{self, Write};

struct MinisterRecords {
    name: String,
    ministry: String,
    geopolitical_zone: String,
}

fn main() -> io::Result<()> {
    let names = vec![
        "Aigbogun Alamba Daudu".to_string(),
        "Murtala Afeez Bendu".to_string(),
        "Okorocha Calistus Ogbona".to_string(),
        "Adewale Jimoh Akanbi".to_string(),
        "Osazuwa Faith Etieye".to_string(),
    ];
    let ministries = vec![
        "Internal Affairs".to_string(),
        "Justice".to_string(),
        "Defense".to_string(),
        "Power & Steel".to_string(),
        "Petroleum".to_string(),
    ];
    let zones = vec![
        "South West".to_string(),
        "North East".to_string(),
        "South East".to_string(),
        "South West".to_string(),
        "South South".to_string(),
    ];

    let mut records = Vec::new();
    for i in 0..names.len() {
        records.push(MinisterRecords { // Changed `MinisterRecord` to `MinisterRecords`
            name: names[i].clone(),
            ministry: ministries[i].clone(),
            geopolitical_zone: zones[i].clone(),
        });
    }

    let mut file = File::create("merged_dataset.txt")?;
    writeln!(file, "{:<3} {:<30} {:<20} {:<15}", "S/N", "Name", "Ministry", "Zone")?;
    writeln!(file, "{}", "-".repeat(70))?;
    for (i, record) in records.iter().enumerate() {
        writeln!(
            file,
            "{:<3} {:<30} {:<20} {:<15}",
            i + 1,
            record.name,
            record.ministry,
            record.geopolitical_zone
        )?;
    }
    println!("The merged dataset has been saved to 'merged_dataset.txt'");
    Ok(())
}
