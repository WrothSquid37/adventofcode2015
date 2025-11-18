use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    
    let inputstring : String;
    let mut count : i32 = 0;
    let mut iterate_count : i32 = 1;
    let mut saved_first_occur : i32 = 0;
    let show_steps : bool = true;
    let mut found_first : bool = false;

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


    for c in inputstring.chars() {
        

        if c == ')' {         
            count -= 1;
        }
        else if c == '(' {           
            count += 1;
        }
        else {
            panic!("Unexpected Character. Stopping...");
        }     

        if count < 0 && found_first == false { 
            saved_first_occur = iterate_count;
            found_first = true;
        }

        iterate_count += 1;

    }
    
    println!("Floor arrived on: {}", count);
    println!("First occurance of floor: {}", saved_first_occur);
    
}