use std::fs;
use std::env;
use std::fs::metadata;

fn file_exists(filepath:&String) -> bool {
    let result = metadata(filepath);
    if result.is_ok() {
        return true;
    } else {
        return false;
    }
}
fn read_file(arg:&String) -> String {
    let contents = fs::read_to_string(arg)
        .expect("Error reading file");
    return contents;
}
fn main() {
    // Vector array for command line args
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);
    for (_, arg) in args.iter().enumerate() {
        println!("{}",arg);
        if file_exists(arg) {
            println!("{}\n", read_file(arg));
        }
    }    
}
