use std::io;

//creating input function
fn input() -> String { 
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("faileto accept input");
    input1.trim().to_string()
}

//using sruct for matching name and praces 
struct Co { 
    name: String,
    price: u32,
}

fn main() {
    let mut total: Vec<u32> = Vec::new();

    let com1 = Co {
        name: String::from("hp"),
        price: 650_000,
    };

    let com2 = Co {
        name: String::from("ibm"),
        price: 755_000,
    };

    let com3 = Co {
        name: String::from("toshiba"),
        price: 550_000,
    };

    let com4 = Co {
        name: String::from("dell"),
        price: 850_000,
    };

    //creating loop for multiple purchase
    loop {
        println!("Welcome to Computer Store!");
        println!("What do you want to buy?");
        println!("We have:\nhp, toshiba, ibm, and dell");

        let computer = input().to_lowercase();

        println!("How many do you want to buy?");
        let quantity: u32 = input().parse().expect("Not a valid number");

        if computer == com1.name {
            let amount = com1.price * quantity;
            println!("Name: {:<10} Price: N{:<10} Qty: {}", com1.name, com1.price, quantity);
            println!("Total amount = {}", amount);
            total.push(amount);
        } 
        else if computer == com2.name {
            let amount = com2.price * quantity;
            println!("Name: {:<10} Price: N{:<10} Qty: {}", com2.name, com2.price, quantity);
            println!("Total amount = {}", amount);
            total.push(amount);
        } 
        else if computer == com3.name {
            let amount = com3.price * quantity;
            println!("Name: {:<10} Price: N{:<10} Qty: {}", com3.name, com3.price, quantity);
            println!("Total amount = {}", amount);
            total.push(amount);
        } 
        else if computer == com4.name {
            let amount = com4.price * quantity;
            println!("Name: {:<10} Price: N{:<10} Qty: {}", com4.name, com4.price, quantity);
            println!("Total amount = {}", amount);
            total.push(amount);
        } 
        else {
            println!("Invalid Item name!");
        }
        
        //asking user to stop or continue 
        println!("Is that all? (yes/no)");
        let cont = input().to_lowercase();
        if cont == "yes" {
            break;
        }
    }

    // Print all individual totals
    println!("\nItems Purchased:");
    for t in &total {
        println!("{}", t);
    }

    // Sum everything
    let sum: u32 = total.iter().sum();
    println!("\nYour grand total purchase = N{}", sum);
    println!("See You later!");
}
