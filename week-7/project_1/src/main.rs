use std::io; // ussing standard input/output 

//creating a function to get input
fn input_value() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("not a valid input");
    input.trim().to_string()
}

//function to calculate tarapezium area
fn area_of_trapezium() -> f32 {
    println!("Enter the height of the Trapezium:");
    let h = input_value();
    let h: f32 = h.trim().parse().expect("invalid input");

    println!("Enter the Base (a) of the Trapezium:");
    let a = input_value();
    let a: f32 = a.trim().parse().expect("invalid input");

    println!("Enter the Base (b) of the Trapezium:");
    let b = input_value();
    let b: f32 = b.trim().parse().expect("invalid input");

    let area: f32 = h / 2.0 * (a + b);
    return area;
}

//function to calculate rhombus area
fn area_of_rhombus() -> f32 {
    println!("Enter the First diagonal of the Rhombus:");
    let d1 = input_value();
    let d1: f32 = d1.trim().parse().expect("invalid input");

    println!("Enter the second diagonal of the Rhombus:");
    let d2 = input_value();
    let d2: f32 = d2.trim().parse().expect("invalid input");

    let area: f32 = 1.0 / 2.0 * (d1 * d2);
    return area; //using return key word
}

//function to calculate parallelogram area
fn area_of_parallelogram() -> f32 {
    println!("Enter the Base:");
    let base = input_value();
    let base: f32 = base.trim().parse().expect("invalid input");

    println!("Enter the Altitude:");
    let altitude = input_value();
    let altitude: f32 = altitude.trim().parse().expect("invalid input");

    base * altitude // area of parallelogram 
}

//function to calculate cube area
fn area_of_cube() -> f32 {
    println!("Enter the length of the cube:");
    let length = input_value();
    let length: f32 = length.trim().parse().expect("invalid input");

    6.0 * length * length // area of the cube 
}

//function to calculate cylinder volume
fn volume_of_cylinder() -> f32 {
    println!("Enter the radius of the Cylinder:");
    let radius = input_value();
    let radius: f32 = radius.trim().parse().expect("invalid input");

    println!("Enter the height of the Cylinder:");
    let height = input_value();
    let height: f32 = height.trim().parse().expect("invalid input");

    let pi: f32 = 22.0 / 7.0;
    let volume: f32 = pi * radius * radius * height;
    return volume; // using return key word 
}

fn main() {
    loop {
        // using loop for multple calculation
        
        println!("\nCalculate area of Trapezium, Rhombus; Parallelogram, Cube or Volume of the Cylinder"
        );
        println!("\nSelect the Shape: Trapezium (T), Rhombus (R); Parallelogram (P), Cube (C) or Cylinder (Cy)"
        );

        let option = input_value().trim().to_lowercase();
        //using if logic to match the selected shape with its function

        if option == "trapezium" || option == "t" {
            let trapezium_area = area_of_trapezium();
            println!("The area of the Trapezium is: {:.2}", trapezium_area);
        } else if option == "rhombus" || option == "r" {
            let rhombus_area = area_of_rhombus();
            println!("The area of the Rhombus is : {:.2}", rhombus_area);
        } else if option == "parallelogram" || option == "p" {
            let parallelogram_area = area_of_parallelogram();
            println!(
                "The area of the parallelogram is : {:.2}",
                parallelogram_area
            );
        } else if option == "cube" || option == "c" {
            let cube_area = area_of_cube();
            println!("The area of the cube is : {:.2}", cube_area);
        } else if option == "cylinder" || option == "cy" {
            let cylinder_volume = volume_of_cylinder();
            println!("The volume of the Cylinder is : {:.2}", cylinder_volume);
        }
        println!("===========================================================");
        println!("\nDo you want to find another area or volume?");
        let cont_calculation = input_value().trim().to_lowercase();
        if cont_calculation == "yes" {
            continue;
        } else {
            println!("Done!");
            break;
        }
    }
}
