use std::io::Write; // Import the trait needed to write to files
use std::fs::File;  // Import the File struct

fn main() {
    //  Define the data in Vectors
    let student_names = vec![
        "Oluchi Mordi",
        "Adams Aliyu",
        "Shania Bolade",
        "Adekunle Gold",
        "Blanca Edemoh",
    ];

    let matric_numbers = vec![
        "ACC10211111",
        "ECO10110101",
        "CSC10328828",
        "EEE11020202",
        "MEE10202001",
    ];

    let departments = vec![
        "Accounting",
        "Economics",
        "Computer",
        "Electrical",
        "Mechanical",
    ];

    let levels = vec![300, 100, 200, 200, 100];

    
    // "data.txt" is the name of the file that will be created in your project folder
    let mut file = File::create("data.txt").expect("Could not create file");

    // 3. Define the Headers and Format
    // We print to console AND write to the file
    let header_main = "PAU SMIS";
    let header_cols = format!(
        "{:<20} {:<15} {:<15} {:<10}",
        "Student Name", "Matric. Number", "Department", "Level"
    );
    
    // Print to Console
    println!("{}", header_main);
    println!("{}", header_cols);
    println!("-------------------------------------------------------------");

    // Write to File
    // We use writeln! instead of println! for files
    writeln!(file, "{}", header_main).expect("Could not write to file");
    writeln!(file, "{}", header_cols).expect("Could not write to file");
    writeln!(file, "-------------------------------------------------------------").expect("Could not write to file");

    // Iterate and Save
    for i in 0..student_names.len() {
        // Construct the formatted row string
        let row = format!(
            "{:<20} {:<15} {:<15} {:<10}",
            student_names[i], matric_numbers[i], departments[i], levels[i]
        );

        // Print to Console
        println!("{}", row);

        // Write to File
        writeln!(file, "{}", row).expect("Could not write to file");
    }

    println!("\nData successfully saved to 'data.txt'.");
}