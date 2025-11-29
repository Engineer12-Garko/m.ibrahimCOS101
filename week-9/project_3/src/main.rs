use std::io::Write; 
use std::fs::File;  

fn main() {
    // Define the Data
    let names = vec![
        "Aigbogun Alamba Daudu",
        "Murtala Afeez Bendu",
        "Okorocha Calistus Ogbona",
        "Adewale Jimoh Akanbi",
        "Osazuwa Faith Etieye"
    ];

    let ministries = vec![
        "Internal Affairs",
        "Justice",
        "Defense",
        "Power & Steel",
        "Petroleum"
    ];

    let zones = vec![
        "South West",
        "North East",
        "South South",
        "South West",
        "South East"
    ];

    //  Create the CSV File
    let mut file = File::create("EFCC_Ministers.csv").expect("Could not create file");

    // 3. Handle Headers
    // A. Print pretty header to Console
    println!("EFCC CONVICTED MINISTERS DATA MERGE");
    println!("========================================================================");
    println!("{:<4} | {:<25} | {:<20} | {:<15}", "S/N", "NAME OF COMMISSIONER", "MINISTRY", "GEOPOLITICAL ZONE");
    println!("------------------------------------------------------------------------");

    // Write simple header to CSV File
    writeln!(file, "S/N,NAME OF COMMISSIONER,MINISTRY,GEOPOLITICAL ZONE").expect("Could not write header to file");

    //The Loop 
    for i in 0..names.len() {
        println!(
            "{:<4} | {:<25} | {:<20} | {:<15}",
            i + 1,
            names[i],
            ministries[i],
            zones[i]
        );

        writeln!(
            file,
            "{},{},{},{}",
            i + 1,
            names[i],
            ministries[i],
            zones[i]
        ).expect("Could not write row to file");
    }

    println!("========================================================================");
    println!("\nSuccess! Data saved to 'EFCC_Ministers.csv'");
}