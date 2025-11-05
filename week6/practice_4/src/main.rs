fn main() {
    let fullname = "Chubudem John Umeh";
    let departmet = "Computer scince";
    let uni = "Pann-atlantic University";
  
  let mut school = "School of Science".to_string();
  // push string 
  school.push_str(" and Technology"); 

  println!("My name is {}",fullname );
  // check length 
  println!("The length of my full name {}", fullname.len());
  println!("I am a student of {} Department", departmet);
  println!("{}", school);
  println!("{}", uni); 
}
