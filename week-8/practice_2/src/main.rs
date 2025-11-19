use std::io;
fn main() {
    let my_alphabet = vec!['M', 'A', 'H', 'F', 'U', 'z'];
    let mut input = String::new();
    println!("Enter an index valu between 0 - 7");
    io::stdin().read_line(&mut input).expect("not a valid input");
    //index is the non negative value is smaller than the size of the vector
    let index:usize = input.trim().parse().expect("invalid input");

    //getting value at index value
    let ch:char = my_alphabet[index];

    println!("{} is the character for index [{}]\n ",ch, index );

}
