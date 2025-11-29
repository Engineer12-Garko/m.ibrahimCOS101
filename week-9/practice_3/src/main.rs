use std::fs::File;
use std::fs;

fn main(){
   fs::remove_file("file.txt").expect("Failed to remove file");
   println!("File removed, next!");
}