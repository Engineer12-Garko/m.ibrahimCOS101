fn main() {
    let b:(i32, bool, f64) = (110, true, 10.9);
    print(b);

}

// pass the tuple as parameter 
fn print(x:(i32, bool, f64)) {
    println!("inside print method");
    //assign a tuple to distinct variables
    let (age, is_male,cgpa) = x;
    println!("Age is {}, isMale? {}, CGPA is {}",age, is_male,cgpa);
}
