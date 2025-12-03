//declare a structure 
struct Employee {
    ceo: String,
    company: String,
    age: u32
}


fn main() {
    //initialising a structure
    let emp1 = Employee {
        company: String::from("Microsoft Corppration"),
        ceo: String::from("Satya Nadella"),
        age: 56
    };

let emp2 = Employee {
    company: String::from("Google Inc."),
        ceo: String::from("Sundai Pichai"),
        age: 51
    }; 
    display(emp1);
    display(emp2);

}

// fetch value of the structure fields using
//operator and print it to the console
fn display(emp:Employee){
    println!("Name is: {} Company is: {} Age is: {}", emp.ceo, emp.company, emp.age);
}