use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::collections::HashSet;

fn main() {
    
    let inputstring : String;
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut rx: i32 = 0;
    let mut ry: i32 = 0;
    let mut uni_cords: HashSet<(i32, i32)>  = Default::default();
    let mut current_count: i32 = 0;
    

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
        if current_count % 2 == 0 { 
            match c {
                '>' => x += 1,
                '<' => x -= 1,
                '^' => y += 1,
                'v' => y -= 1,
                _ => panic!("Error Parsing Data"),
            }
            uni_cords.insert((x, y));
        }
        else {
            match c {
            '>' => rx += 1,
            '<' => rx -= 1,
            '^' => ry += 1,
            'v' => ry -= 1,
            _ => panic!("Error Parsing Data"),
            }
            uni_cords.insert((rx, ry));
        }
        current_count += 1;
        
    }

    println!("{}", uni_cords.len());

}