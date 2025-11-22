use std::io; //using standard input/out

fn main() {
    println!("\nMost experienced programmer checker"); //program description 

    let mut list_of_names = Vec::new(); //create a vector to store the names 
    let mut list_of_years = Vec::new(); //create a vector to store the years

    loop {
        // initiating loop for multiple inputs

        println!();
        println!("What is your name?"); // get the name of the programmer 
        let mut name = String::new();
        io::stdin().read_line(&mut name).expect("not a valid input");
        let name = name.trim();
        if name.to_lowercase() != "stop" {
            println!("\nWelcome {}", name);
        }

        if name.to_lowercase() == "stop" {
            // breaking the loop
            break;
        }

        println!("{}, How many years of experince?", name); // get the years of the programmer's experience  
        let mut years = String::new();
        io::stdin()
            .read_line(&mut years)
            .expect("not a valid input");
        let years: u32 = years.trim().parse().expect("Not a valid number");
        list_of_names.push(name.to_string()); //adding the names to vector   
        list_of_years.push(years); //adding the years to vector

        println!(".......Type 'stop' to ends the list!"); // asking user to end the program
    }

    let mut highest_experince = 0;
    let mut year_index = 0;

    for (i, &years) in list_of_years.iter().enumerate() {
        if years > highest_experince {
            highest_experince = years;
            year_index = i;
        }
    }

    println!(
        "\n{} is the programmer with highest yeasrs of experince : {}",
        list_of_names[year_index], highest_experince
    );

    println!("\nList of the Total programmers:");
    for (i, list_of_name) in list_of_names.iter().enumerate() {
        println!(
            "{}. {} with {}years of experince",
            i + 1,
            list_of_name,
            list_of_years[i]
        );
    }
}
