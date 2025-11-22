use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
fn main() {
    
    let inputstring : String;
    let mut nice_strs: i16 = 0;

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
  
    let split_string = inputstring.split("\n");
    
    for ln in split_string {

        let mut found_doubles: i8 = 0;
        let mut claimed_character_indexes = vec![];
        for i in 0..ln.chars().count() {
            if ln.chars().nth(i+1) == ln.chars().nth(i) && !claimed_character_indexes.contains(&i){
                found_doubles += 1;
                claimed_character_indexes.push(i);
                claimed_character_indexes.push(i+1);
            }
        }

        if (found_doubles < 2) {
            println!("Found Naughty: {}", ln);
            continue;
        }
        
        println!("Found Nice: {}", ln)

    }

    

}
