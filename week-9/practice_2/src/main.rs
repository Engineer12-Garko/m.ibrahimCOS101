use std::io::Write;
use std::io::Read; 

fn main() {
let mut file = std::fs::File::open("Mahfuz.txt").expect("Failed to read file");
let mut contents = String::new();
file.read_to_string(&mut contents).unwrap();
    println!("{}", contents);
}
