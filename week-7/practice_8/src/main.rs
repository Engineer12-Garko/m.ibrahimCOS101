fn main() {
    let city_array : [&str; 5] = ["Abuja","Porthacourt", "Maiduguri","Kano","Lagos"];
    println!("Arra is :{:?}", city_array);
    println!("array size is :{}", city_array.len());

    for index in 0..5 {
        println!("city index {} is located in: {}",index,city_array[index]);
    }
}
