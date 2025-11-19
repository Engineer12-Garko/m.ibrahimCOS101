fn main() {
    //initialization of turple with data type
    let datatype_tuple : (&str, f32, u8) = ("Rust", 3.14, 100);
    println!("Tuple comtents: {:?}", datatype_tuple); 

    //initialization of turple without data type
    let no_datatype = ("Rust", 3.14, 100);
    println!("Tuple comtents: {:?}",no_datatype);

    //accessing tuple element at index 0
    println!("Value at index 0 = {}", datatype_tuple.0);

    //accessing tuple element at index 1
    println!("Value at index 1 = {}", datatype_tuple.1);

    //accessing tuple element at index 2
    println!("Value at index 0 = {}", datatype_tuple.2);
}
