use std::fs;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    //let mut contents: String 
    let mut contents: String = String::from("");
    for (_, arg) in args.iter().enumerate() {
        //println!("args {}: {}", i , arg)
        contents = fs::read_to_string(arg)
            .expect("Error reading file");
    }
    println!("file contains: {}", contents);

}
