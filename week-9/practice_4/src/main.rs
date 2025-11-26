use std::fs::OpenOptions;
use std::io::Write;

fn main() {
    let mut file = OpenOptions::new().append(true).open("data.txt").expect("Failed to update");
    file.write_all("Hello Mahfu!\n".as_bytes()).expect("write failed");
    file.write_all("This is the appendage to the document.".as_bytes()).expect("write failed");
    println!("File appended successfull!");

}
