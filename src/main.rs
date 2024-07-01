use std::fs::File;
use std::io::Read;

mod interpreter;
mod machine;

fn main() {
    let mut file = File::open("./main.obj").expect("./main.obj not found");

    // Create a string to hold the file contents
    let mut object_code = String::new();
    // Read the file into the string
    file.read_to_string(&mut object_code).unwrap();

    let vec: Vec<&str> = object_code.split_whitespace().collect();
}
