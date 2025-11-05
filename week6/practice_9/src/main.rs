fn main() {
    let a:i32 = 10;
    let b:i32 = 20;

    println!("value of A:{}",a );
    println!("Value of B:{}",b );

    let mut res = a > b; 
    println!("A greater than B: {}", res);

    res = a < b; 
    println!("A lesser than B: {}", res);

    res = a >= b; 
    println!("A greater than or equal to B: {}", res);
    
    res = a <= b; 
    println!("A less than or equal to B: {}", res);

    res = a == b; 
    println!("A equal to B: {}", res);

    res = a != b; 
    println!("A not equal to B: {}", res);

}
