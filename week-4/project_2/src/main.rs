// calculatuing incentives 
    
use std::io; 

fn main(){


let mut input1 = String::new();
let mut input2 = String::new(); 
// take experience 
println!("experience or not? : (true or false)"); 
io::stdin().read_line(&mut input1).expect("Invalid input"); 
let experience : bool = input1.trim().parse().expect("put true or false");

// take age 
println!("Enter your age: ");
io::stdin().read_line(&mut input2).expect("Invalid input"); 
let age:i32 = input2.trim().parse().expect("invalid input"); 

if age >= 40 && experience == true {
    println!("Your incentive is: 1,560,000.0");
} else if age >= 30 && age < 40 && experience == true {
    println!("Your incentive is: 1,480,000.0"); 
} else if experience == true && age < 28 {
    println!("Your incentive is: 1,300,000.0");
}else if experience == false && age > 0 {
    println!("Your incentive is: 100,000.0"); 
}else {println!("out of scope!");}


}

