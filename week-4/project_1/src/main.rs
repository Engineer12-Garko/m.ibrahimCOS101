//defining roots of quadratic equatuin 
use std::io;

fn main() {
    let mut a = String::new();
    let mut b = String::new();
    let mut c = String::new();

   println!("Enter the value of 'a':");
   io::stdin().read_line(&mut a).expect("Invalid input");
   let a :f32 = a.trim().parse().expect("invalid input");

   println!("Enter the value of 'b':");
   io::stdin().read_line(&mut b).expect("Invalid input");
   let b :f32 = b.trim().parse().expect("invalid input");
   
   println!("Enter the value of 'c':");
   io::stdin().read_line(&mut c).expect("Invalid input");
   let c :f32 = c.trim().parse().expect("invalid input");
   //finding discrinminat of tge equation 

   let disc: f32 = b * b - (4.0 * a * c); 
   if disc > 0.0 {
    println!("The roots are real and not equal"); 
   }
    else if disc == 0.0 {
      println!("The roots are real and equal");
   }
    else if disc < 0.0 {
    println!("The root are complex"); 
   } 
   else {
    println!("out of scope!")
   }

}
