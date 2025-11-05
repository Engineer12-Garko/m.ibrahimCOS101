fn main() {
    let name = "Aisha Lawal";
    let uni = "Pan-atlantic University";
    let addr = "Km 52 Lekki-epe Expressway, Ibeju-Lekki, Lagos";
    println!("Name: {}",name );
    print!("University: {}, \nAddress: {}", uni, addr);


    let department: &'static str = "computer scince"; 
    let schoool: &'static str = "School of science and Technology"; 
    println!("Department: {}, \nSchool: {}",department, schoool ); 
}
