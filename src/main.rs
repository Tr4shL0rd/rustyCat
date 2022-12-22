use std::iter;
use std::fs;
use std::env;
/// This function checks if a file exists at the given filepath.
///
/// # Examples
///
/// ```
/// assert!(file_exists(&"test.txt".to_string()));
/// assert!(!file_exists(&"nonexistent.txt".to_string()));
/// ```
///
/// # Parameters
///
/// - `filepath`: A string slice that represents the path of the file to be checked.
///
/// # Returns
///
/// A boolean value indicating whether the file exists at the given filepath.
fn file_exists(filepath:&String) -> bool {
    let result = fs::metadata(filepath);
    if result.is_ok() {
        return true;
    } else {
        return false;
    }
}

/// This function reads the contents of a file and returns the contents as a string.
///
/// # Examples
///
/// ```
/// let contents = read_file(&"test.txt".to_string());
/// assert_eq!("Hello, world!", contents);
/// ```
///
/// # Parameters
///
/// - `arg`: A string slice that represents the path of the file to be read.
///
/// # Returns
///
/// A string containing the contents of the file.
///
/// # Errors
///
/// If the file cannot be opened or read, the function will panic with an error message.
fn read_file(arg:&String) -> String {
    let contents = fs::read_to_string(arg)
        .expect("Error reading file");
    return contents;
}

/// This function repeats a given string `n` times and returns the result as a new string.
///
/// # Examples
///
/// ```
/// assert_eq!("aaa", repeat_string("a", 3));
/// assert_eq!("HelloHello", repeat_string("Hello", 2));
/// ```
///
/// # Parameters
///
/// - `s`: The string to be repeated.
/// - `n`: The number of times to repeat the string.
///
/// # Returns
///
/// A new string that is the repetition of the input string `n` times.
fn repeat_string(s:&str, n:usize) -> String {
    iter::repeat(s).take(n).collect()
}

fn main() {
    // Vector array for command line args
    let mut args: Vec<String> = env::args().collect();
    // removes the proram element from the array
    args.remove(0);
    // loops over each arg and checks if its a file
    for (_, arg) in args.iter().enumerate() {
        if file_exists(arg) {
            println!("CONTENTS OF {}\n{}",arg, repeat_string("-", arg.len()+12));
            println!("{}\n", read_file(arg));
        }
    }    
}
