use std::fs::File;
use std::io::prelude::*;
use std::ops::Index;
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

        let mut found_double: bool = false;
        
        if ln.contains("ab") || ln.contains("cd") || ln.contains("pq") || ln.contains("xy") {

            println!("Naughty Found: {}", ln);
            continue;

        }

        let mut vowel_count: i32 = 0; 

        for c in ln.chars() {

            if c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u' {
                vowel_count += 1;
            }

        }

        if vowel_count < 3 {
            println!("Naughty Found: {}", ln);
            continue;
        }


        for i in 0..ln.chars().count() {
            if ln.chars().nth(i+1) == ln.chars().nth(i){
                found_double = true;
                break;
            }
        }

        if !found_double {
            continue;
        }

        nice_strs += 1;

    }
    
    println!("Nice Strings Found: {}", nice_strs);

}
