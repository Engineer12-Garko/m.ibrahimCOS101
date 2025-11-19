fn main() {
    //name vector 
    let name = vec!["Maryam", "Sam", "Sally", "Greg", "Ade", "Mark", "June", "Ife"];

    // age vector
    let age = vec![16,17,19,22,20,21,18,23];

    println!("\nAge allocation\n"); 
    //loop to iterate elements in vector
    for i in 0..age.len() {
        //iterating through i on vector
        println!("{} is {} years old",name[i],age[i]);
    }
}
