use std::io;  // using standard input/output

fn main() {
       loop { // initiating loop through the process

    println!("-------------- Welcome to APS Checker -----------------");
    println!("Office administrator (Admin), Academic, Lawyer, Teacher"); 
    println!("-------------------------------------------------------");
    
    // converting input to lowercase  
    println!("What is your profession?"); 
    let mut profession = String::new(); 
    io::stdin().read_line(&mut profession).expect("invalid input");
    let profession = profession.trim().to_lowercase(); 

    println!("How many years of experience?"); 
    let mut exp = String::new(); 
    io::stdin().read_line(&mut exp).expect("invalid input");
    let exp: u8 = exp.trim().parse().expect("not a valid number");
   
    // Using if-else to determine aps_level 
    let aps_level = if exp >= 1 && exp <= 2 {
        "APS 1-2"
    } else if exp >= 3 && exp <= 4 { 
        "APS 3-5"
    } else if exp >= 5 && exp <= 8 {
        "APS 5-8"
    } else if exp >= 9 && exp <= 10 { 
        "EL1 8-10"
    } else if exp >= 11 && exp <= 13 {
        "EL2 10-13"
    } else {
        "SES"
    }; 

    // Determine job title based on profession and experience
    let job_title = match profession.as_str() {
        "office administrator" | "admin" => match exp {
            1..=2 => "Intern",
            3..=4 => "Administrator", 
            5..=8 => "Senior Administrator",
            9..=10 => "Office Manager",
            11..=13 => "Director",
            _ => "CEO"
        },
        "academic" => match exp {
            1..=2 => "–",
            3..=4 => "Research Assistant", 
            5..=8 => "PhD Candidate",
            9..=10 => "Post-Doc Researcher",
            11..=13 => "Senior Lecturer",
            _ => "Dean"
        },
        "lawyer" => match exp {
            1..=2 => "Paralegal",
            3..=4 => "Junior Associate", 
            5..=8 => "Associate",
            9..=10 => "Senior Associate 1-2",
            11..=13 => "Senior Associate 3-4",
            _ => "Partner"
        },
        "teacher" => match exp {
            1..=2 => "Placement",
            3..=4 => "Classroom Teacher", 
            5..=8 => "Sm Teacher",
            9..=10 => "Leading Teacher",
            11..=13 => "Deputy Principal",
            _ => "Principal"
        },
        _ => {
            println!("Invalid profession!");
            return;
        }
    };

    // Displaying final results
    println!("\n-------------------- Final Result --------------------");
    println!("------------------------------------------------------");
    println!("Profession: {}", profession);
    println!("Experience: {} years", exp);
    println!("APS Level: {}", aps_level);
    println!("Job Title: {}", job_title);
    println!("\n------------------------------------------------------");

    // loop for mutiple checking 
    println!("\nDo want to continue? (yes or no)"); 
    let mut to_continue = String::new(); 
    io::stdin().read_line(&mut to_continue).expect("invalid input");
    let to_continue = to_continue.trim().to_lowercase();

    if to_continue == "yes" {
        continue; 
    } else {
        break;
    }
}

}