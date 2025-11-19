fn main() {
    //an array of numbers 
    let numbers = [1,2,3,4,5];
    println!("Original array = {:?}", numbers);

    //create a slice of 2nd and 3rd ekement 
    let slice1 = &numbers[1..3];
    println!("2nd and 3rd elements sliced = {:?}", slice1);

    //omit the start index
    let slice2 = &numbers[..3];
    //This means the element slice start from 0 and goes up to index 3 (exlusive)
    println!("index 0 to index 3 sliced = {:?}", slice2);

    //omit the end index 
    let slice3 = &numbers[2..];
    //This means the slice start from index 2 and goes up to index 5 (exlusive)
    println!("index 2 to index 5 sliced = {:?}", slice3); 

    //omit the start index and end index 
    //reference the whole array 
    let slice4 = &numbers[..];
    //This meas the slice starts from index 0 and goes up to index 5 (exlusive)
    println!("index 0 to index 5 sliced = {:?}",slice4);
}
