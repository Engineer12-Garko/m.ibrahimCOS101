fn main() {
    //create an empty vector "city"
    let mut city:Vec<String> = Vec::new(); 

    //print city vector 
    println!("The city Vector has element {}", city.len()); 
    //push new element into vector 
    let mut input1 = String::new();
    println!("How many cities do want enter?");
    std::io::stdin().read_line(&mut input1).expect("Failed to read input");
  

    let city_num : i32 = input1.trim().parse().expect("invalid input");
    for count in 0..city_num {
        let mut input2 = String::new();
        println!("Enter your the city:");
        std::io::stdin().read_line(&mut input2).expect("not a valid input");
        let new_city:String = input2.trim().parse().expect("invalid input");
        city.push(new_city); 
    }
println!("Your preferred cities are\n"); 
let mut count = 1; 
for i in city {
    //iterating through elements i on vector
    println!("{} {}", count, i);
    count += 1;
}


}