use array2d::{Array2D, Error};
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    
    let mut grid: Array2D<i32> =  Array2D::filled_with(100, 1000, 1000);
    
    let inputstring : String;

    // Create a path to the desired file
    let path = Path::new("input.txt");
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    // This checks when opening the file and throws an error if not succeded 
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => inputstring = s,
    }

    let lines_vector: Vec<String> = inputstring.lines().map(String::from).collect();

    // Look for tokens showing instructions
    for it in &lines_vector {
        if check_for_tokens(it, "turn off") {
            println!("Found turn off");
        }
    }

    println!("{:?}",lines_vector)

}

fn check_for_tokens(search_string: String, test_string: String) -> bool {

    return search_string.contains(&test_string)
        
}
