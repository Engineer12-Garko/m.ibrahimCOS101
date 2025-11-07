//use standard input/output
use std::io;

fn main() {
    println!("---------------------------------------------------------");
    println!("-------------- Welcome to Mahfuz Restaurent -------------");

    // display the menu
    println!("--- Code ---- | ---------- Item --------- | --- Price ---");
    println!("    P         | Poundo Yam/Edinkaiko Soup |     N3,200   ");
    println!("    F         |    Fried Rice & Chicken   |     N3,000   ");
    println!("    A         |    Amala & Ewedu Soup     |     N2,500   ");
    println!("    E         |     Eba & Egusi Soup      |     N2,000   ");
    println!("    W         |     White Rice & Stew     |     N2,500   ");

    // taking customer name
    println!("\nWhat do you want to buy? Enter its code:");
    let mut code = String::new();
    io::stdin()
        .read_line(&mut code)
        .expect("invalid input code");
    let code = code.trim().to_uppercase();

    //taking the quantity
    println!("How many portions do you want to buy?");
    let mut quantity = String::new();
    io::stdin().read_line(&mut quantity).expect("invalid input");
    let quantity: u32 = quantity.trim().parse().expect("not a valid quantity");

    //using match for item/code determination
    let (item, price): (&str, u32) = match code.as_str() {
        "P" => ("Poundo Yam/Edinkaiko Soup", 3200),
        "F" => ("Fried Rice & Chicken", 3000),
        "A" => ("Amala & Ewedu Soup", 2500),
        "E" => ("Eba & Egusi Soup", 2000),
        "W" => ("White Rice & Stew", 2500),
        _ => {
            println!("Invalid code");
            return;
        }
    };
    
    // calculate the bill
    let bill = price * quantity;

    //the discount logic
    let (discount, total_price): (f32, f32) = if bill > 10000 {
        let discount = bill as f32 * 0.05;
        let total_price = bill as f32 - discount;
        println!("\nCogratulations! You got 5% discount of {:.2}", discount);
        (discount, total_price)
    } else {
        (0.00, bill as f32)
    };

    //order details
    println!();
    println!("---------------------------------------------------------");
    println!("--------------------- Order details ---------------------");
    println!(
        "Item: {} \nPrice: N{}\nQuantity: {} Portions\nDiscount: N{:.2}\nTotal: N{:.2}",
        item, price, quantity, discount, total_price
    );

    println!("------------------------ Enjoy It! ----------------------");
    println!("---------------------------------------------------------");
}
