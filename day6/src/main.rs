use array2d::{Array2D, Error};
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::io::{self, Write};


fn main() {
    
    let mut grid: Array2D<i32> =  Array2D::filled_with(100, 1000, 1000);
    let mut edit_state: i8 = 0;
    let inputstring : String;
    let mut first_set_found: bool = true;

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

    // Iterate through the array to find the lines needed
    for it in &lines_vector {

        let mut first_found_cords: (i32, i32);
        let mut second_found_cords: (i32, i32);

        let cleaned_up_line = it.replacen("turn", "", 1);       

        // Choose the path according to the "on" or "off" token. Flag the path
        if cleaned_up_line.contains("on") { edit_state = 1 }
        else if cleaned_up_line.contains("off") { edit_state = 2 }
        else if cleaned_up_line.contains("toggle") { edit_state = 3 }

        // Find the first set of cordinates and store them in seperate x and y
        let split_words = cleaned_up_line.split(' ');
        
        for split_one in split_words {
            
            if split_one.contains(",") {
                
                let temp_split: Vec<&str> = split_one.split(',').collect();

                println!("X: {} Y: {}", temp_split[0], temp_split[1]);

                if !first_set_found {
                    first_found_cords = (temp_split[0].parse().unwrap(), temp_split[1].parse().unwrap());
                    first_set_found = true;
                }
                else {
                    second_found_cords = (temp_split[0].parse().unwrap(), temp_split[1].parse().unwrap());
                }

                
                
            }

            let mut buffer = String::new();
            io::stdin().read_line(&mut buffer).unwrap();

        }

    }

    

}


