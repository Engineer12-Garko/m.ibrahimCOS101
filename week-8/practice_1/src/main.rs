fn main() {
   //using Vec::new()
   let vector : Vec<i64> = Vec::new();

   //print the size of the vector 
   println!("\nThe length of Vec::new is : {}", vector.len());
   
   //using macro 
   let vector_macro = vec!["Musa", "Bello", "Farida","Ibrahim"];

   //printing the size of the vec! 
   println!("\nThe lwngth of vec macro is : {}",vector_macro.len());
}
