use std::io;
use std::fs;

fn main() {
    println!("\nWelcome to Globacom DB Manager");

    // Starting the loop 
    loop {
        println!("\n------------------------------");
        println!("Please select your role by entering a number:");
        println!("1. Administrator");
        println!("2. Project Manager");
        println!("3. Employee");
        println!("4. Customer");
        println!("5. Vendor");
        println!("6. Exit"); 
        println!("------------------------------");
        println!("Enter choice (1-6):");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let choice = input.trim();

        // Checking for exit condition first
        if choice == "6" {
            println!("Exiting program. Goodbye!");
            break; 
        }

        let file_path = match choice {
            "1" => "globacom_db.sql",
            "2" => "project_tb.sql",
            "3" => "staff_tb.sql",
            "4" => "customer_tb.sql",
            "5" => "dataplan_tb.sql",
            _ => {
                println!("Invalid selection. Please try again.");
                continue; 
            }
        };

        // Read and print the file content
        match fs::read_to_string(file_path) {
            Ok(contents) => {
                println!("\nAccess Granted!\n");
                println!("{}", contents);
        
            },
            Err(e) => {
                println!("Error: Could not read file '{}'.", file_path);
                println!("Details: {}", e);
            }
        }
    }
}