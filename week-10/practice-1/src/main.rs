fn main() {
    

    let  v = vec![101, 250, 330, 400];
    // vector v owns the object in heap

    //only a single variable owns heap at any given time
    let v2 = v.clone();
    // here two variable owns heap value,
    //two pointers to the to the same conten in rust

    //Rust is very smart in terms of memory access, so it detects a frace cindition
    //as a two variables point to same heap

}
