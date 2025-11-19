//method to print the get value 
use std::io;
fn value(n:Option<&char>) {
  println!("Element of Vector {:?}",n);
}

fn main() {
   let v = vec!['R', 'U', 'S', 'T', 'A', 'C', 'I', 'A', 'N',];

  let mut input = String::new();
    println!("\nEnter an index valu between 0 - 8");
    io::stdin().read_line(&mut input).expect("not a valid input");

    //index is the non negative value is smaller than the size of the vector
    let index:usize = input.trim().parse().expect("invalid input");

    //getting value at given index value 
    let ch:Option<&char> = v.get(index);
    value(ch);
}
